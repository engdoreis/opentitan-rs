// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use crate::earlgrey_es::*;
use crate::prelude::*;
use core::fmt::Write;
use embedded_hal::digital::OutputPin;
use hal::pin::{self, GpioExt};

const TIMER_TICK_HZ: u64 = 1_000_000;
pub type Duration = fugit::Duration<u64, 1, TIMER_TICK_HZ>;
pub type Instant = fugit::Instant<u64, 1, TIMER_TICK_HZ>;

pub type Led2 = hal::static_pin::Pin0<pin::OutputState>;
pub type Led4 = hal::static_pin::Pin1<pin::OutputState>;
pub type LcdRst = hal::static_pin::Pin10<pin::OutputState>;
pub type LcdDc = hal::static_pin::Pin11<pin::OutputState>;
pub type LcdBl = hal::static_pin::Pin12<pin::OutputState>;

pub struct Voyager {
    pub led_status: Led2,
    pub led_boot_ok: Led4,
    pub lcd_bl: LcdBl,
    pub uart0: hal::Uart,
    pub timer: hal::RvTimer<{ Silicon::PERIPHERAL_CLOCK_HZ as u64 }>,
    pub plic: hal::Plic,
    pub joystick: Joystick,
    pub display: st7735_lcd::ST7735<hal::spi_host::SpiHost, LcdDc, LcdRst>,
}

impl Default for Voyager {
    fn default() -> Self {
        Self::new()
    }
}

impl Voyager {
    pub fn new() -> Self {
        let peri = zermio::earlgrey::Peripherals::take().unwrap();
        let mut mux = hal::pinmux::Pinmux::new(peri.pinmux);

        let mut uart0 = hal::Uart::new(
            peri.uart0,
            crate::earlgrey_es::Silicon::PERIPHERAL_CLOCK_HZ,
            crate::earlgrey_es::Silicon::BAUD,
        );
        let _ = writeln!(uart0, "Initializing Voyager");

        let timer = hal::RvTimer::new(peri.timer);
        let plic = hal::Plic::new(peri.plic);

        let pins = peri.gpio.split();

        mux.select_output(
            <EarlgreyEs as PadOutput>::IOA4 as usize,
            <EarlgreyEs as PeripheralOutput>::GPIO0 as usize,
        );
        let led_status = pins.pin0.into_output();

        mux.select_output(
            <EarlgreyEs as PadOutput>::IOA5 as usize,
            <EarlgreyEs as PeripheralOutput>::GPIO1 as usize,
        );
        let led_boot_ok = pins.pin1.into_output();

        mux.select_output(
            <EarlgreyEs as PadOutput>::IOC7 as usize,
            <EarlgreyEs as PeripheralOutput>::GPIO10 as usize,
        );
        let lcd_rst = pins.pin10.into_output();

        mux.select_output(
            <EarlgreyEs as PadOutput>::IOC9 as usize,
            <EarlgreyEs as PeripheralOutput>::GPIO11 as usize,
        );
        let lcd_dc = pins.pin11.into_output();

        mux.select_output(
            <EarlgreyEs as PadOutput>::IOB0 as usize,
            <EarlgreyEs as PeripheralOutput>::GPIO12 as usize,
        );
        let mut lcd_bl = pins.pin12.into_output();

        // Joystick gpios
        mux.select_input(
            <EarlgreyEs as PadInput>::IOC12 as usize,
            <EarlgreyEs as PeripheralInput>::GPIO5 as usize,
        );

        mux.select_input(
            <EarlgreyEs as PadInput>::IOC10 as usize,
            <EarlgreyEs as PeripheralInput>::GPIO6 as usize,
        );

        mux.select_input(
            <EarlgreyEs as PadInput>::IOR10 as usize,
            <EarlgreyEs as PeripheralInput>::GPIO7 as usize,
        );

        mux.select_input(
            <EarlgreyEs as PadInput>::IOC11 as usize,
            <EarlgreyEs as PeripheralInput>::GPIO8 as usize,
        );

        mux.select_input(
            <EarlgreyEs as PadInput>::IOR11 as usize,
            <EarlgreyEs as PeripheralInput>::GPIO9 as usize,
        );
        let joystick = Joystick {
            up: Button { pin: pins.pin5 },
            down: Button { pin: pins.pin6 },
            left: Button { pin: pins.pin7 },
            right: Button { pin: pins.pin8 },
            ok: Button { pin: pins.pin9 },
        };

        let config = hal::spi_host::Config {
            speed: 24_000_000,
            clock_freq_hz: Silicon::PERIPHERAL_CLOCK_HZ * 2,
            chip_select: hal::spi_host::ChipSelectTiming {
                idle: 1,
                trail: 1,
                lead: 1,
            },
            cpha: false,
            cpol: false,
            full_cycle: false,
            tx_watermark: 5,
            rx_watermark: 5,
        };

        mux.select_output(
            <EarlgreyEs as PadOutput>::IOC6 as usize,
            <EarlgreyEs as PeripheralOutput>::SPIHOST1CSB as usize,
        );
        mux.select_output(
            <EarlgreyEs as PadOutput>::IOB2 as usize,
            <EarlgreyEs as PeripheralOutput>::SPIHOST1SCK as usize,
        );
        mux.select_output(
            <EarlgreyEs as PadOutput>::IOB1 as usize,
            <EarlgreyEs as PeripheralOutput>::SPIHOST1SD0 as usize,
        );

        let mut spi_lcd = hal::SpiHost::new(peri.spi_host1);
        spi_lcd
            .configure(config)
            .unwrap()
            .enable(true)
            .enable_output(true);

        let _ = writeln!(uart0, "starting lcd");
        let mut delay = riscv::delay::McycleDelay::new(crate::earlgrey_es::Silicon::CPU_CLOCK_HZ);
        let mut display = st7735_lcd::ST7735::new(spi_lcd, lcd_dc, lcd_rst, true, false, 160, 128);

        display.init(&mut delay).unwrap();
        display
            .set_orientation(&st7735_lcd::Orientation::LandscapeSwapped)
            .unwrap();
        lcd_bl.set_high().unwrap();

        Self {
            led_status,
            led_boot_ok,
            lcd_bl,
            display,
            uart0,
            timer,
            plic,
            joystick,
        }
    }

    pub fn enable_joystick_irq(&mut self) {
        self.plic.interrupt_set_threshold(1);

        for id in [
            PlicIrqId::Gpio5,
            PlicIrqId::Gpio6,
            PlicIrqId::Gpio7,
            PlicIrqId::Gpio8,
            PlicIrqId::Gpio9,
            PlicIrqId::RvTimerTimerExpiredHart0Timer0,
        ] {
            self.plic.interrupt_set_enable(id, true);
            self.plic.interrupt_set_priority(id, 2);
        }

        let modes = [
            hal::gpio::InterruptMode::EdgeRising,
            hal::gpio::InterruptMode::EdgeFalling,
        ];
        self.joystick
            .up
            .pin
            .interrupt_set_mode(true, &modes)
            .interrupt_set_enable(true);
        self.joystick
            .down
            .pin
            .interrupt_set_mode(true, &modes)
            .interrupt_set_enable(true);

        self.joystick
            .left
            .pin
            .interrupt_set_mode(true, &modes)
            .interrupt_set_enable(true);

        self.joystick
            .right
            .pin
            .interrupt_set_mode(true, &modes)
            .interrupt_set_enable(true);

        self.joystick
            .ok
            .pin
            .interrupt_set_mode(true, &modes)
            .interrupt_set_enable(true);

        unsafe {
            riscv::interrupt::enable_interrupt(riscv::interrupt::Interrupt::MachineExternal);
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Ok,
}

type BtnUp = Button<hal::static_pin::Pin5<pin::InputState>>;
type BtnDown = Button<hal::static_pin::Pin6<pin::InputState>>;
type BtnLeft = Button<hal::static_pin::Pin7<pin::InputState>>;
type BtnRight = Button<hal::static_pin::Pin8<pin::InputState>>;
type BtnOk = Button<hal::static_pin::Pin9<pin::InputState>>;

pub struct Joystick {
    pub up: BtnUp,
    pub down: BtnDown,
    pub left: BtnLeft,
    pub right: BtnRight,
    pub ok: BtnOk,
}

impl Joystick {
    pub fn scan(&mut self) -> Option<Direction> {
        if self.up.pressed() {
            return Some(Direction::Up);
        }
        if self.down.pressed() {
            return Some(Direction::Down);
        }
        if self.left.pressed() {
            return Some(Direction::Left);
        }
        if self.right.pressed() {
            return Some(Direction::Right);
        }
        if self.ok.pressed() {
            return Some(Direction::Ok);
        }
        None
    }
}

pub struct Button<PIN> {
    pin: PIN,
}

impl<PIN> Button<PIN>
where
    PIN: embedded_hal::digital::InputPin,
{
    pub fn pressed(&mut self) -> bool {
        self.pin.is_low().unwrap()
    }
}

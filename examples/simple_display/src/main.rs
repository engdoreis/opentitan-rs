// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

#[path = "../../lib/benchmark.rs"]
mod benchmark;
mod visuals;
use crate::benchmark::Benchmark;

use embedtrait::timer::Timer;
use fugit::ExtU32;

use bsp::earlgrey_es::*;
use bsp::prelude::*;
use bsp::voyager::Direction;

use core::fmt::Write;
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE},
    mono_font::{MonoTextStyle, ascii::FONT_8X13},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use riscv::delay::McycleDelay;
use riscv_rt::entry;

use crate::visuals::menu::{MenuItem, MenuTheme};
use embedded_menu::{
    Menu, MenuStyle, interaction, selection_indicator::style::rectangle::Rectangle,
};

#[entry]
fn main() -> ! {
    let mut delay = McycleDelay::new(Silicon::CPU_CLOCK_HZ);
    let mut board = bsp::Voyager::new();
    let serial = core::cell::RefCell::new(board.uart0);
    let mut bm = Benchmark::new(&serial, Silicon::CPU_CLOCK_HZ as u64);

    let _ = writeln!(serial.borrow_mut(), "Board initialized");

    board.display.set_offset(0, 0);
    bm.start();
    board.display.clear(Rgb565::WHITE).unwrap();
    bm.finish("Display cleared");

    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../../resources/opentitan.rgb565"), 160);
    let image: Image<_> = Image::new(&image_raw, Point::new(0, 0));

    bm.start();
    image.draw(&mut board.display).unwrap();
    bm.finish("Image rendered");

    let style = MonoTextStyle::new(&FONT_8X13, Rgb565::BLUE);
    Text::new("Successful boot", Point::new(15, 115), style)
        .draw(&mut board.display)
        .unwrap();
    delay.delay_ms(1000);
    board.display.clear(Rgb565::WHITE).unwrap();

    let mut menu = Menu::with_style(
        "Main Menu",
        MenuStyle::new(MenuTheme)
            .with_selection_indicator(Rectangle)
            .with_input_adapter(interaction::programmed::Programmed {}),
    )
    .add_item("1.", MenuItem::BootOk, |_| MenuItem::BootOk)
    .add_item("2.", MenuItem::BootNOk, |_| MenuItem::BootNOk)
    .build();

    menu.draw(&mut board.display).unwrap();

    let mut led_timer = board.timer.new_timer();
    led_timer.start(500u32.millis());
    let mut led_on = true;

    let mut btn_timer = board.timer.new_timer();
    btn_timer.start(100u32.millis());

    loop {
        if btn_timer.is_ready() {
            btn_timer.reload();
            match board.joystick.scan() {
                None => continue,
                Some(dir) => {
                    match dir {
                        Direction::Down => {
                            menu.interact(interaction::Interaction::Navigation(
                                interaction::Navigation::Next,
                            ));
                        }
                        Direction::Up => {
                            menu.interact(interaction::Interaction::Navigation(
                                interaction::Navigation::Previous,
                            ));
                        }
                        Direction::Ok => {
                            let selected = menu.selected_value();
                            match selected {
                                MenuItem::BootOk => board.led_boot_ok.set_high().unwrap(),
                                MenuItem::BootNOk => board.led_boot_ok.set_low().unwrap(),
                            }
                        }
                        _ => (),
                    };

                    let _ = writeln!(serial.borrow_mut(), "{:?}", dir);
                    delay.delay_ms(300);
                    menu.update(&board.display);
                }
            }
            menu.draw(&mut board.display).unwrap();
        }

        if led_timer.is_ready() {
            if led_on {
                board.led_status.set_low().unwrap();
            } else {
                board.led_status.set_high().unwrap();
            }
            led_on = !led_on;
            led_timer.reload();
        }
    }
}

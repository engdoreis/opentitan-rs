// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

mod channel;
mod joystick;
mod my_executor;
mod screen;
mod timer;

use crate::my_executor as executor;

use bsp::earlgrey_es::*;
use bsp::prelude::*;

use bsp::voyager::Direction;
use channel::{Channel, Receiver, Sender};
use core::{fmt::Write, pin::pin};
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE},
    mono_font::{MonoTextStyle, ascii::FONT_8X13},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut delay = McycleDelay::new(Silicon::CPU_CLOCK_HZ);
    let mut board = bsp::Voyager::new();
    board.enable_joystick_irq();
    let serial = core::cell::RefCell::new(board.uart0);

    let _ = writeln!(serial.borrow_mut(), "Board initialized");

    board.display.set_offset(0, 0);
    board.display.clear(Rgb565::WHITE).unwrap();

    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../../resources/opentitan.rgb565"), 160);
    let image: Image<_> = Image::new(&image_raw, Point::new(0, 0));

    image.draw(&mut board.display).unwrap();

    let style = MonoTextStyle::new(&FONT_8X13, Rgb565::BLUE);
    Text::new("Successful boot", Point::new(15, 115), style)
        .draw(&mut board.display)
        .unwrap();
    delay.delay_ms(1000);
    board.display.clear(Rgb565::WHITE).unwrap();

    let channel: Channel<Direction> = channel::Channel::new();
    let action_channel: Channel<screen::LedAction> = channel::Channel::new();

    timer::AsyncTimer::init(board.timer.into_raw());

    let btn_r_task = pin!(btn_task(board.joystick, channel.get_sender()));

    let led_r_task = pin!(led_task(board.led_status, action_channel.get_receiver()));

    let screen_r_task = pin!(screen_task(
        board.display,
        channel.get_receiver(),
        action_channel.get_sender(),
    ));

    executor::run(&mut [led_r_task, btn_r_task, screen_r_task]);
}

async fn btn_task(js: bsp::voyager::Joystick, sender: channel::Sender<'_, Direction>) {
    let debounce = bsp::voyager::Duration::from_millis(50);
    let mut btn = joystick::Button::new(js);
    loop {
        let dir = btn.wait_for_event(joystick::Event::Press).await;
        timer::delay(debounce).await;
        _ = btn.wait_for_event(joystick::Event::Release(dir)).await;
        sender.send(dir);
    }
}

use embedded_hal::digital::OutputPin;
use futures::FutureExt;
use screen::{LedAction, Screen};

async fn led_task<P: OutputPin>(mut led: P, mut receiver: Receiver<'_, LedAction>) {
    enum State {
        Blinking(bool),
        On,
        Off,
    }
    let mut state = State::Blinking(true);
    let led_blink_period = bsp::voyager::Duration::from_millis(400);
    loop {
        match state {
            State::On => {
                led.set_high().unwrap();
            }
            State::Off => {
                led.set_low().unwrap();
            }
            State::Blinking(on) => {
                if on {
                    led.set_high().unwrap();
                } else {
                    led.set_low().unwrap();
                }
                state = State::Blinking(!on);
            }
        }

        futures::select_biased! {
            signal = receiver.receive().fuse() => {
                state = match signal {
                    LedAction::On => State::On,
                    LedAction::Off => State::Off,
                    LedAction::Blink => State::Blinking(true),
                };
            }
            _ = timer::delay(led_blink_period).fuse() => {
            }
        }
    }
}

async fn screen_task<D: DrawTarget<Color = Rgb565>>(
    display: D,
    mut receiver: Receiver<'_, Direction>,
    sender: Sender<'_, LedAction>,
) {
    let mut screen = Screen::new(display, sender);
    loop {
        let direction = receiver.receive().await;
        screen.update(direction)
    }
}

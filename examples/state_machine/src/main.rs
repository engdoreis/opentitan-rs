// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

#[path = "../../lib/channel.rs"]
mod channel;
mod tasks;

use crate::tasks::Task;

use embedtrait::timer::Timer;
use fugit::ExtU32;

use bsp::earlgrey_es::*;
use bsp::prelude::*;

use core::fmt::Write;
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

    let channel: channel::Channel<bsp::voyager::Direction> = channel::Channel::new();
    let action_channel: channel::Channel<tasks::screen::LedAction> = channel::Channel::new();

    let mut btn_timer = board.timer.new_timer();
    btn_timer.start(40u32.millis());
    let mut btn_task =
        tasks::joystick::Button::new(btn_timer, board.joystick, channel.get_sender());

    let mut led_timer = board.timer.new_timer();
    led_timer.start(400u32.millis());
    let mut led_task =
        tasks::led::Led::new(led_timer, board.led_status, action_channel.get_receiver());
    let mut screen_task = tasks::screen::Screen::new(
        board.display,
        channel.get_receiver(),
        action_channel.get_sender(),
    );

    loop {
        btn_task.poll();
        led_task.poll();
        screen_task.poll();
    }
}

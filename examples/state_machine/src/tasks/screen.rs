// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::Task;
use crate::channel::{Receiver, Sender};
use bsp::voyager::Direction;

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

use embedded_menu::interaction::programmed::Programmed;
use embedded_menu::{
    Menu, MenuStyle, interaction,
    selection_indicator::{StaticPosition, style::rectangle::Rectangle},
};
use embedded_menu::{SelectValue, theme::Theme};

// Define an enum for the actions your menu will return
#[derive(Debug, Copy, Clone, PartialEq, SelectValue)]
pub enum LedAction {
    On,
    Off,
    Blink,
}

#[derive(Clone, Copy)]
pub struct MenuTheme;

impl Theme for MenuTheme {
    type Color = Rgb565;

    fn text_color(&self) -> Self::Color {
        Rgb565::RED
    }

    fn selected_text_color(&self) -> Self::Color {
        Rgb565::BLUE
    }

    fn selection_color(&self) -> Self::Color {
        Rgb565::new(51, 255, 51)
    }
}

type Menu_<'a> = Menu<
    &'a str,
    Programmed,
    embedded_layout::object_chain::Link<
        embedded_menu::items::MenuItem<&'a str, LedAction, LedAction, true>,
        embedded_layout::object_chain::Link<
            embedded_menu::items::MenuItem<&'a str, LedAction, LedAction, true>,
            embedded_layout::object_chain::Chain<
                embedded_menu::items::MenuItem<&'a str, LedAction, LedAction, true>,
            >,
        >,
    >,
    LedAction,
    StaticPosition,
    embedded_menu::selection_indicator::style::rectangle::Rectangle,
    MenuTheme,
>;

pub struct Screen<'a, D> {
    display: D,
    receiver: Receiver<'a, Direction>,
    sender: Sender<'a, LedAction>,
    menu: Menu_<'a>,
}

impl<'a, D: DrawTarget<Color = Rgb565>> Screen<'a, D> {
    pub fn new(
        mut display: D,
        receiver: Receiver<'a, Direction>,
        sender: Sender<'a, LedAction>,
    ) -> Self {
        let menu = Menu::with_style(
            "Main Menu",
            MenuStyle::new(MenuTheme)
                .with_selection_indicator(Rectangle)
                .with_input_adapter(interaction::programmed::Programmed {}),
        )
        .add_item("1. Led", LedAction::On, |_| LedAction::On)
        .add_item("2. Led", LedAction::Off, |_| LedAction::Off)
        .add_item("3. Led", LedAction::Blink, |_| LedAction::Blink)
        .build();

        let _ = menu.draw(&mut display);
        Self {
            display,
            receiver,
            sender,
            menu,
        }
    }
}

impl<'a, D: DrawTarget<Color = Rgb565>> Task for Screen<'a, D> {
    fn poll(&mut self) {
        let Some(direction) = self.receiver.receive() else {
            return;
        };

        match direction {
            Direction::Down => {
                self.menu.interact(interaction::Interaction::Navigation(
                    interaction::Navigation::Next,
                ));
            }
            Direction::Up => {
                self.menu.interact(interaction::Interaction::Navigation(
                    interaction::Navigation::Previous,
                ));
            }
            Direction::Ok => {
                let selected = self.menu.selected_value();
                self.sender.send(selected);
            }
            Direction::Left => {}
            Direction::Right => {}
        }
        self.menu.update(&self.display);
        let _ = self.menu.draw(&mut self.display);
    }
}

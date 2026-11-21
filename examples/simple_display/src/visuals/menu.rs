use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

use embedded_menu::{SelectValue, theme::Theme};

// Define an enum for the actions your menu will return
#[derive(Debug, Copy, Clone, PartialEq, SelectValue)]
pub enum MenuItem {
    BootOk,
    BootNOk,
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

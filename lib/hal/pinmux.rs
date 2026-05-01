// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub struct Pinmux {
    regs: zermio::pinmux::Pinmux,
}

impl Pinmux {
    #[inline]
    pub fn new(regs: zermio::pinmux::Pinmux) -> Self {
        Self { regs }
    }

    #[inline]
    pub fn select_input(&mut self, io_pad: usize, peripheral_pad: usize) -> &mut Self {
        self.regs.mio_periph_insel[peripheral_pad]
            .insel()
            .write(io_pad as u32)
            .commit();
        self
    }

    #[inline]
    pub fn select_output(&mut self, io_pad: usize, peripheral_pad: usize) -> &mut Self {
        self.regs.mio_outsel[io_pad]
            .out()
            .write(peripheral_pad as u32)
            .commit();
        self
    }
}

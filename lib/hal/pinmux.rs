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
    pub fn select_input(
        &mut self,
        io_pad: impl PadInput,
        peripheral_pad: impl PeripheralInput,
    ) -> &mut Self {
        self.regs.mio_periph_insel[peripheral_pad.number()]
            .insel()
            .write(io_pad.number() as u32)
            .commit();
        self
    }

    #[inline]
    pub fn select_output(
        &mut self,
        io_pad: impl PadOutput,
        peripheral_pad: impl PeripheralOutput,
    ) -> &mut Self {
        self.regs.mio_outsel[io_pad.number()]
            .out()
            .write(peripheral_pad.number() as u32)
            .commit();
        self
    }
}

pub trait PeripheralOutput: embedtrait::PinMuxNumber {}
pub trait PeripheralInput: embedtrait::PinMuxNumber {}
pub trait PadOutput: embedtrait::PinMuxNumber {}
pub trait PadInput: embedtrait::PinMuxNumber {}

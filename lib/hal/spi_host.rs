// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::{HalError, HalResult};

pub struct SpiHost {
    regs: zermio::spi_host::SpiHost,
}

impl SpiHost {
    #[inline]
    pub fn new(regs: zermio::spi_host::SpiHost) -> Self {
        Self { regs }
    }

    pub fn configure(&mut self, config: SpiHostConfig) -> HalResult<&mut Self> {
        config.validate(&mut self.regs)?;
        self.reset();

        self.regs.configopts.write(|opt| {
            opt.clkdiv().write(config.clk_divider());
            opt.csnidle().write(config.chip_select.idle);
            opt.csnlead().write(config.chip_select.lead);
            opt.csntrail().write(config.chip_select.trail);
            opt.fullcyc().assign(config.full_cycle);
            opt.cpha().assign(config.cpha);
            opt.cpol().assign(config.cpol);
        });
        self.regs.control.update(|ctrl| {
            ctrl.tx_watermark().write(config.tx_watermark);
            ctrl.rx_watermark().write(config.rx_watermark);
        });

        Ok(self)
    }

    pub fn enable(&mut self, enable: bool) -> &mut Self {
        self.regs.control.update(|ctrl| {
            ctrl.spien().assign(enable);
        });
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.regs.control.update(|ctrl| {
            ctrl.sw_rst().set();
        });

        while self.regs.status.fetch().active().is_set() {}

        while self.regs.status.fetch().txqd().get() + self.regs.status.rxqd().get() > 0 {}

        self.regs.control.update(|ctrl| {
            ctrl.sw_rst().reset();
        });
        self
    }

    pub fn enable_output(&mut self, enable: bool) -> &mut Self {
        self.regs.control.update(|ctrl| {
            ctrl.output_en().assign(enable);
        });
        self
    }

    fn is_ready(&mut self) -> bool {
        self.regs.status.fetch().ready().is_set()
    }

    fn wait_ready(&mut self) {
        while !self.is_ready() {}
    }

    fn wait_tx_fifo_free(&mut self) {
        while self.regs.status.fetch().txqd().get() == self.regs.status.txqd().max() {}
    }

    fn wait_rx_fifo(&mut self) {
        while self.regs.status.fetch().rxempty().is_set() {}
    }

    fn write_fifo(&mut self, data: &[u8]) {
        for chunk in data.chunks(4) {
            self.wait_tx_fifo_free();
            self.regs.txdata.write_fifo(chunk);
        }
    }

    fn read_fifo(&mut self, data: &mut [u8]) {
        for chunk in data.chunks_mut(4) {
            self.wait_rx_fifo();
            chunk.copy_from_slice(
                &self.regs.rxdata.fetch().val().get().to_le_bytes()[0..chunk.len()],
            );
        }
    }

    fn issue_command(&mut self, dir: u32, len: u32, speed: u32, last: bool) {
        self.regs.command.write(|cmd| {
            cmd.direction().write(dir);
            cmd.speed().write(speed);
            cmd.csaat().assign(!last);
            cmd.len().write(len - 1);
        });
    }
}

use embedtrait::spi_host::{Data, Operation, SpiBus, Width};

impl SpiBus for SpiHost {
    type Error = HalError;

    fn transfer(&mut self, operations: &mut [Operation<'_, '_>]) -> HalResult<()> {
        let last = operations.len() - 1;
        for (i, op) in operations.iter_mut().enumerate() {
            self.wait_ready();
            let (dir, len) = match &op.data {
                Data::Dummy(len) => (0, *len as usize),
                Data::Read(data) => (1, data.len()),
                Data::Write(data) => (2, data.len()),
                Data::Both { w, .. } => (3, w.len()),
            };
            let speed = match op.width {
                Width::Single => 0,
                Width::Dual => 1,
                Width::Quad => 2,
            };
            self.issue_command(dir, len as u32, speed, i == last);

            match &mut op.data {
                Data::Write(data) => self.write_fifo(data),
                Data::Read(data) => self.read_fifo(data),
                Data::Both { w, r } => {
                    self.write_fifo(w);
                    self.read_fifo(r)
                }
                Data::Dummy(_) => (),
            };
        }
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiError {
    BusBusy,
    BadCSConfig,
}

/// Chip-select timing configuration in SCK half-cycles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChipSelectTiming {
    pub idle: u32,
    pub trail: u32,
    pub lead: u32,
}

/// Configuration for the SPI host peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SpiHostConfig {
    pub speed: u32,
    pub clock_freq_hz: u32,
    pub chip_select: ChipSelectTiming,
    /// Full-cycle sampling mode.
    pub full_cycle: bool,
    /// SPI clock phase.
    pub cpha: bool,
    /// SPI clock polarity.
    pub cpol: bool,
    /// TX FIFO watermark: interrupt fires when depth drops below this many words (32-bit each).
    pub tx_watermark: u32,
    /// RX FIFO watermark: interrupt fires when depth drops below this many words (32-bit each).
    pub rx_watermark: u32,
}

impl SpiHostConfig {
    pub fn validate(&self, regs: &mut zermio::spi_host::SpiHost) -> HalResult<()> {
        if self.clock_freq_hz + self.speed == 0
            || !regs.configopts.clkdiv().in_range(self.clk_divider())
        {
            return Err(HalError::BadClock);
        }
        if !(regs.configopts.csnidle().in_range(self.chip_select.idle)
            && regs.configopts.csnlead().in_range(self.chip_select.lead)
            && regs.configopts.csntrail().in_range(self.chip_select.trail))
        {
            return Err(SpiError::BadCSConfig.into());
        }

        Ok(())
    }

    pub fn clk_divider(&self) -> u32 {
        (self.clock_freq_hz / self.speed) / 2u32 - 1u32
    }
}

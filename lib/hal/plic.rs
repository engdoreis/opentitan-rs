// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub struct Plic {
    regs: zermio::rv_plic::RvPlic,
}

impl Plic {
    #[inline]
    pub fn new(regs: zermio::rv_plic::RvPlic) -> Self {
        Self { regs }
    }

    #[inline]
    pub fn interrupt_set_enable<I>(&mut self, id: I, en: bool) -> &mut Self
    where
        I: riscv::InterruptNumber,
    {
        let id = id.number();
        let reg_index = id / 32;
        let field_index = id % 32;
        let mask = 0x01 << field_index;

        if reg_index < self.regs.ie_0.len() {
            self.regs.ie_0[reg_index].update(|reg| {
                let field = reg.all_fields();
                let masked = field.get() & !mask;
                field.write(masked | ((en as u32) << field_index));
            })
        } else {
            self.regs.ie_1.update(|reg| {
                let field = reg.all_fields();
                let masked = field.get() & !mask;
                field.write(masked | ((en as u32) << field_index));
            })
        }
        self
    }

    #[inline]
    pub fn is_interrupt_pending<I>(&mut self, id: I) -> bool
    where
        I: riscv::InterruptNumber,
    {
        let id = id.number();
        let reg_index = id / 32;
        let field_index = id % 32;
        let mask = 0x01 << field_index;

        let masked = if reg_index < self.regs.ip_0.len() {
            self.regs.ip_0[reg_index].fetch().all_fields().get() & mask
        } else {
            self.regs.ip_1.fetch().all_fields().get() & mask
        };
        masked > 0
    }

    #[inline]
    pub fn interrupt_set_priority<I>(&mut self, id: I, priority: usize) -> &mut Self
    where
        I: riscv::InterruptNumber,
    {
        let id = id.number();
        self.regs.prio[id].prio().write(priority as u32).commit();
        self
    }

    #[inline]
    pub fn interrupt_get_priority<I>(&mut self, id: I) -> usize
    where
        I: riscv::InterruptNumber,
    {
        let id = id.number();
        self.regs.prio[id].fetch().prio().get() as usize
    }

    #[inline]
    pub fn interrupt_set_threshold(&mut self, threshold: usize) -> &mut Self {
        self.regs
            .threshold0
            .threshold0()
            .write(threshold as u32)
            .commit();
        self
    }

    #[inline]
    pub fn interrupt_claim(&mut self) -> usize {
        self.regs.cc0.fetch().cc0().get() as usize
    }

    #[inline]
    pub fn interrupt_complete<I>(&mut self, id: I) -> &mut Self
    where
        I: riscv::InterruptNumber,
    {
        let id = id.number();
        self.regs.cc0.cc0().write(id as u32).commit();
        self
    }
}

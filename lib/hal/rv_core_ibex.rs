// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub struct RvCoreIbex {
    regs: zermio::rv_core_ibex::RvCoreIbex,
}

impl RvCoreIbex {
    #[inline]
    pub fn new(regs: zermio::rv_core_ibex::RvCoreIbex) -> Self {
        Self { regs }
    }

    pub fn test_status(&mut self, status: u32) {
        self.regs.dv_sim_window[0].val().write(status).commit();
    }
}

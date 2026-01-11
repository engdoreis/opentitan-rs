pub struct RvCoreIbex {
    regs: zermio::rv_core_ibex::RvCoreIbex,
}

impl RvCoreIbex {
    #[inline]
    pub fn new(regs: zermio::rv_core_ibex::RvCoreIbex) -> Self {
        Self { regs }
    }

    pub fn test_status(&mut self, status: u32) {
        self.regs.dv_sim_window0.val().write(status).commit();
    }
}

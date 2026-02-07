//! Indicates the status of the software running on the CPU, from a testing
//! perspective.
//!
//! It is useful to track the current status of the test SW executing on the CPU
//! as a signal that helps aid in debug.
//!
//! The values set to these literals are arbitrary and do not possess any special
//! meaning (other than being hex "words"). They are chosen to be 16-bits wide.
//! The upper 16-bits are reserved for future use.

#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum TestStatus {
    /// Indicates that the CPU has started executing the test_rom code.
    ///
    /// Writing this value to `device_test_status_address()` must not stop
    /// simulation of the current device.
    InBootRom = 0xb090, // 'bogo', BOotrom GO

    /// Indicates that the CPU has seen `ROM_EXEC_EN = 0` and will now stop
    /// execution.
    ///
    /// Writing this value to `device_test_status_address()` must not stop
    /// simulation of the current device.
    InBootRomHalt = 0xb057, // 'bost', BOotrom STop

    /// Indicates that the CPU has started executing the test code.
    ///
    /// Writing this value to `device_test_status_address()` must not stop
    /// simulation of the current device.
    InTest = 0x4354, // 'test'

    /// Indicates that the CPU is in the WFI state.
    ///
    /// Writing this value to `device_test_status_address()` must not stop
    /// simulation of the current device.
    InWfi = 0x1d1e, // 'idle'

    /// This indicates that the test has passed. This is a terminal state. Any code
    /// appearing after this value is set is unreachable.
    ///
    /// Writing this value to `device_test_status_address()` may stop simulation
    /// of the current device.
    Passed = 0x900d, // 'good'

    /// This indicates that the test has failed. This is a terminal state. Any code
    /// appearing after this value is set is unreachable.
    ///
    /// Writing this value to `device_test_status_address()` may stop simulation
    /// of the current device.
    Failed = 0xbaad, // 'baad'
}

impl TestStatus {
    pub fn set(self, ibex: &mut hal::rv_core_ibex::RvCoreIbex) {
        ibex.test_status(self as u32);
    }
}

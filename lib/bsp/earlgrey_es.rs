// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use crate::chip::*;

pub struct Verilator;
impl Clocks for Verilator {}

pub struct Silicon;
impl Clocks for Silicon {
    const CPU_CLOCK_HZ: u32 = 100_000_000;
    const PERIPHERAL_CLOCK_HZ: u32 = 24_000_000;
    const HI_SPEED_PERIPHERAL_CLOCK_HZ: u32 = 96_000_000;
    const BAUD: u32 = 115200;
}

pub struct EarlgreyEs;
impl PeripheralInput for EarlgreyEs {}
impl PeripheralOutput for EarlgreyEs {}

impl PadInput for EarlgreyEs {}
impl PadOutput for EarlgreyEs {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, test_utils_macros::PlicVectorTable)]
#[repr(usize)]
pub enum PlicIrqId {
    Nil = 0,
    Uart0TxWatermark = 1,
    Uart0RxWatermark = 2,
    Uart0TxEmpty = 3,
    Uart0RxOverflow = 4,
    Uart0RxFrameErr = 5,
    Uart0RxBreakErr = 6,
    Uart0RxTimeout = 7,
    Uart0RxParityErr = 8,
    Uart1TxWatermark = 9,
    Uart1RxWatermark = 10,
    Uart1TxEmpty = 11,
    Uart1RxOverflow = 12,
    Uart1RxFrameErr = 13,
    Uart1RxBreakErr = 14,
    Uart1RxTimeout = 15,
    Uart1RxParityErr = 16,
    Uart2TxWatermark = 17,
    Uart2RxWatermark = 18,
    Uart2TxEmpty = 19,
    Uart2RxOverflow = 20,
    Uart2RxFrameErr = 21,
    Uart2RxBreakErr = 22,
    Uart2RxTimeout = 23,
    Uart2RxParityErr = 24,
    Uart3TxWatermark = 25,
    Uart3RxWatermark = 26,
    Uart3TxEmpty = 27,
    Uart3RxOverflow = 28,
    Uart3RxFrameErr = 29,
    Uart3RxBreakErr = 30,
    Uart3RxTimeout = 31,
    Uart3RxParityErr = 32,
    Gpio0 = 33,
    Gpio1 = 34,
    Gpio2 = 35,
    Gpio3 = 36,
    Gpio4 = 37,
    Gpio5 = 38,
    Gpio6 = 39,
    Gpio7 = 40,
    Gpio8 = 41,
    Gpio9 = 42,
    Gpio10 = 43,
    Gpio11 = 44,
    Gpio12 = 45,
    Gpio13 = 46,
    Gpio14 = 47,
    Gpio15 = 48,
    Gpio16 = 49,
    Gpio17 = 50,
    Gpio18 = 51,
    Gpio19 = 52,
    Gpio20 = 53,
    Gpio21 = 54,
    Gpio22 = 55,
    Gpio23 = 56,
    Gpio24 = 57,
    Gpio25 = 58,
    Gpio26 = 59,
    Gpio27 = 60,
    Gpio28 = 61,
    Gpio29 = 62,
    Gpio30 = 63,
    Gpio31 = 64,
    SpiDeviceGenericRxFull = 65,
    SpiDeviceGenericRxWatermark = 66,
    SpiDeviceGenericTxWatermark = 67,
    SpiDeviceGenericRxError = 68,
    SpiDeviceGenericRxOverflow = 69,
    SpiDeviceGenericTxUnderflow = 70,
    SpiDeviceUploadCmdfifoNotEmpty = 71,
    SpiDeviceUploadPayloadNotEmpty = 72,
    SpiDeviceUploadPayloadOverflow = 73,
    SpiDeviceReadbufWatermark = 74,
    SpiDeviceReadbufFlip = 75,
    SpiDeviceTpmHeaderNotEmpty = 76,
    I2c0FmtThreshold = 77,
    I2c0RxThreshold = 78,
    I2c0FmtOverflow = 79,
    I2c0RxOverflow = 80,
    I2c0Nak = 81,
    I2c0SclInterference = 82,
    I2c0SdaInterference = 83,
    I2c0StretchTimeout = 84,
    I2c0SdaUnstable = 85,
    I2c0CmdComplete = 86,
    I2c0TxStretch = 87,
    I2c0TxOverflow = 88,
    I2c0AcqFull = 89,
    I2c0UnexpStop = 90,
    I2c0HostTimeout = 91,
    I2c1FmtThreshold = 92,
    I2c1RxThreshold = 93,
    I2c1FmtOverflow = 94,
    I2c1RxOverflow = 95,
    I2c1Nak = 96,
    I2c1SclInterference = 97,
    I2c1SdaInterference = 98,
    I2c1StretchTimeout = 99,
    I2c1SdaUnstable = 100,
    I2c1CmdComplete = 101,
    I2c1TxStretch = 102,
    I2c1TxOverflow = 103,
    I2c1AcqFull = 104,
    I2c1UnexpStop = 105,
    I2c1HostTimeout = 106,
    I2c2FmtThreshold = 107,
    I2c2RxThreshold = 108,
    I2c2FmtOverflow = 109,
    I2c2RxOverflow = 110,
    I2c2Nak = 111,
    I2c2SclInterference = 112,
    I2c2SdaInterference = 113,
    I2c2StretchTimeout = 114,
    I2c2SdaUnstable = 115,
    I2c2CmdComplete = 116,
    I2c2TxStretch = 117,
    I2c2TxOverflow = 118,
    I2c2AcqFull = 119,
    I2c2UnexpStop = 120,
    I2c2HostTimeout = 121,
    PattgenDoneCh0 = 122,
    PattgenDoneCh1 = 123,
    RvTimerTimerExpiredHart0Timer0 = 124,
    OtpCtrlOtpOperationDone = 125,
    OtpCtrlOtpError = 126,
    AlertHandlerClassa = 127,
    AlertHandlerClassb = 128,
    AlertHandlerClassc = 129,
    AlertHandlerClassd = 130,
    SpiHost0Error = 131,
    SpiHost0SpiEvent = 132,
    SpiHost1Error = 133,
    SpiHost1SpiEvent = 134,
    UsbdevPktReceived = 135,
    UsbdevPktSent = 136,
    UsbdevDisconnected = 137,
    UsbdevHostLost = 138,
    UsbdevLinkReset = 139,
    UsbdevLinkSuspend = 140,
    UsbdevLinkResume = 141,
    UsbdevAvEmpty = 142,
    UsbdevRxFull = 143,
    UsbdevAvOverflow = 144,
    UsbdevLinkInErr = 145,
    UsbdevRxCrcErr = 146,
    UsbdevRxPidErr = 147,
    UsbdevRxBitstuffErr = 148,
    UsbdevFrame = 149,
    UsbdevPowered = 150,
    UsbdevLinkOutErr = 151,
    PwrmgrAonWakeup = 152,
    SysrstCtrlAonEventDetected = 153,
    AdcCtrlAonMatchDone = 154,
    AonTimerAonWkupTimerExpired = 155,
    AonTimerAonWdogTimerBark = 156,
    SensorCtrlAonIoStatusChange = 157,
    SensorCtrlAonInitStatusChange = 158,
    FlashCtrlProgEmpty = 159,
    FlashCtrlProgLvl = 160,
    FlashCtrlRdFull = 161,
    FlashCtrlRdLvl = 162,
    FlashCtrlOpDone = 163,
    FlashCtrlCorrErr = 164,
    HmacHmacDone = 165,
    HmacFifoEmpty = 166,
    HmacHmacErr = 167,
    KmacKmacDone = 168,
    KmacFifoEmpty = 169,
    KmacKmacErr = 170,
    OtbnDone = 171,
    KeymgrOpDone = 172,
    CsrngCsCmdReqDone = 173,
    CsrngCsEntropyReq = 174,
    CsrngCsHwInstExc = 175,
    CsrngCsFatalErr = 176,
    EntropySrcEsEntropyValid = 177,
    EntropySrcEsHealthTestFailed = 178,
    EntropySrcEsObserveFifoReady = 179,
    EntropySrcEsFatalErr = 180,
    Edn0CmdReqDone = 181,
    Edn0FatalErr = 182,
    Edn1CmdReqDone = 183,
    Edn1FatalErr = 184,
    Total = 185,
}

impl TryFrom<usize> for PlicIrqId {
    type Error = usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= 184 {
            // Safety: The enum is repr(u32) and all values 0..=184 are defined.
            Ok(unsafe { core::mem::transmute::<usize, Self>(value) })
        } else {
            Err(value)
        }
    }
}

use riscv::result::Error;
unsafe impl riscv::InterruptNumber for PlicIrqId {
    const MAX_INTERRUPT_NUMBER: usize = Self::Total as usize;

    // Required methods
    fn number(self) -> usize {
        self as usize
    }
    fn from_number(value: usize) -> Result<Self, Error> {
        Self::try_from(value).map_err(|_| Error::InvalidVariant(value))
    }
}

unsafe impl riscv::ExternalInterruptNumber for PlicIrqId {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum PlicPeripheralId {
    Unknown = 0,
    Uart0 = 1,
    Uart1 = 2,
    Uart2 = 3,
    Uart3 = 4,
    Gpio = 5,
    SpiDevice = 6,
    I2c0 = 7,
    I2c1 = 8,
    I2c2 = 9,
    Pattgen = 10,
    RvTimer = 11,
    OtpCtrl = 12,
    AlertHandler = 13,
    SpiHost0 = 14,
    SpiHost1 = 15,
    Usbdev = 16,
    PwrmgrAon = 17,
    SysrstCtrlAon = 18,
    AdcCtrlAon = 19,
    AonTimerAon = 20,
    SensorCtrlAon = 21,
    FlashCtrl = 22,
    Hmac = 23,
    Kmac = 24,
    Otbn = 25,
    Keymgr = 26,
    Csrng = 27,
    EntropySrc = 28,
    Edn0 = 29,
    Edn1 = 30,
}

impl TryFrom<usize> for PlicPeripheralId {
    type Error = usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= 30 {
            // Safety: The enum is repr(u32) and all values 0..=30 are defined.
            Ok(unsafe { core::mem::transmute::<usize, Self>(value) })
        } else {
            Err(value)
        }
    }
}

#[riscv_rt::core_interrupt(riscv::interrupt::Interrupt::MachineExternal)]
fn machine_external_interrupt_handler() {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut plic = hal::Plic::new(peri.plic);
    let irq_id = PlicIrqId::try_from(plic.interrupt_claim()).unwrap();

    // The linker guarantees that PLIC_VECTOR_TABLE[claim_id] will either be
    // the user's strong function OR our weak DefaultPlicHandler.
    // PLIC_VECTOR_TABLE is created by proc macro PlicVectorTable.
    unsafe {
        PLIC_VECTOR_TABLE[irq_id as usize]();
    }
    plic.interrupt_complete(irq_id);
}

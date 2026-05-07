// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::HalError;
use crate::spi_host::SpiHost;
use embedded_hal::spi;
use embedtrait::spi_host;

impl embedded_hal::spi::ErrorType for SpiHost {
    type Error = crate::HalError;
}

impl embedded_hal::spi::Error for crate::HalError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}
impl embedded_hal::spi::SpiDevice for SpiHost {
    fn transaction(
        &mut self,
        operations: &mut [spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        for operation in operations.iter_mut() {
            match operation {
                spi::Operation::Read(words) => spi::SpiBus::read(self, words),
                spi::Operation::Write(words) => spi::SpiBus::write(self, words),
                spi::Operation::Transfer(read, write) => spi::SpiBus::transfer(self, read, write),
                spi::Operation::TransferInPlace(words) => {
                    spi::SpiBus::transfer_in_place(self, words)
                }
                spi::Operation::DelayNs(_ns) => {
                    return Err(HalError::NotSupported);
                }
            }?
        }
        Ok(())
    }
}

impl spi::SpiBus for SpiHost {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        spi_host::SpiBus::transfer(
            self,
            &mut [spi_host::Operation {
                data: spi_host::Data::Read(words),
                width: spi_host::Width::Single,
            }],
        )
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        spi_host::SpiBus::transfer(
            self,
            &mut [spi_host::Operation {
                data: spi_host::Data::Write(words),
                width: spi_host::Width::Single,
            }],
        )?;
        self.wait_idle();
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        assert!(write.len() <= read.len());
        spi_host::SpiBus::transfer(
            self,
            &mut [
                spi_host::Operation {
                    data: spi_host::Data::Write(write),
                    width: spi_host::Width::Single,
                },
                spi_host::Operation {
                    data: spi_host::Data::Read(read),
                    width: spi_host::Width::Single,
                },
            ],
        )
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        spi_host::SpiBus::transfer(
            self,
            &mut [spi_host::Operation {
                data: spi_host::Data::Write(words),
                width: spi_host::Width::Single,
            }],
        )?;
        spi_host::SpiBus::transfer(
            self,
            &mut [spi_host::Operation {
                data: spi_host::Data::Read(words),
                width: spi_host::Width::Single,
            }],
        )
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

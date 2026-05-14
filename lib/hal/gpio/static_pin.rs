// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::pin::{self, Pin};

pub struct StaticPin<const ID: usize, S = pin::InputState> {
    addr: usize,
    _phantom: core::marker::PhantomData<S>,
}

impl<const ID: usize, S> StaticPin<ID, S> {
    pub fn new(addr: usize) -> Self {
        Self {
            addr,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<const ID: usize> StaticPin<ID, pin::InputState> {
    pub fn into_output(self) -> StaticPin<ID, pin::OutputState> {
        self.port().output_en(ID, true);
        StaticPin::<ID, pin::OutputState>::new(self.addr)
    }

    pub fn interrupt_set_enable(&mut self, en: bool) -> &mut Self {
        self.port().interrupt_set_enable(ID, en);
        self
    }

    pub fn interrupt_set_mode(&mut self, en: bool, modes: &[crate::InterruptMode]) -> &mut Self {
        self.port().interrupt_set_mode(ID, en, modes);
        self
    }

    pub fn interrupt_pending(&mut self) -> bool {
        self.port().interrupt_pending(ID)
    }

    pub fn interrupt_clear(&mut self) -> &mut Self {
        self.port().interrupt_clear(ID);
        self
    }

    pub fn interrupt_force(&mut self) -> &mut Self {
        self.port().interrupt_force(ID);
        self
    }
}

impl<const ID: usize> StaticPin<ID, pin::OutputState> {
    pub fn into_input(self) -> StaticPin<ID, pin::InputState> {
        self.port().output_en(ID, false);
        StaticPin::<ID, pin::InputState>::new(self.addr)
    }
}

impl<const ID: usize, S> pin::Pin for StaticPin<ID, S> {
    fn id(&self) -> usize {
        ID
    }

    fn port(&self) -> super::Gpio {
        super::Gpio::from_addr(self.addr)
    }
}

seq_macro::seq!(N in 0..32 {
    #(
        pub type Pin~N<S = pin::InputState> = StaticPin<N,S>;
    )*
    pub struct GpioPins {
        #(
            pub pin~N: Pin~N,
        )*
    }

    impl GpioPins {
        pub fn new(addr: usize) -> Self {
            Self {
                #(
                    pin~N: Pin~N::new(addr),
                )*
            }
        }
    }
});

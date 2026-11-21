// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use bsp::earlgrey_es::*;

use core::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use critical_section::Mutex;

pub static TIMER: Mutex<RefCell<Option<AsyncTimer>>> = Mutex::new(RefCell::new(None));

use heapless::binary_heap::{BinaryHeap, Min};

pub struct AsyncTimer {
    timer: hal::RawTimer,
    wake_deadlines: BinaryHeap<(u64, WakerWrapper), Min, 4>,
}

impl AsyncTimer {
    pub fn init(timer: hal::RawTimer) {
        critical_section::with(|cs| {
            TIMER.borrow(cs).replace(Some(Self {
                timer,
                wake_deadlines: BinaryHeap::new(),
            }));
        })
    }

    pub fn start(&mut self, waker: Waker, ticks: u64) {
        let is_earliest = if let Some((next, _)) = self.wake_deadlines.peek() {
            ticks < *next
        } else {
            true
        };
        if self
            .wake_deadlines
            .push((ticks, WakerWrapper(waker)))
            .is_err()
        {
            panic!("Deadline dropped for task");
        }
        if is_earliest {
            self.schedule_wakeup();
        }
    }

    pub fn schedule_wakeup(&mut self) {
        self.timer.clear_interrupt();
        while let Some((deadline, waker)) = self.wake_deadlines.peek() {
            if *deadline > self.timer.get_ticks() + 1 {
                self.timer.set_compare(*deadline);
                self.timer.set_interrupt(true);
                unsafe {
                    riscv::interrupt::enable_interrupt(riscv::interrupt::Interrupt::MachineTimer);
                }
                break;
            }
            waker.0.wake_by_ref();
            self.wake_deadlines.pop();
        }

        if self.wake_deadlines.is_empty() {
            self.timer.set_interrupt(false);
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    Init,
    Wait,
}

struct Timer {
    state: State,
    timeout: u64,
}

impl Timer {
    pub fn new<const TICK_HZ: u64, D>(duration: D) -> Self
    where
        D: Into<fugit::Duration<u64, 1, TICK_HZ>>,
    {
        Self {
            state: State::Init,
            timeout: duration.into().as_ticks(),
        }
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            State::Init => {
                self.state = State::Wait;
                critical_section::with(|cs| {
                    let mut borrow = TIMER.borrow(cs).borrow_mut();
                    let t = borrow.as_mut().unwrap();

                    self.timeout += t.timer.get_ticks();
                    t.start(ctx.waker().clone(), self.timeout);
                });
                Poll::Pending
            }
            State::Wait => {
                let mut now = 0;
                critical_section::with(|cs| {
                    let mut borrow = TIMER.borrow(cs).borrow_mut();
                    let t = borrow.as_mut().unwrap();
                    now = t.timer.get_ticks();
                });
                if now >= self.timeout {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

pub async fn delay<const TICK_HZ: u64, D>(duration: D)
where
    D: Into<fugit::Duration<u64, 1, TICK_HZ>>,
{
    Timer::new(duration).await
}

// Wapper implementing the Ord trait.
struct WakerWrapper(Waker);
impl PartialEq for WakerWrapper {
    fn eq(&self, other: &Self) -> bool {
        // Wakers have a built-in method to check if they wake the same task
        self.0.will_wake(&other.0)
    }
}

impl Eq for WakerWrapper {}

use core::cmp::Ordering;
impl PartialOrd for WakerWrapper {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for WakerWrapper {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

#[riscv_rt::external_interrupt(PlicIrqId::RvTimerTimerExpiredHart0Timer0)]
fn timer_irs() {
    critical_section::with(|cs| {
        TIMER
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .schedule_wakeup();
    });
}

// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use bsp::earlgrey_es::*;

use core::sync::atomic::AtomicUsize;
const INVALID_TASK_ID: usize = 0xFF;
const DEFAULT_TASK_ID: AtomicUsize = AtomicUsize::new(INVALID_TASK_ID);
static TASK_ID: AtomicUsize = DEFAULT_TASK_ID;

use heapless::binary_heap::{BinaryHeap, Min};

// static WAKE_DEADLINES: Mutex<RefCell<BinaryHeap<(u64, usize), Min, 4>>> =
//     Mutex::new(RefCell::new(BinaryHeap::new()));

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    Init,
    Wait,
}

pub struct TimerTask<const CLOCK: u64> {
    timer: hal::RvTimer<CLOCK>,
    state: State,
    wake_deadlines: BinaryHeap<(u64, usize), Min, 4>,
}

use embedtrait::timer::Timer;
impl<const CLOCK: u64> TimerTask<CLOCK> {
    pub fn new(timer: hal::RvTimer<CLOCK>) -> Self {
        Self {
            timer,
            state: State::Init,
            wake_deadlines: BinaryHeap::new(),
        }
    }

    pub fn start<const TICK_HZ: u64, T>(&mut self, task_id: usize, duration: T)
    where
        T: Into<fugit::Duration<u64, 1, TICK_HZ>>,
        fugit::Duration<u64, 1, 1000000>: From<T>,
    {
        let mut count_down = self.timer.new_timer();
        count_down.start(duration);
        let new_deadline = count_down.end_time.as_ticks();
        let is_earliest = if let Some((next, _)) = self.wake_deadlines.peek() {
            new_deadline < *next
        } else {
            true
        };
        if self.wake_deadlines.push((new_deadline, task_id)).is_err() {
            panic!("Deadline dropped for task {}", task_id);
        }
        if is_earliest {
            self.schedule_wakeup();
        }
    }

    pub fn schedule_wakeup(&mut self) {
        self.timer.clear_interrupt();
        while let Some((deadline, task_id)) = self.wake_deadlines.peek() {
            if *deadline > self.timer.get_ticks() + 1 {
                self.timer.set_compare(*deadline);
                self.timer.set_interrupt(true);
                unsafe {
                    riscv::interrupt::enable_interrupt(riscv::interrupt::Interrupt::MachineTimer);
                }
                break;
            }
            crate::my_executor::wake_task(*task_id);
            self.wake_deadlines.pop();
        }

        if self.wake_deadlines.is_empty() {
            self.timer.set_interrupt(false);
        }
    }
}

use crate::my_future::{Poll, ReisFuture};
impl<const CLOCK: u64> ReisFuture for TimerTask<CLOCK> {
    type Output = ();
    fn poll(&mut self, id: usize) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::Init => {
                    self.state = State::Wait;
                    TASK_ID.store(id, core::sync::atomic::Ordering::Relaxed);
                }
                State::Wait => {
                    self.schedule_wakeup();
                    break;
                }
            }
        }
        Poll::Pending
    }
}

use bsp::prelude::*;
#[riscv_rt::external_interrupt(PlicIrqId::RvTimerTimerExpiredHart0Timer0)]
fn timer_irs() {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };

    let mut timer: hal::RvTimer<{ Silicon::PERIPHERAL_CLOCK_HZ as u64 }> =
        hal::RvTimer::new(peri.timer);
    timer.clear_interrupt();
    crate::my_executor::wake_task(TASK_ID.load(core::sync::atomic::Ordering::Relaxed));
    riscv::interrupt::disable_interrupt(riscv::interrupt::Interrupt::MachineTimer);
}

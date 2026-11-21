// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use crate::my_future::{Poll, ReisFuture};
use crate::tasks::timer;
use fugit::ExtU64;
use heapless::mpmc::Queue;

static TASK_ID_READY: Queue<usize, 8> = Queue::new();

pub fn wake_task(id: usize) {
    if TASK_ID_READY.enqueue(id).is_err() {
        panic!("Task queue is full, consider increasing space.");
    }
}

pub fn run<const N: usize, const CLOCK: u64>(
    mut timer: timer::TimerTask<CLOCK>,
    tasks: &mut [&mut dyn ReisFuture<Output = ()>; N],
) -> ! {
    for id in 0..tasks.len() {
        TASK_ID_READY.enqueue(id).ok();
    }
    let timer_id = tasks.len();
    TASK_ID_READY.enqueue(timer_id).ok();
    loop {
        while let Some(id) = TASK_ID_READY.dequeue() {
            if id == timer_id {
                timer.poll(timer_id);
                continue;
            }

            match tasks[id].poll(id) {
                Poll::Sleep(t) => timer.start(id, t.millis()),
                _ => continue,
            }
        }
        unsafe {
            riscv::interrupt::enable();
        }
        riscv::asm::wfi();
    }
}

// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use core::{
    future::Future,
    pin::Pin,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use heapless::mpmc::Queue;

static TASK_ID_READY: Queue<usize, 8> = Queue::new();

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

unsafe fn clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

unsafe fn drop(_p: *const ()) {}

unsafe fn wake(p: *const ()) {
    wake_task(p as usize);
}

unsafe fn wake_by_ref(p: *const ()) {
    wake_task(p as usize);
}

fn get_waker(task_id: usize) -> Waker {
    unsafe { Waker::from_raw(RawWaker::new(task_id as *const (), &VTABLE)) }
}

pub fn wake_task(id: usize) {
    if TASK_ID_READY.enqueue(id).is_err() {
        panic!("Task queue is full, consider increasing space.");
    }
}

pub fn run(tasks: &mut [Pin<&mut dyn Future<Output = ()>>]) -> ! {
    for id in 0..tasks.len() {
        TASK_ID_READY.enqueue(id).ok();
    }
    loop {
        while let Some(id) = TASK_ID_READY.dequeue() {
            _ = tasks[id]
                .as_mut()
                .poll(&mut Context::from_waker(&get_waker(id)));
        }
        unsafe {
            riscv::interrupt::enable();
        }
        riscv::asm::wfi();
        riscv::interrupt::disable();
    }
}

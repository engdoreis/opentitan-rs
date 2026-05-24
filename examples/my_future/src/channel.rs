// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use core::cell::Cell;

pub struct Channel<T> {
    item: Cell<Option<T>>,
    task_id: Cell<Option<usize>>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            item: Cell::new(None),
            task_id: Cell::new(None),
        }
    }

    fn send(&self, item: T) {
        self.item.replace(Some(item));
        if let Some(id) = self.task_id.get() {
            crate::my_executor::wake_task(id);
        }
    }

    pub fn get_sender(&self) -> Sender<'_, T> {
        Sender { channel: self }
    }

    fn receive(&self) -> Option<T> {
        self.item.take()
    }

    fn register(&self, task_id: usize) {
        self.task_id.replace(Some(task_id));
    }

    pub fn get_receiver(&self) -> Receiver<'_, T> {
        Receiver { channel: self }
    }
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
    pub fn send(&self, item: T) {
        self.channel.send(item);
    }
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Receiver<'_, T> {
    pub fn receive(&self, task_id: usize) -> Option<T> {
        // Register task id to enable the sender to wakeup task.
        self.channel.register(task_id);
        self.channel.receive()
    }
}

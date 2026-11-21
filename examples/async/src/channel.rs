// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use core::{
    cell::{Cell, RefCell},
    task::{Poll, Waker},
};

pub struct Channel<T> {
    item: Cell<Option<T>>,
    waker: RefCell<Option<Waker>>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            item: Cell::new(None),
            waker: RefCell::new(None),
        }
    }

    fn send(&self, item: T) {
        self.item.replace(Some(item));
        if let Some(waker) = self.waker.borrow().as_ref() {
            waker.wake_by_ref();
        }
    }

    pub fn get_sender(&self) -> Sender<'_, T> {
        Sender { channel: self }
    }

    fn receive(&self) -> Option<T> {
        self.item.take()
    }

    fn register(&self, waker: Waker) {
        self.waker.replace(Some(waker));
    }

    pub fn get_receiver(&self) -> Receiver<'_, T> {
        Receiver {
            channel: self,
            state: State::Init,
        }
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

enum State {
    Init,
    Waiting,
}
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    state: State,
}

impl<T> Receiver<'_, T> {
    pub async fn receive(&mut self) -> T {
        core::future::poll_fn(|ctx| match self.state {
            State::Init => {
                self.channel.register(ctx.waker().clone());
                self.state = State::Waiting;
                Poll::Pending
            }
            State::Waiting => match self.channel.receive() {
                Some(item) => Poll::Ready(item),
                None => Poll::Pending,
            },
        })
        .await
    }
}

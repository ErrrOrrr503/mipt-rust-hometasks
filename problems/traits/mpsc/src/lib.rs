#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

struct Channel<T> {
    buffer: VecDeque<T>,
    is_hup: bool,
    senders_amount: usize,
}

impl<T> Channel<T> {
    fn send (&mut self, value: T) -> Result<(), SendError<T>> {
        if !self.is_hup {
            self.buffer.push_back(value);
            return Ok(());
        }
        Err(SendError { value })
    }

    fn recv (&mut self) -> Result<T, ReceiveError> {
        if !self.buffer.is_empty() {
            return Ok(self.buffer.pop_front().unwrap());
        }
        if self.is_hup {
            return Err(ReceiveError::Closed);
        }
        return Err(ReceiveError::Empty);
    }

    fn hup(&mut self) {
        self.is_hup = true;
    }

    fn is_hup(&self) -> bool {
        self.is_hup
    }

    fn inc_senders_amount(&mut self) {
        self.senders_amount += 1;
    }

    fn dec_senders_amount(&mut self) {
        self.senders_amount -= 1;
    }

    fn senders_amount(&self) -> usize {
        self.senders_amount
    }
}

pub struct Sender<T> {
    channel: Rc<RefCell<Channel<T>>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.channel.borrow_mut().send(value)
    }

    pub fn is_closed(&self) -> bool {
        self.channel.borrow().is_hup()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.channel, &other.channel)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.channel.borrow_mut().inc_senders_amount();
        Self {
            channel: self.channel.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.channel.borrow_mut().dec_senders_amount();
        if self.channel.borrow().senders_amount() == 0 {
            self.channel.borrow_mut().hup();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Receiver<T> {
    channel: Rc<RefCell<Channel<T>>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError>
    {
        self.channel.borrow_mut().recv()
    }

    pub fn close(&mut self) {
        self.channel.borrow_mut().hup();
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.channel.borrow_mut().hup();
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Rc::new(RefCell::new(Channel {
        senders_amount: 1,
        buffer: VecDeque::new(),
        is_hup: false,
    }));
    (Sender {
        channel: channel.clone(),
    },
    Receiver {
        channel: channel,
    })
}

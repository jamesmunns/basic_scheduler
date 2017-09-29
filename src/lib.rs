extern crate chrono;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::cmp::Ordering;

use chrono::prelude::*;

mod events;

pub use self::events::{BasicEvent, Eventer};
pub use chrono::Duration;

type InternalTime = DateTime<Utc>;

pub struct Scheduler {
    stuff: Vec<ScheduledEvent>,
    new_items_rx: Receiver<Box<Eventer + Send>>,
    new_items_tx: Sender<Box<Eventer + Send>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let (tx,rx) = channel();
        Scheduler {
            stuff: vec!(),
            new_items_tx: tx,
            new_items_rx: rx,
        }
    }

    pub fn add_handle(&self) -> Sender<Box<Eventer + Send>> {
        self.new_items_tx.clone()
    }

    pub fn run(&mut self) {
        loop {
            self.step()
        }
    }

    fn step(&mut self) {
        let time_to_next = self.process_pending();

        match self.new_items_rx.recv_timeout(time_to_next.to_std().unwrap()) {
            Ok(evt) => {
                //println!("PING");
                let mut new_evts: Vec<_> =
                    self.new_items_rx.try_iter().fold(vec![evt], |mut acc, x| {
                        acc.push(x);
                        acc
                    });

                // Immediately run all new tasks
                for evt in new_evts.drain(..) {
                    self.process_single(evt);
                }
            }
            _ => {
                //println!("PONG");
                // Timeout, its probably time to run an event
            }
        }
    }

    fn process_single(&mut self, mut evt: Box<Eventer + Send>) {
        match evt.execute() {
            Some(d) => {
                // reschedule
                self.insert(ScheduledEvent {
                    when_next: Utc::now() + d,
                    what: evt,
                });
            }
            None => {} // Nothing to reschedule
        }
    }

    fn process_pending(&mut self) -> Duration {
        // println!("Processing Pending");
        loop {
            // Is there a pending item?
            if self.stuff.len() == 0 {
                return Duration::hours(24);
            }

            let now = Utc::now();
            let next = self.stuff
                .get(0)
                .unwrap()
                .when_next;

            if next <= now {
                let x = self.stuff.remove(0);
                self.process_single(x.what);
            } else {
                return next.signed_duration_since(now);
            }
        }
    }

    fn insert(&mut self, evt: ScheduledEvent) {
        let idx = match self.stuff.binary_search(&evt) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        self.stuff.insert(idx, evt);
    }
}

struct ScheduledEvent {
    when_next: InternalTime,
    what: Box<Eventer + Send>,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &ScheduledEvent) -> Ordering {
        self.when_next.cmp(&other.when_next)
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &ScheduledEvent) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &ScheduledEvent) -> bool {
        self.when_next == other.when_next
    }
}

// This probably shouldn't be a thing
impl Eq for ScheduledEvent {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

    }
}

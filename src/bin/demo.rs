extern crate basic_scheduler;
use std::time::{Instant};
use std::time::Duration as OldDuration;
use basic_scheduler::*;
use std::thread::{spawn, sleep};

struct Repeater<T> {
    count: u8,
    data: T,
    start: Instant
}

fn main() {
    let mut z = Scheduler::new();
    let a = z.add_handle();

    let x = BasicEvent {
        task: |s: &mut Repeater<u32>| {
            println!("{:?} '{}' {:?}", s.data, s.count, s.start.elapsed());
            if s.count > 0 {
                s.count -= 1;
                Some(Duration::seconds(3))
            } else {
                None
            }
        },
        state: Repeater {
            count: 10,
            data: 16u32,
            start: Instant::now(),
        },
    };

    let y = BasicEvent {
        task: |s: &mut Repeater<String>| {
            println!("{:?} '{}' {:?}", s.data, s.count, s.start.elapsed());
            if s.count > 0 {
                s.count -= 1;
                Some(Duration::seconds(5))
            } else {
                None
            }
        },
        state: Repeater {
            count: 5,
            data: "foo".to_string(),
            start: Instant::now(),
        },
    };

    spawn(move || {
        sleep(OldDuration::from_millis(500));
        a.send(Box::new(x)).unwrap();
        sleep(OldDuration::from_millis(1000));
        a.send(Box::new(y)).unwrap();
    });

    z.run();


}

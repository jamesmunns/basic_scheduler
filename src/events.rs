use chrono::Duration;

pub trait Eventer {
    fn execute(&mut self) -> Option<Duration>;
}

pub struct BasicEvent<F, T>
where
    F: FnMut(&mut T) -> Option<Duration>,
{
    pub task: F,
    pub state: T,
}

impl<F, T> Eventer for BasicEvent<F, T>
where
    F: FnMut(&mut T) -> Option<Duration>,
{
    fn execute(&mut self) -> Option<Duration> {
        (self.task)(&mut self.state)
    }
}

#[allow(dead_code)]
pub struct OneShot<F>
where
    F: Fn()
{
    delay: Option<Duration>,
    task: F,
}

impl<F> OneShot<F>
where
    F: Fn()
{
    #[allow(dead_code)]
    fn new(delay: Duration, task: F) -> Self {
        Self {
            delay: Some(delay),
            task: task
        }
    }
}

impl<F> Eventer for OneShot<F>
where
    F: Fn()
{
    fn execute(&mut self) -> Option<Duration> {
        if self.delay.is_some() {
            let mut x = None;
            ::std::mem::swap(&mut x, &mut self.delay);
            x
        } else {
            (self.task)();
            None
        }
    }
}

#[macro_use]
extern crate tokio01_test;
extern crate futures;
extern crate tokio_timer;

use futures::Future;
use std::time::{Duration, Instant};
use tokio01_test::clock::MockClock;
use tokio01_test::task::MockTask;
use tokio_timer::Delay;

#[test]
fn clock() {
    let mut mock = MockClock::new();

    mock.enter(|handle| {
        let deadline = Instant::now() + Duration::from_secs(1);
        let mut delay = Delay::new(deadline);

        assert_not_ready!(delay.poll());

        handle.advance(Duration::from_secs(2));

        assert_ready!(delay.poll());
    });
}

#[test]
fn notify() {
    let deadline = Instant::now() + Duration::from_secs(1);
    let mut mock = MockClock::new();
    let mut task = MockTask::new();

    mock.enter(|handle| {
        let mut delay = Delay::new(deadline);

        task.enter(|| assert_not_ready!(delay.poll()));

        handle.advance(Duration::from_secs(1));

        assert!(task.is_notified());
        assert_ready!(delay.poll());
    });
}

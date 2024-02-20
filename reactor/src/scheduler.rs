use std::future::Future;

use tokio::task::JoinHandle;

pub trait LocalScheduler {
    fn spawn_local<Fut: Future + 'static>(future: Fut) -> JoinHandle<Fut::Output>;
}

pub trait Scheduler {
    fn spawn<Fut: Future + Sync + Send + 'static>(future: Fut) -> JoinHandle<Fut::Output>;
}

use futures::future::LocalBoxFuture;

pub struct Task {
    future: Option<LocalBoxFuture<'static, ()>>
}

impl Task {
    pub fn new<Fut: Future + 'static>(future: Fut) -> Self {
        Self{future: Box::pin(future)}
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        EXECUTOR.enqueue(self.clone());
    }
}

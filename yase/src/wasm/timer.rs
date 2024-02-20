use std::task::Waker;
use std::rc::Rc;
use std::cell::RefCell;

use super::set_timeout;

#[derive(Default)]
struct Inner {
    done: bool,
    waker: Option<Waker>
}

pub struct TimerFuture(Rc<RefCell<Inner>>);

impl TimerFuture {
    pub fn new(ms: u32) -> Self {
        let inner = Rc::RefCell(Inner::default());
        let in2 = inner.clone();
        
        set_timeout(
            Closure::once_into_js(move || {
                in2.borrow_mut().done = true;
                if let Some(waker) = in2.borrow_mut().waker.take() {
                    waker.wake()
                }
            }),
            ms,
        );

        return TimerFuture(in2);
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let done = self.inner.borrow().done;
        if done {
            Poll::Ready(())
        } else {
            self.0.borrow_mut().waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
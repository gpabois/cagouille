mod any;
pub use any::AnyAction;

use crate::local::Context;

type BoxedAction<Matter> = Box<dyn FnOnce(Context<Matter>) + 'static>;

pub struct Action<Matter>(BoxedAction<Matter>);

impl<Matter> Action<Matter> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Context<Matter>) + 'static,
    {
        Self(Box::new(f))
    }

    pub fn execute(self, ctx: Context<Matter>) {
        let f = self.0;
        f(ctx);
    }
}

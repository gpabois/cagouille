mod any;
pub use any::AnyAction;

use crate::sync::Context;

type BoxedAction<Matter> = Box<dyn FnOnce(Context<Matter>) + Sync + Send + 'static>;

pub struct Action<Matter>(BoxedAction<Matter>);

impl<Matter> Action<Matter> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Context<Matter>) + Sync + Send + 'static,
    {
        Self(Box::new(f))
    }

    pub fn execute(self, ctx: Context<Matter>) {
        let f = self.0;
        f(ctx);
    }
}

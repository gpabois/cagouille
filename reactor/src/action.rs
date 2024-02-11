use crate::Context;

/// An action in the reactor.
pub struct Action<Matter>(Box<dyn FnOnce(Context<Matter>) + Sync + Send + 'static>);

impl<Matter> Action<Matter> {
    pub fn new<F>(f: F) -> Self 
    where F: FnOnce(Context<Matter>) + Sync + Send + 'static {
        Self(Box::new(f))
    }

    pub fn execute(self, ctx: Context<Matter>) {
        let f = self.0;
        f(ctx);
    }
}

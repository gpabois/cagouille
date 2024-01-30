
pub mod reactor;
pub mod interaction;
pub mod ray;
pub mod tracker;

pub use interaction::Interaction;
pub use reactor::Reactor;
pub use ray::Ray;

#[cfg(test)]
mod test {
    use super::{Reactor, Ray};

    pub struct Data {
        foo: Ray<Self, usize>
    }

    #[tokio::test]
    pub async fn simple_test() {
        let reactor = Reactor::new(|r| {
            Data {
                foo: r.use_ray(0)
            }
        });

    }
}
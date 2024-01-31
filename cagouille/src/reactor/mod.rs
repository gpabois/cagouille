
pub mod reactor;
pub mod interaction;
pub mod atom;
pub mod wave;
pub mod tracker;

pub use interaction::Interaction;
pub use reactor::Reactor;
pub use atom::Atom;
pub use wave::Wave;


#[cfg(test)]
mod test {
    use super::{Reactor, Atom};

    pub struct Matter {
        foo: Atom<Self, usize>
    }

    #[tokio::test]
    pub async fn simple_test() {
        let reactor = Reactor::new(|r| {
            Matter {
                foo: r.atom(0)
            }
        });

        reactor.interact(|matter| {
            println!("Reaction {}", *matter.foo);
        }).unwrap().await.unwrap();

        reactor.interact(|matter| {
            // Must trigger the first interaction
            *matter.foo = 10;
        }).unwrap().await.unwrap();
        
        reactor.nuke().unwrap().await.unwrap();
        
        reactor.wait_for_nuke().await;
    }
}
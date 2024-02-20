#[cfg(all(feature = "wasm", not(target_arch = "wasm32-unknown-unknown")))]
compile_error!("wasm feature is only available for wasm32 target");

#[cfg(all(feature = "wasm", feature = "tokio"))]
compile_error!("choose either between wasm or tokio");

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::Executor as Spawner;

#[cfg(feature = "tokio")]
pub use wasm::Executor as Spawner;

pub trait LocalSpawner {
    fn spawn<Fut: Future + 'static>(future: Fut);
}

pub trait Spawner {
    fn spawn<Fut: Future + Sync + Send + 'static>(future: Fut);
    fn spawn_local<Fut: Future + 'static>(future: Fut);
}
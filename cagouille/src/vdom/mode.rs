/// Mounted within a DOM tree.
pub struct BrowserMode;

/// Serialize the virtual dom into a string.
pub struct DebugMode;

impl Mode for DebugMode {
    type ComponentNodeState = ();
}

/// SSR rendering.
pub struct SSRMode;

pub trait Mode: 'static {
    /// Data stored in the component node state for the mode handler.
    type ComponentNodeState: Default;
}

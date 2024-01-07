pub struct VComponent;

pub type VNodeResult = Result<VNode, crate::error::Error>;

pub enum VNode {
    VComponent(VComponent)
}
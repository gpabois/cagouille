/// Receive
pub struct Atom(Box<dyn Fn>);

pub trait Reactor {
    pub fn set_current_atom(atom: Atom);

}

pub struct Ray<D> {
    inner: D,
    atoms: Vec<Atom> 
}
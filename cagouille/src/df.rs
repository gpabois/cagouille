pub mod traits {
    use futures::future::LocalBoxFuture;

    /// A differential between two states of an object
    pub trait Df<T> {
        /// Apply the differential to the destination.
        fn apply(self, dest: &mut T);
    }

    pub trait Differentiable : Sized {
        type Df;

        /// Compute the differential from the source to the destination.
        fn df<'a, 'fut>(src: &'a Self, dest: &'a Self) -> Self::Df;
    }

    /// Compute a differential between two states of an object
    pub trait AsyncDifferentiable : Sized {
        type Df;

        /// Compute the differential from the source to the destination.
        fn df<'a, 'fut>(src: &'a Self, dest: &'a Self) -> LocalBoxFuture<'fut, Self::Df> where 'a: 'fut;
    }
}

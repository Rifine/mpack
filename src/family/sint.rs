pub trait SignedIntFamily: Into<i64> {}

impl<T: Into<i64>> SignedIntFamily for T {}


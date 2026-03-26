use std::marker::PhantomData;

#[cfg(test)]
mod test;

pub struct Id<T, U> {
    inner: U,
    _phantom: PhantomData<T>,
}

impl<T, U> Id<T, U> {
    /// create Id object. you should use this method carefully because value was not checked as valid
    pub fn new(inner: U) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    /// access to internal value reference. you should use this method carefully.
    pub fn inner(&self) -> &U {
        &self.inner
    }
}

crate::__impl_id_core_traits!([T, U] Id<T, U>, U);
crate::__impl_id_serde!([T, U] Id<T, U>, U);
crate::__impl_id_fake!([T, U] Id<T, U>, U);
crate::__impl_id_sqlx_any!([T, U] Id<T, U>, U);
crate::__impl_id_sqlx_mysql!([T, U] Id<T, U>, U);
crate::__impl_id_sqlx_postgres!([T, U] Id<T, U>, U);
crate::__impl_id_sqlx_sqlite!([T, U] Id<T, U>, U);

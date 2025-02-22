use std::sync::{Arc, RwLock};

pub trait ArcRwLock<T>: Sync + Send + 'static {
    type Data;

    fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Self::Data) -> R;

    fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R;
}

pub struct ThreadSafe<T>(Arc<RwLock<T>>);

impl<T> ThreadSafe<T> {
    pub fn new(value: T) -> Self {
        ThreadSafe(Arc::new(RwLock::new(value)))
    }
}

impl<T: Sync + Send + 'static> ArcRwLock<T> for ThreadSafe<T> {
    type Data = T;

    fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.0.read().unwrap())
    }

    fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        f(&mut self.0.write().unwrap())
    }
}

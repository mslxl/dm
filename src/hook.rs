use std::ops::{Deref, DerefMut};

pub struct DropHook<T> {
    value: T,
    hook: Option<Box<dyn Fn(&mut T)>>,
}

impl<T> Drop for DropHook<T> {
    fn drop(&mut self) {
        if let Some(hook) = &self.hook {
            hook(&mut self.value);
        }
    }
}

impl<T> Deref for DropHook<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for DropHook<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> DropHook<T> {
    pub fn new<F>(value: T, hook: F) -> Self
    where
        F: Fn(&mut T) + 'static,
    {
        Self {
            value: value,
            hook: Some(Box::new(hook)),
        }
    }
}

unsafe impl <T> Sync for DropHook<T>{}
unsafe impl <T> Send for DropHook<T>{}
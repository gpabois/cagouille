use std::{cell::RefCell, ops::DerefMut};

use async_std::task::yield_now;


pub struct AsyncCounter(RefCell<usize>);

impl AsyncCounter {
    pub fn new(val: usize) -> Self {
        Self(RefCell::new(val))
    }
    
    pub fn read(&self) -> usize {
        return *self.0.borrow();
    }

    fn write(&self, value: usize) {
        *self.0.borrow_mut() = value;
    }

    pub async fn compare_inc(&self, value: usize) -> usize {
        let val = self.read();
        
        if val + 1 == value {
            self.write(val + 1);
        }

        return val + 1;
    }

    pub fn increment(&self) -> usize {
        let val = self.read();
        self.write(val + 1);
        return val + 1;
    }

    pub fn decrement(&self) -> usize {
        let val = self.read();
        self.write(val - 1);
        return val - 1;
    }
}

pub struct AsyncRefCell<D>(RefCell<D>);

impl<D: Default> Default for AsyncRefCell<D> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<D> AsyncRefCell<D> {
    pub fn  new(value: D) -> Self {
        Self(RefCell::new(value))
    }
    pub async fn borrow(&self) -> std::cell::Ref<'_, D> {
        loop {
            match self.0.try_borrow() {
                Ok(rf) => return rf,
                Err(_) => {},
            }
        }
    }

    pub async fn borrow_mut(&self) -> std::cell::RefMut<'_, D> {
        loop {
            match self.0.try_borrow_mut() {
                Ok(rf) => return rf,
                Err(_) => {},
            }
        }
    }
}
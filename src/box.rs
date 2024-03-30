#![no_std]

use core::ptr;
use core::alloc::Layout;

pub struct Box<T, A: Allocator> {
    ptr: *mut T,
    _marker: core::marker::PhantomData<A>,
}

impl<T, A: Allocator> Box<T, A> {
    pub fn new(value: T) -> Self {
        let ptr = A::alloc(Layout::new::<T>()) as *mut T;
        unsafe {
            ptr::write(ptr, value);
        }
        Box { ptr, _marker: core::marker::PhantomData }
    }
}

impl<T, A: Allocator> Drop for Box<T, A> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr);
            A::dealloc(self.ptr as *mut u8, Layout::new::<T>());
        }
    }
}

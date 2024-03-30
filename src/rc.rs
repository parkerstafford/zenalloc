#![no_std]

use core::ptr;
use core::mem;
use core::alloc::Layout;

pub struct Rc<T, A: Allocator> {
    ptr: *mut T,
    strong: *mut usize,
    _marker: core::marker::PhantomData<A>,
}

impl<T, A: Allocator> Rc<T, A> {
    pub fn new(value: T) -> Self {
        let ptr = A::alloc(Layout::new::<T>()) as *mut T;
        unsafe {
            ptr::write(ptr, value);
        }
        let strong = A::alloc(Layout::new::<usize>()) as *mut usize;
        unsafe {
            ptr::write(strong, 1);
        }
        Rc { ptr, strong, _marker: core::marker::PhantomData }
    }

    pub fn clone(&self) -> Self {
        unsafe {
            let strong = *self.strong;
            *self.strong = strong + 1;
            Rc {
                ptr: self.ptr,
                strong: self.strong,
                _marker: core::marker::PhantomData,
            }
        }
    }
}

impl<T, A: Allocator> Drop for Rc<T, A> {
    fn drop(&mut self) {
        unsafe {
            let strong = *self.strong - 1;
            if strong == 0 {
                ptr::drop_in_place(self.ptr);
                A::dealloc(self.ptr as *mut u8, Layout::new::<T>());
                A::dealloc(self.strong as *mut u8, Layout::new::<usize>());
            } else {
                *self.strong = strong;
            }
        }
    }
}
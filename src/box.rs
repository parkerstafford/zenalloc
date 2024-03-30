use std::ptr;
use std::mem;
use std::alloc::Layout;

pub struct Box<T> {
    ptr: *mut T,
    alloc: Box<dyn Alloc>
}

impl<T> Box<T> {
    pub fn new(value: T, alloc: Box<dyn Alloc>) -> Self {
        let ptr = alloc.alloc(mem::size_of::<T>()) as *mut T;
        unsafe {
            ptr::write(ptr, value);
        }
        Box { ptr, alloc }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr);
            self.alloc.dealloc(self.ptr as *mut u8, mem::size_of::<T>());
        }
    }
}
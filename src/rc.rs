use std::ptr;
use std::mem;
use std::alloc::Layout;
use super::Alloc;

pub struct Rc<T> {
    ptr: *mut T,
    strong: *mut usize,
    alloc: Box<dyn Alloc>,
}

impl<T> Rc<T> {
    pub fn new(value: T, alloc: Box<dyn Alloc>) -> Self {
        let ptr = alloc.alloc(mem::size_of::<T>()) as *mut T;
        unsafe {
            ptr::write(ptr, value);
        }
        let strong = alloc.alloc(mem::size_of::<usize>()) as *mut usize;
        unsafe {
            ptr::write(strong, 1);
        }
        Rc { ptr, strong, alloc }
    }

    pub fn clone(&self) -> Self {
        unsafe {
            let strong = *self.strong;
            *self.strong = strong + 1;
            Rc {
                ptr: self.ptr,
                strong: self.strong,
                alloc: self.alloc.clone(),
            }
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let strong = *self.strong - 1;
            if strong == 0 {
                ptr::drop_in_place(self.ptr);
                self.alloc.dealloc(self.ptr as *mut u8, mem::size_of::<T>());
                self.alloc.dealloc(self.strong as *mut u8, mem::size_of::<usize>());
            } else {
                *self.strong = strong;
            }
        }
    }
}

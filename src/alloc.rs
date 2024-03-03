trait Alloc {
    fn alloc(&self, size: usize) -> *mut u8;
    fn dealloc(&self, ptr: *mut u8, size: usize);
    fn realloc(&self, ptr: *mut u8, size: usize) -> *mut u8;
    fn alloc_zeroed(&self, size: usize) -> *mut u8;
}
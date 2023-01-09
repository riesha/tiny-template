#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(pointer_byte_offsets)]
extern crate alloc;
mod wrappers;
mod writer;
use core::{
    alloc::{GlobalAlloc, Layout},
    panic::PanicInfo,
};

use winapi::{
    ctypes::c_void,
    um::{
        heapapi::{GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc},
        winnt::HEAP_ZERO_MEMORY,
    },
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> !
{
    // zzzzzzzzzz mimimimimi

    loop
    {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> !
{
    // zzzzzzzzzz mimimimimi

    loop
    {}
}
pub struct HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator
{
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8
    {
        HeapAlloc(GetProcessHeap(), 0, _layout.size()) as *mut u8
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
    {
        HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout)
    {
        HeapFree(GetProcessHeap(), 0, _ptr as *mut c_void);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8
    {
        HeapReAlloc(
            GetProcessHeap(),
            HEAP_ZERO_MEMORY,
            ptr as *mut c_void,
            new_size,
        ) as *mut u8
    }
}

#[global_allocator]
static GLOBAL: HeapAllocator = HeapAllocator;

fn main()
{
    minicrt_println!("Hello world!");
}

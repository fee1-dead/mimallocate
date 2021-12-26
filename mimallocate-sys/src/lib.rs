// Derived from https://github.com/purpleprotocol/mimalloc_rust/blob/master/libmimalloc-sys/src/lib.rs
// and https://github.com/purpleprotocol/mimalloc_rust/blob/master/libmimalloc-sys/src/extended.rs

#![no_std]

use core::ffi::c_void;

#[allow(non_camel_case_types)]
pub type c_int = i32;

extern "C" {
    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory.
    ///
    /// All items are initialized to zero.
    pub fn mi_calloc(count: usize, size: usize) -> *mut c_void;

    /// Allocate `size` of bytes aligned by `alignment` and place the address of the
    /// allocated memory to `ptr`.
    ///
    /// Returns zero on success, invalid argument for invalid alignment, or out-of-memory.
    pub fn mi_posix_memalign(ptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;

    /// Allocate `size` bytes.
    ///
    /// Returns pointer to the allocated memory or null if out of memory.
    /// Returns a unique pointer if called with `size` 0.
    pub fn mi_malloc(size: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes.
    ///
    /// Return pointer to the allocated memory or null if out of memory. If null
    /// is returned, the pointer `p` is not freed. Otherwise the original
    /// pointer is either freed or returned as the reallocated result (in case
    /// it fits in-place with the new size).
    ///
    /// If `p` is null, it behaves as [`mi_malloc`]. If `newsize` is larger than
    /// the original `size` allocated for `p`, the bytes after `size` are
    /// uninitialized.
    pub fn mi_realloc(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Free previously allocated memory.
    ///
    /// The pointer `p` must have been allocated before (or be null).
    pub fn mi_free(p: *mut c_void);

    /// Override functions for macOS. This should not be called directly.
    #[cfg(target_os = "macos")]
    pub fn _mi_macos_override_malloc();
}
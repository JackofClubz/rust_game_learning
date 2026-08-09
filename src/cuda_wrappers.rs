//CUDA optimization wrappers for the functions in src/cuda_wrappers.rs

import { cudaMalloc, cudaMemcpy, cudaFree } from 'cuda';
use std::ptr;

pub fn cuda_malloc<T>(size: usize) -> *mut T {
    let mut ptr: *mut T = ptr::null_mut();
    unsafe {
        cudaMalloc(&mut ptr as *mut *mut T as *mut std::ffi::c_void, size);
    }
    ptr
}

pub fn cuda_free<T>(ptr: *mut T) {
    unsafe {
        cudaFree(ptr as *mut std::ffi::c_void);
    }
}

pub struct CudaBuffer<T> {
    ptr: *mut T,
    size: usize,
}

impl<T> CudaBuffer<T> {
    pub fn new(size: usize) -> Self {
        let ptr = cuda_malloc::<T>(size);
        CudaBuffer { ptr, size }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
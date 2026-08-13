//CUDA optimization wrappers for the functions in src/cuda_wrappers.rs

import { cudaMalloc, cudaMemcpy, cudaFree } from 'cuda';
use std::ptr;
use std::collections::HashMap;

import { CudaBuffer, cuda_malloc, cuda_free, copy_to_device, copy_from_device } from './cuda_wrappers';

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

pub fn copy_to_device<T>(host_data: &[T], device_buffer: &mut CudaBuffer<T>) {
    unsafe {
        cudaMemcpy(
            device_buffer.as_ptr() as *mut std::ffi::c_void,
            host_data.as_ptr() as *const std::ffi::c_void,
            host_data.len() * std::mem::size_of::<T>(),
            cudaMemcpyKind::HostToDevice,
        );
    }
}

//memory coalescing and shared memory optimization can be implemented in the CUDA kernels themselves, which are not shown here. The above wrappers provide a convenient way to allocate and manage device memory from Rust code.
pub fn copy_from_device<T>(device_buffer: &CudaBuffer<T>, host_data: &mut [T]) {
    unsafe {
        cudaMemcpy(
            host_data.as_mut_ptr() as *mut std::ffi::c_void,
            device_buffer.as_ptr() as *const std::ffi::c_void,
            host_data.len() * std::mem::size_of::<T>(),
            cudaMemcpyKind::DeviceToHost,
        );
    }
}

impl<T> Drop for CudaBuffer<T> {
    fn drop(&mut self) {
        cuda_free(self.ptr);
    }
}
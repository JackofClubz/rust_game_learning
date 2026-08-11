// cache for CUDA memory management

use std::collections::HashMap;
use std::ptr;
use crate::cuda_wrappers::{CudaBuffer, cuda_malloc, cuda_free, copy_to_device, copy_from_device};   

pub struct InMemoryCache<T> {
    cache: HashMap<String, CudaBuffer<T>>,
}
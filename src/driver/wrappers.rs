use crate::driver::sys::{CUcontext, CUfunction};
use libc::c_void;

use super::sys::{CUdevice, CUdeviceptr, CUevent, CUkernel, CUlibrary, CUmodule, CUstream};

macro_rules! impl_wrapper {
    ($wrapper: ident, $type: ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $wrapper(pub $type);
        unsafe impl Send for $wrapper {}
        unsafe impl Sync for $wrapper {}
    };
}

impl_wrapper!(Void, *const c_void);
impl_wrapper!(WrCUcontext, CUcontext);
impl_wrapper!(WrCUfunction, CUfunction);
impl_wrapper!(WrCUmodule, CUmodule);
impl_wrapper!(WrCUstream, CUstream);
impl_wrapper!(WrCUdevice, CUdevice);
impl_wrapper!(WrClibrary, CUlibrary);
impl_wrapper!(WrCUkernel, CUkernel);
impl_wrapper!(WrCUdeviceptr, CUdeviceptr);
impl_wrapper!(WrCUevent, CUevent);

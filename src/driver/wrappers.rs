use crate::driver::sys::{CUcontext, CUfunction};
use libc::c_void;

use super::sys::{CUdevice, CUdeviceptr, CUevent, CUkernel, CUlibrary, CUmodule, CUstream};

macro_rules! sync_send {
    ($wrapper: ident, $type: ident) => {
        #[derive(Debug)]
        pub struct $wrapper(pub $type);
        unsafe impl Send for $wrapper {}
        unsafe impl Sync for $wrapper {}
    };
}

sync_send!(Void, c_void);
sync_send!(WrCUcontext, CUcontext);
sync_send!(WrCUfunction, CUfunction);
sync_send!(WrCUmodule, CUmodule);
sync_send!(WrCUstream, CUstream);
sync_send!(WrCUdevice, CUdevice);
sync_send!(WrClibrary, CUlibrary);
sync_send!(WrCUkernel, CUkernel);
sync_send!(WrCUdeviceptr, CUdeviceptr);
sync_send!(WrCUevent, CUevent);

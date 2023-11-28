use super::sys::cudaError_t;
use crate::driver::sys::CUresult;

impl From<cudaError_t> for CUresult {
    fn from(err: cudaError_t) -> Self {
        match err {
            cudaError_t::cudaErrorApiFailureBase => {
                panic!("cudaErrorApiFailureBase variant does not have a corresponding CUresult")
            }
            err => unsafe { std::mem::transmute(err) },
        }
    }
}

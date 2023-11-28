pub mod result;
#[allow(warnings)]
pub mod sys;

#[cfg(test)]
mod tests {
    use crate::{
        driver::sys::CUresult,
        runtime::sys::{cudaDriverGetVersion, cudaError, cudaGetErrorString},
    };

    // This should work without GPU
    #[test]
    fn get_version() {
        let mut version: i32 = 0;
        let result = unsafe { cudaDriverGetVersion(&mut version as *mut i32) };
        if result != cudaError::cudaSuccess {
            panic!("Cannot get driver version: ERROR={:?}", result);
        }
        println!("Version = {}", version);
    }

    #[test]
    fn get_err_string() {
        let err_str = unsafe { cudaGetErrorString(cudaError::cudaErrorInvalidValue) };
        let err_str = unsafe { std::ffi::CStr::from_ptr(err_str) };
        println!("Error String = {:?}", err_str);
    }

    #[test]
    fn into_result() {
        let result = unsafe {
            let mut version = 0;
            cudaDriverGetVersion(&mut version as _)
        };
        println!("cudaError = {:?}", result);
        let result: CUresult = result.into();
        println!("CUresult = {:?}", result);
        assert!(result.result().is_ok());
    }
}

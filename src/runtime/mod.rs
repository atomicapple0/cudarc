pub mod result;
#[allow(warnings)]
pub mod sys;

#[cfg(test)]
mod tests {
    use crate::{
        driver::sys::CUresult,
        runtime::sys::{cudaDriverGetVersion, cudaError},
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
    fn into_result() {
        let result = unsafe {
            let mut version = 0;
            cudaDriverGetVersion(&mut version as _)
        };
        let result: CUresult = result.into();
        assert!(result.result().is_ok());
    }
}

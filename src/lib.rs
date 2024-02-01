use std::ffi::{c_int, c_char, CString};

pub fn run(params: &str, problem: &str) -> i32 {
    let params = CString::new(params).unwrap();
    let problem = CString::new(problem).unwrap();

    unsafe {
        lkh_run(params.as_ptr(), problem.as_ptr()).into()
    }
}

extern "C" {
    fn lkh_run(params: *const c_char, problem: *const c_char) -> c_int;
}
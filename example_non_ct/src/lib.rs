use sidefuzz::black_box;
use std::slice;

#[no_mangle]
pub extern "C" fn len() -> i32 {
    return 28;
}

#[no_mangle]
pub extern "C" fn sidefuzz(ptr: i32, len: i32) {
    let input: &[u8] = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    black_box(not_constant_time_comparison(input));
}

fn not_constant_time_comparison(message: &[u8]) -> bool {
    message == b"NOT CONSTANT TIME COMPARISON"
}

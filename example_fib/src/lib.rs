use sidefuzz::black_box;
use std::slice;

#[no_mangle]
pub extern "C" fn len() -> i32 {
    return 1;
}

#[no_mangle]
pub extern "C" fn sidefuzz(ptr: i32, len: i32) {
    let input: &[u8] = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    black_box(fibonacci(input[0]));
}

pub fn fibonacci(n: u8) -> f64 {
    if n == 0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == 1 {
        return 1.0;
    }

    let mut sum = 0.0;
    let mut last = 0.0;
    let mut curr = 1.0;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

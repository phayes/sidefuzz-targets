use sidefuzz::black_box;
use std::slice;
use subtle::ConstantTimeEq;

#[no_mangle]
pub extern "C" fn len() -> i32 {
  return 24;
}

#[no_mangle]
pub extern "C" fn sidefuzz(ptr: i32, len: i32) {
  let input: &[u8] = unsafe { slice::from_raw_parts(ptr as _, len as _) };
  black_box(input.ct_eq(b"CONSTANT TIME COMPARISON"));
}

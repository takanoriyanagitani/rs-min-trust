use std::os::raw::c_void;

const PAGE_SIZE: usize = 65536;

const FULL_PAGE: usize = PAGE_SIZE;
const HALF_PAGE: usize = PAGE_SIZE >> 1;

static mut IN_BUF64K: [u8; FULL_PAGE] = [0; FULL_PAGE];
static mut OUT_BUF32K: [u8; HALF_PAGE] = [0; HALF_PAGE];

#[no_mangle]
pub extern "C" fn in_buf64k() -> *mut c_void {
    unsafe { IN_BUF64K.as_mut_ptr() as *mut c_void }
}

#[no_mangle]
pub extern "C" fn out_buf32k() -> *mut c_void {
    unsafe { OUT_BUF32K.as_mut_ptr() as *mut c_void }
}

fn _bitwise_or32k(i1: *const u8, i2: *const u8, o: *mut u8) -> Result<i64, ()> {
    let s1: &[u8] = unsafe { std::slice::from_raw_parts(i1, HALF_PAGE) };
    let s2: &[u8] = unsafe { std::slice::from_raw_parts(i2, HALF_PAGE) };
    let o: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(o, HALF_PAGE) };
    for i in 0..HALF_PAGE {
        let j1: u8 = s1[i];
        let j2: u8 = s2[i];
        o[i] = j1 | j2;
    }
    Ok(HALF_PAGE as i64)
}

#[no_mangle]
pub extern "C" fn bitwise_or32k(i1: *const c_void, i2: *const c_void, o: *mut c_void) -> i64 {
    _bitwise_or32k(i1 as *const u8, i2 as *const u8, o as *mut u8).unwrap_or(-1)
}

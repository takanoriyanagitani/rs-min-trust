use std::os::raw::c_void;

static mut IN_BUF128K: [u8; 131072] = [0; 131072];
static mut OUT_BUF64K: [u8; 65536] = [0; 65536];

#[no_mangle]
pub extern "C" fn in_buf128k() -> *mut c_void {
    unsafe { IN_BUF128K.as_mut_ptr() as *mut c_void }
}

#[no_mangle]
pub extern "C" fn out_buf64k() -> *mut c_void {
    unsafe { OUT_BUF64K.as_mut_ptr() as *mut c_void }
}

fn _bitwise_or64k(i1: *const u8, i2: *const u8, o: *mut u8) -> Result<i64, ()> {
    let s1: &[u8] = unsafe { std::slice::from_raw_parts(i1, 65536) };
    let s2: &[u8] = unsafe { std::slice::from_raw_parts(i2, 65536) };
    let o: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(o, 65536) };
    for i in 0..65536 {
        let j1: u8 = s1[i];
        let j2: u8 = s2[i];
        o[i] = j1 | j2;
    }
    Ok(65536)
}

#[no_mangle]
pub extern "C" fn bitwise_or64k(i1: *const c_void, i2: *const c_void, o: *mut c_void) -> i64 {
    _bitwise_or64k(i1 as *const u8, i2 as *const u8, o as *mut u8).unwrap_or(-1)
}

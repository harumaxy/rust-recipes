// mangle = 噛み砕く、潰す、こねくり回す
#[no_mangle]
pub extern "C" fn add_double(first: i32, second: i32) -> i32 {
    first + second * 2
}

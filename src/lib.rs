use libc::size_t;
use rocksdb::DB;
use std::convert::TryInto;
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn store_byte_array(
    key_array_pointer: *const c_char,
    key_size: size_t,
    value_array_pointer: *const c_char,
    value_size: size_t,
) {
    let key = unsafe {
        assert!(!key_array_pointer.is_null());
        slice::from_raw_parts(key_array_pointer as *const _, key_size as usize)
    };
    let value = unsafe {
        assert!(!value_array_pointer.is_null());
        slice::from_raw_parts(value_array_pointer as *const _, value_size as usize)
    };
    let db_path = "/media/nvme/ssvm_database";
    let db = DB::open_default(db_path).unwrap();
    db.put(key, value).unwrap();
}

#[no_mangle]
pub extern "C" fn get_byte_array_pointer(
    key_array_pointer: *const c_char,
    key_size: size_t,
) -> *mut c_char {
    let key = unsafe {
        assert!(!key_array_pointer.is_null());
        slice::from_raw_parts(key_array_pointer as *const _, key_size as usize)
    };
    let db_path = "/media/nvme/ssvm_database";
    let db = DB::open_default(db_path).unwrap();
    let loaded_data = db.get(&key).unwrap().unwrap();
    let data = loaded_data.as_ptr() as *mut _;
    std::mem::forget(loaded_data);
    data
}

#[no_mangle]
pub extern "C" fn get_byte_array_length(
    key_array_pointer: *const c_char,
    key_size: size_t,
) -> size_t {
    let key = unsafe {
        assert!(!key_array_pointer.is_null());
        slice::from_raw_parts(key_array_pointer as *const _, key_size as usize)
    };
    let db_path = "/media/nvme/ssvm_database";
    let db = DB::open_default(db_path).unwrap();
    let loaded_data = db.get(&key).unwrap();
    let size: size_t = loaded_data.unwrap().len().try_into().unwrap();
    size
}

#[no_mangle]
pub extern "C" fn free_byte_array_pointer(value_array_pointer: *mut c_char, value_size: size_t) {
    let s = unsafe { std::slice::from_raw_parts_mut(value_array_pointer, value_size) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}

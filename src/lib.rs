extern crate cc;
extern crate libc;
use libc::{c_char, uint32_t, size_t, uintptr_t};
use rocksdb::{Options, DB, DBPinnableSlice};
use std::ffi::{CStr, CString};
use std::slice;

#[no_mangle]
pub extern "C" fn store_byte_array(_key_array_pointer: *const uintptr_t, _key_size: size_t, _value_array_pointer: *const uintptr_t, _value_size: size_t) {
    let _key = unsafe {
        assert!(!_key_array_pointer.is_null());

        std::slice::from_raw_parts(_key_array_pointer as *const u32, _key_size as usize;
    };
    let _value = unsafe {
        assert!(!_value_array_pointer.is_null());

        std::slice::from_raw_parts(_value_array_pointer as *const u32, _value_size as usize;

    };
    println!("Storing data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    db.put(_key, _value).unwrap();
    println!("Item added to database");
}

#[no_mangle]
pub extern "C" fn load_byte_array(_key_array_pointer: *const uintptr_t, _key_size: size_t) -> *mut uintptr_t {
    let _key = unsafe {
        assert!(!_key.is_null());

        std::slice::from_raw_parts(_key_array_pointer as *const u32, _key_size as usize;
    };
    println!("Loading data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    let loaded_data = db.get(_key).unwrap();
    println!("Loaded data: {:?}", loaded_data);
    let ptr: *const uintptr_t = loaded_data.as_ptr();
    println!("Pointer: {:?}", ptr);
    let size: size_t = loaded_data.len();
    println!("Size: {:?}", size);
    ptr
}

#[no_mangle]
pub extern "C" fn free_byte_array_pointer(s: *const uintptr_t) {
    unsafe {
        if s.is_null() {
            return;
        }
        // TODO find best way to deallocate pointer now that we are not using CStr but raw pointer
        CString::from_raw(s)
    };
}


/*
#[no_mangle]
pub extern "C" fn store_bytes(_key: *const c_char, _value: *const c_char) {
    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    let c_str_value = unsafe {
        assert!(!_value.is_null());

        CStr::from_ptr(_value)
    };
    println!("Storing data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    db.put(c_str_key.to_bytes(), c_str_value.to_bytes())
        .unwrap();
    println!("Item added to database");
}

#[no_mangle]
pub extern "C" fn load_bytes(_key: *const c_char) -> *mut c_char {
    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    println!("Loading data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    CString::new(db.get(c_str_key.to_bytes()).unwrap().unwrap())
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn store_data(_key: *const c_char, _value: *const c_char) {
    println!("Storing data, please wait ...");
    let c_str = unsafe {
        assert!(!_value.is_null());

        CStr::from_ptr(_value)
    };
    let _value_as_string = c_str.to_str().unwrap();

    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    let _key_as_string = c_str_key.to_str().unwrap();
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    let mut opts = Options::default();
    opts.increase_parallelism(3);
    opts.create_if_missing(true);
    //println!("Database options are set");
    db.put(_key_as_string, _value_as_string).unwrap();
    println!("Item added to database");
}

#[no_mangle]
pub extern "C" fn load_data(_key: *const c_char) -> *mut c_char {
    println!("Loading data, please wait ...");
    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    let _key_as_string = c_str_key.to_str().unwrap();
    let path = "/media/nvme/ssvm_database";
    //println!("Database path: {:?}", path);*
    let db = DB::open_default(path).unwrap();
    //println!("Database instance: {:?}", db);
    let mut opts = Options::default();
    opts.increase_parallelism(3);
    //println!("Database options are set");
    let db_value_as_vec = db.get(_key_as_string).unwrap().unwrap();
    let db_value_as_cstring = CString::new(db_value_as_vec).unwrap();
    //println!("Value as CString: {:?}", db_value_as_cstring);
    db_value_as_cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn free_pointer(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
*/

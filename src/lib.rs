extern crate cc;
extern crate libc;
use libc::c_char;
use rocksdb::{Options, DB};
use std::ffi::{CStr, CString};
use std::slice;

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

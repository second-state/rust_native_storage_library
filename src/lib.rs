extern crate libc;

use libc::c_char;
use rocksdb::{Options, DB};
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn store_data(_key: i64, _value: *const c_char) {
    let c_str = unsafe {
        assert!(!_value.is_null());

        CStr::from_ptr(_value)
    };
    let _value_as_string = c_str.to_str().unwrap();
    println!("value_as_string: {:?}", _value_as_string);
    println!("Storing data");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    let mut opts = Options::default();
    opts.increase_parallelism(3);
    opts.create_if_missing(true);
    println!("Database options are set");
    db.put(_key.to_string(), _value_as_string).unwrap();
    println!("Item added to database");
}

#[no_mangle]
pub extern "C" fn load_data(_key: i64) -> *mut c_char {
    println!("Loading data");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    let mut opts = Options::default();
    opts.increase_parallelism(3);
    println!("Database options are set");
    let db_value_as_vec = db.get(_key.to_string()).unwrap().unwrap();
    let db_value_as_cstring = CString::new(db_value_as_vec).unwrap();
    println!("Value as CString: {:?}", db_value_as_cstring);
    db_value_as_cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

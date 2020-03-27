extern crate libc;

use rocksdb::{DB, Options};
use std::ffi::{CString, CStr};
use libc::c_char;


#[no_mangle]
pub extern "C" fn store_data(_key: i64, _value: *const c_char) {
	//let _value_as_cstring = CStr::from_bytes_with_nul(b"foo\0").expect("CStr::from_bytes_with_nul failed");
	
	println!("Value as const char: {:?}", _value);
    let _value_as_cstring = unsafe {
	    assert!(!_value.is_null());
	    CStr::from_ptr(_value)
    }; 
    
    println!("Value as CString: {:?}", _value_as_cstring);
    let _value_as_string = _value_as_cstring.to_str().unwrap();
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
// Returns C-compatible, nul-terminated string with no nul bytes in the middle
// https://doc.rust-lang.org/stable/std/ffi/struct.CString.html#method.into_raw
#[no_mangle]
pub extern "C" fn load_data(_key: i64) {
	println!("Loading data");
	let path = "/media/nvme/ssvm_database";
	println!("Database path: {:?}", path);
	let db = DB::open_default(path).unwrap();
	println!("Database instance: {:?}", db);
	let mut opts = Options::default();
	opts.increase_parallelism(3);
	println!("Database options are set");
    match db.get(_key.to_string()) {
        Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e), 
   }

    //let db_value_as_vec = db.get(_key.to_string());
    //let db_value_as_string = String::from_utf8(db_value_as_vec.unwrap().unwrap());
    //println!("DB Value as String: {:?}", db_value_as_string);
	//let db_value_as_cstring = CString::new(db_value_as_vec);
	//db_value_as_cstring
}


extern crate libc;

use rocksdb::{DB, Options};
use std::ffi::{CString, CStr};
use libc::c_char;


#[no_mangle]
pub extern "C" fn store_data(_key: i64, _value: *const c_char) {
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
}
// Returns C-compatible, nul-terminated string with no nul bytes in the middle
#[no_mangle]
pub extern "C" fn load_data(_key: i64) -> CString {
	println!("Loading data");
	let path = "/media/nvme/ssvm_database";
	let db = DB::open_default(path).unwrap();
	let mut opts = Options::default();
	opts.increase_parallelism(3);	
    let db_value_as_vec = db.get(_key.to_string()).unwrap().unwrap();
    println!("DB Value as Vec: {:?}", db_value_as_vec);
	let db_value_as_cstring = CString::new(db_value_as_vec).expect("CString::new failed");
	db_value_as_cstring
}
use rocksdb::{DB, Options};
use std::ffi::{CString};

#[no_mangle]
pub extern "C" fn store_data(_key: i64, _value: CString) {
	println!("Storing data");
	let path = "/media/nvme/ssvm_database";
	println!("Database path: {:?}", path);
	let db = DB::open_default(path).unwrap();
	println!("Database instance: {:?}", db);
	let mut opts = Options::default();
	opts.increase_parallelism(3);
    opts.create_if_missing(true);
    println!("Database options are set");
    println!("Processing CString value: {:?}", _value);
    let value_as_string = _value.into_string().expect("into_string() call failed");
    db.put(_key.to_string(), value_as_string).unwrap();
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
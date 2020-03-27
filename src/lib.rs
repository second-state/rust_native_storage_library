use rocksdb::{DB, Options};
use std::ffi::{CString};

#[no_mangle]
pub extern "C" fn store_data(_key: &i64, _value: CString) {
	let path = "/media/nvme/ssvm_database";
	let db = DB::open_default(path).unwrap();
	let mut opts = Options::default();
	opts.increase_parallelism(3);
    opts.create_if_missing(true);
    db.put(_key, _value).unwrap();
}
// Returns C-compatible, nul-terminated string with no nul bytes in the middle
#[no_mangle]
pub extern "C" fn load_data(_key: &i64) -> CString {
	let path = "/media/nvme/ssvm_database";
	let db = DB::open_default(path).unwrap();
	let mut opts = Options::default();
	opts.increase_parallelism(3);	
    let db_value_as_vec = db.get(_key.to_string()).unwrap().unwrap();
    println!("DB Value as Vec: {:?}", db_value_as_vec);
	let db_value_as_cstring = CString::new(db_value_as_vec).expect("CString::new failed");
	db_value_as_cstring
}
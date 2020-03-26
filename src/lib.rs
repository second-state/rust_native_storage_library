use rocksdb::{DB, Options};

#[no_mangle]
pub extern fn store_data(_key: String, _value: Vec<u8>) {
	let path = "rocksdb_storage";
	let db = DB::open_default(path).unwrap();
	let mut opts = Options::default();
	opts.increase_parallelism(3);
    opts.create_if_missing(true);
	DB::put(&db, _key, _value).unwrap();
}

#[no_mangle]
pub extern fn load_data(_key: String) -> Vec<u8> {
	let path = "rocksdb_storage";
	let db = DB::open_default(path).unwrap();
	let mut opts = Options::default();
	opts.increase_parallelism(3);
	db.get(_key).unwrap().unwrap()
}
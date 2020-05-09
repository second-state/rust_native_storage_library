use libc::size_t;
use rocksdb::DB;
use std::alloc::{dealloc, Layout};
use std::convert::TryInto;
use std::os::raw::c_char;
use std::ptr;
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
    let loaded_data = db.get(&key).unwrap();
    let ptr1 = loaded_data.as_ref().unwrap().as_ptr() as *mut _;
    let a_box = unsafe { Box::from_raw(ptr1) };
    Box::into_raw(a_box)
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
pub extern "C" fn free_byte_array_pointer(s: *mut c_char) {
    let new_box = unsafe {
        assert!(!s.is_null());
        Box::from_raw(s)
    };
    /*
    let p = Box::into_raw(new_box);
    unsafe {
        ptr::drop_in_place(p);
        dealloc(p as *mut u8, Layout::for_value(&*p));
    };
    */
}

// The code below worked really well
// i.e. store/load (using CStr as string) and store/load (using CStr as bytes)
// However these original functions will be excluded from the API
// Reason being any use of CStr means that the data is not allowed to have \0 \0x00 nul
// We need to accomodate these values as valid input. The DB will put and get arbitrary byte arrays so we need to ensure that this API does not offer less than that
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

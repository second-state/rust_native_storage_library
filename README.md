# Rust Native Storage Library

## Fetch
```
cd ~
git clone https://github.com/second-state/rust_native_storage_library.git
```
## Build
```
cd ~/rust_native_storage_library
make
```
## Output
Creates an executable dynamic library at the following location
```
~/rust_native_storage_library/target/debug/librust_native_storage_library.so
```
Compiles and links the C++ code (`main.cpp`), the header file `test_lib.h` and the Rust code `lib.rs` and provides an executable called `run_test_demonstration`
## Run example C++ implementation (test demonstration)
```
./run_test_demonstration
```
This test demonstration produces the following output (stores and loads from RocksDB)
```
Starting main function ...
i64 variable as integer: 1234567111
i64 variable as string: 1234567111
i64 variable as char: 1234567111
i32 variable as integer: 1234567111
i32 variable as string: 1234567111
i32 variable as char: 1234567111
Calling store data ... 
Storing data, please wait ...
Database path: "/media/nvme/ssvm_database"
Database instance: RocksDB { path: "/media/nvme/ssvm_database" }
Item added to database
Calling load data ... 1234567111
Loading data, please wait ...
Retrieved the following string: 1234567111
```

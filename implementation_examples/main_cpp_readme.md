
# Implementation in C++

## Compiling the Rust source code first

We first need to compile the Rust source code so that we have a dynamic library available somewhere on our file system. We add the location of the compiled Rust (`.so`, `.dylib` etc. depending on the OS) to the C++ compile command.

Perform this command to get the target
```
rustup target add x86_64-unknown-linux-gnu
```

Add the target/linker config to your ~/.cargo/conf file like this
```
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"
```
The Rust compilation looks like this
```
cargo build --release --target x86_64-unknown-linux-gnu
```
## C++ source code
```
#include <string>
#include <iostream>

extern std::string
load_data(int64_t);

int main(void) {
  std::string loaded_string = load_data(1234567890);
  std::cout << loaded_string; 
}
```
## Compilation and linking
```
gcc --std=c11 -o c-example implementation_examples/main.c -L target/release/ -librust_native_storage_library.dylib
```
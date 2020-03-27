
# Implementation in C++

## System requirements
Install Rust
```
sudo apt-get update
sudo apt-get -y upgrade
sudo apt-get -y install make clang pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup override set nightly

```

## Fetch the Rust source code
```
git clone https://github.com/second-state/rust_native_storage_library.git
cd rust_native_storage_library/
```

## Compiling the Rust source code first

We first need to compile the Rust source code so that we have a dynamic library available somewhere on our file system. We add the location of the compiled Rust (`.so`, `.dylib` etc. depending on the OS) to the C++ compile command.

## Optional START
Perform this command to get the target
```
rustup target add x86_64-unknown-linux-gnu
```

Add the target/linker config to your `~/.cargo/conf` file like this
```
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"
```
The Rust compilation looks like this
```
cargo build --release --target x86_64-unknown-linux-gnu
```
## Optional END

The Rust compilation looks like this
```
cargo build --release
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
g++ -c -o cpp-example implementation_examples/main.cpp
g++ cpp-example target/x86_64-unknown-linux-gnu/release/librust_native_storage_library.so
```
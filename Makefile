ifeq ($(shell uname),Darwin)
	EXT := dylib
else
	EXT := so
endif
all: target/debug/librust_native_storage_library.$(EXT)
	g++ src/main.cpp -L ./target/debug/ -lrust_native_storage_library -o run
	LD_LIBRARY_PATH=./target/debug/ ./run

target/debug/librust_native_storage_library.$(EXT): src/lib.rs Cargo.toml
	cargo build --verbose

clean:
	cargo clean
	cargo update
	rm -rf target
	rm -rf run
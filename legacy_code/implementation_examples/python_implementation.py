import sys, ctypes
from ctypes import c_void_p, c_uint8, c_longlong, c_char_p

lib = ctypes.cdll.LoadLibrary("target/x86_64-unknown-linux-gnu/release/librust_native_storage_library.so")

# Pertains to load data function
lib.load_data.argtypes = (c_char_p, )
lib.load_data.restype = c_void_p
lib.free.argtypes = (c_void_p, )
lib.free.restype = None

# Pertains to store data function
lib.store_data.argtypes = (c_char_p, c_char_p, )
lib.store_data.restype = None

# Call load data function and then free the pointer
def load_data_and_free_pointer(key):
    ptr = lib.load_data(key.encode('utf-8'))
    try:
        print(ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8'))
    # Use this finally syntax to call the "free" function; ensure that the pointer is always freed the correct way
    finally:
        lib.free(ptr)

def store_data(key, a_string):
    lib.store_data(key.encode('utf-8'), a_string.encode('utf-8'))

# Call functions
store_data("1111111111", "This is a string stored in relation to the key 1111111111")
store_data("2222222222", "This is a string stored in relation to the key 2222222222")
load_data_and_free_pointer("1111111111")
load_data_and_free_pointer("2222222222")


#ifndef _TESTLIB_H
#define _TESTLIB_H
 
#ifdef __cplusplus
extern "C"{
#endif

// Original store/load (using CStr as string)
//void store_data(const char *key, const char *value);
//char * load_data(const char *key);

// Original store/load (using CStr as bytes)
//void store_bytes(const char *key, const char *value);
//char * load_bytes(const char *key);

// Original free pointer for the above functions
//void free_pointer(char *);

// New store/load (using byte array pointer and length)
void store_byte_array(const char *_key_array_pointer, uint32_t _key_size, const char *_value_array_pointer, uint32_t _value_size);
char * load_byte_array(const char *_key_array_pointer, uint32_t _key_size);

// New free pointer for the byte array approach
void free_byte_array_pointer(char *);

#ifdef __cplusplus
}
#endif
#endif
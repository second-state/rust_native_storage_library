#ifndef _TESTLIB_H
#define _TESTLIB_H
 
#ifdef __cplusplus
extern "C"{
#endif

void store_data(const char *key, const char *value);
char * load_data(const char *key);
void store_bytes(const char *key, const char *value);
char * load_bytes(const char *key);
void free_pointer(char *);

void store_byte_array(const uint32_t *_key_array_pointer, size_t _key_size, const uint32_t *_value_array_pointer, size_t _value_size);

#ifdef __cplusplus
}
#endif
#endif
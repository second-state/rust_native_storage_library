#ifndef _TESTLIB_H
#define _TESTLIB_H
 
#ifdef __cplusplus
extern "C"{
#endif

void store_data(const char *key, const char *value);
char * load_data(const char *key);
void free_pointer(char *);

void store_bytes(const char *key, const char *buffer_address, const size_t *length);
char * load_bytes(const char *key);
void free_byte_pointer(char *);

#ifdef __cplusplus
}
#endif
#endif
#ifndef _TESTLIB_H
#define _TESTLIB_H
 
#ifdef __cplusplus
extern "C"{
#endif

void store_data(const char *key, const char *value);
char * load_data(const char *key);
void free_pointer(char *);

#ifdef __cplusplus
}
#endif
#endif
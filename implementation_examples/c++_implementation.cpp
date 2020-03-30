#include <stdio.h>
#include <iostream>
#include <string>
using namespace std;

extern "C" void 
store_data(const char *key, const char *value);

int main()
{
	// i64 equivalent     
    signed long long var64bit = 1234567890;
    cout << sizeof(var64bit) << endl << var64bit << endl;
    string var64bitString = to_string(var64bit);
    char const *str64 = var64bitString.data();

    // i32 equivalent
    signed int var32bit = 1234567891;
    cout << sizeof(var32bit) << endl << var32bit << endl;    
    string var32bitString = to_string(var32bit);
    char const *str32 = var32bitString.data();

    store_data(str64, str32);

    return 0;
}
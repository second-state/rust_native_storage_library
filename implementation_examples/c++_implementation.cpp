#include <stdio.h>
#include <iostream>
#include <string>
using namespace std;

extern "C" void 
store_data(const char *key, const char *value);

int main()
{
	// Setting up example key and value

	// i64 equivalent     
    signed long long var64bit = 1234567890;
    cout << "i64 variable as integer: " << var64bit << endl;
    string var64bitString = to_string(var64bit);
    cout << "i64 variable as string: " << var64bitString << endl;
    char const *str64 = var64bitString.data();
    cout << "i64 variable as char: " << str64 << endl;

    // i32 equivalent
    signed int var32bit = 1234567891;
    cout << "i32 variable as integer: " << var32bit << endl;
    string var32bitString = to_string(var32bit);
    cout << "i32 variable as string: " << var32bitString << endl;
    char const *str32 = var32bitString.data();
    cout << "i32 variable as char: " << str32 << endl;
    
    // Calling store data
    cout << "Calling store data ... " << var32bit << endl;
    store_data(str64, str32);
    cout << "Data storage complete." << var32bit << endl;
    return 0;
}
#include <stdio.h>
#include <iostream>
#include <string>
#include <vector>
#include <cstring>
#include "test_lib.h"
using namespace std;

int main()
{
    // Setting up example key and value
    cout << "Starting main function ..." << endl;
    // i64 equivalent     
    signed long long var64bit = 1234567111;
    cout << "i64 variable as integer: " << var64bit << endl;
    string var64bitString = to_string(var64bit);
    cout << "i64 variable as string: " << var64bitString << endl;
    char const *str64 = var64bitString.data();
    cout << "i64 variable as char: " << str64 << endl;

    // i32 equivalent
    signed int var32bit = 1234567111;
    cout << "i32 variable as integer: " << var32bit << endl;
    string var32bitString = to_string(var32bit);
    cout << "i32 variable as string: " << var32bitString << endl;
    char const *str32 = var32bitString.data();
    cout << "i32 variable as char: " << str32 << endl;

    // Calling store data
    cout << "Calling store data ... " << endl;
    store_data(str64, str32);
    cout << "Calling load data ... " << var32bit << endl;
    char *loaded_pointer = load_data(str64);
    string loaded_string = loaded_pointer;
    cout << "Retrieved the following string: " << loaded_string << endl;
    free_pointer(loaded_pointer);

    
    return 0;
}
#include <stdio.h>
#include <iostream>
#include <string>
#include <vector>
#include <cstdint>
#include <cstring>
#include <algorithm>
#include "test_lib.h"
using namespace std;

int main()
{
    // Store byte array
    std::vector<uint8_t> Key = {0x03, 0x02, 0x01, 0x00, 0xFF, 0x00};
    std::vector<uint8_t> Val = {0xFF, 0xFE, 0x00, 0x01, 0x02, 0x00, 0x03, 0x04};
    store_byte_array(reinterpret_cast<char *>(&(Key[0])), Key.size(), reinterpret_cast<char *>(&(Val[0])), Val.size());
    
    // Get byte array length
    std::vector<uint8_t> Key2 = {0x03, 0x02, 0x01, 0x00, 0xFF, 0x00};
    std::vector<uint8_t> Val2;
    uint32_t Len = get_byte_array_length(reinterpret_cast<char *>(&(Key2[0])), Key2.size());

    // Get the byte array pointer
    char *Ptr = get_byte_array_pointer(reinterpret_cast<char *>(&(Key2[0])), Key2.size());
    std::copy_n(Ptr, Len, std::back_inserter(Val2));

    // Free the pointer
    free_byte_array_pointer(Ptr);

    //  Check actual values
    std::vector<uint8_t> Key3 = {0x03, 0x02, 0x01, 0x00, 0xFF, 0x00};
    for (auto &Val : Key3) {
        printf("0x%02x ", Val3);
    }

    // The code below worked really well
    // i.e. store/load (using CStr as string) and store/load (using CStr as bytes)
    // However these original functions will be excluded from the API
    // Reason being any use of CStr means that the data is not allowed to have \0 \0x00 nul
    // We need to accomodate these values as valid input. The DB will put and get arbitrary byte arrays so we need to ensure that this API does not offer less than that

    /*
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

    // Calling store bytes
    signed long long var64bit_2 = 1111111111;
    cout << "i64 variable as integer: " << var64bit_2 << endl;
    string var64bitString_2 = to_string(var64bit_2);
    cout << "i64 variable as string: " << var64bitString_2 << endl;
    char const *str64_2 = var64bitString_2.data();
    cout << "i64 variable as char: " << str64_2 << endl;
    // i32 equivalent
    signed int var32bit_2 = 1111111112;
    cout << "i32 variable as integer: " << var32bit_2 << endl;
    string var32bitString_2 = to_string(var32bit_2);
    cout << "i32 variable as string: " << var32bitString_2 << endl;
    char const *str32_2 = var32bitString_2.data();
    cout << "i32 variable as char: " << str32_2 << endl;
    
    // Calling store data
    cout << "Calling store bytes ... " << endl;
    store_bytes(str64_2, str32_2);
    cout << "Calling load bytes ... " << var32bit_2 << endl;
    char *loaded_pointer_2 = load_bytes(str64_2);
    string loaded_string_2 = loaded_pointer_2;
    cout << "Retrieved the following data: " << loaded_string_2 << endl;
    free_pointer(loaded_pointer_2);
    */
    
    return 0;
}
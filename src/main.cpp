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

    // Print the pointer
    printf("Ptr    = %p\n", (void *) Ptr);

    // Print the values  
    printf("Values available in C++\n");   
    for (auto &PtrVal : Val2) {
        printf("0x%02x ", PtrVal);
    }
    // Free the pointer
    free_byte_array_pointer(Ptr);

    printf("\n");
    
    return 0;
}
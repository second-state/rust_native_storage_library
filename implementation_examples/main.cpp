#include <string>
#include <iostream>

extern "C" void store_data(std::int64_t, std::string);
extern "C" std::string load_data(std::int64_t);

int main(void) {
	std::int64_t key = 1234567890;
	std::string string_to_store = "This is a string!";
    store_data(key, string_to_store);
    std::string loaded_string = load_data(key);
    std::cout << loaded_string; 
}

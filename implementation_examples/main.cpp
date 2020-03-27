#include <string>
#include <cstring>
#include <iostream>

extern "C" void store_data(std::int64_t, std::string);
// extern "C" std::string load_data(std::int64_t);
extern "C" void load_data(std::int64_t);


int main(void) {
	std::int64_t key = 1234567890;
	std::cout << "Key:";
	std::cout << key;
	std::string string_to_store = "aaaa";
	std::cout << "String to store:";
	std::cout << string_to_store;
    store_data(key, string_to_store.c_str());
    //std::string loaded_string = load_data(key);
    load_data(key);
    //std::cout << "Loaded string:";
	//std::cout << loaded_string;
}

#include <string>
#include <iostream>

extern "C" std::string load_data(std::int64_t);

int main(void) {
  std::string loaded_string = load_data(1234567890);
  std::cout << loaded_string; 
}

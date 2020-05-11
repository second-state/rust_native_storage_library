#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>

namespace io {
namespace second_state {
namespace rust_native_storage_library {

class Key_Value {
  private:
  std::vector<uint8_t> key;
  std::vector<uint8_t> value;
  
  public:
  Key_Value() {
    printf("KeyValue instantiated, use set_key and set_value to populate\n"); 
  } 
  std::vector<uint8_t> get_key(void) {
      return key;
  }
    std::vector<uint8_t> get_value(void) {
      return value;
  }
  void set_key(std::vector<uint8_t> _key) {
      key = _key;
  }
  void set_value(std::vector<uint8_t> _value) {
      value = _value;
  }

};

struct SharedThing;

std::unique_ptr<Key_Value> make_demo(rust::Str appname);
const std::string &get_name(const ThingC &thing);
void do_thing(SharedThing state);

        } //namespace rust_native_storage_library 
    } //namespace second_state 
} //namespace io 

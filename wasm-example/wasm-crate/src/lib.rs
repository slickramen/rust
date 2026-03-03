use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    log(&format!("adding {} and {}", a, b));
    return a + b;
}

#[wasm_bindgen]
pub struct Person {
    name: String,
    age: u32,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn age(&self) -> u32 {
        self.age
    }
}

#[wasm_bindgen]
pub fn get_people() -> Vec<Person> {
    vec![
        Person::new("Alice".to_string(), 30),
        Person::new("Bob".to_string(), 25),
        Person::new("Charlie".to_string(), 35),
    ]
}

// size of the buffer
const WASM_MEMORY_BUFFER_SIZE: usize = 2;
// lazy initialization of static global memory byte slice
lazy_static!{
    static ref WASM_MEMORY_BUFFER: Mutex<[u8; WASM_MEMORY_BUFFER_SIZE]> = Mutex::new([0; WASM_MEMORY_BUFFER_SIZE]);
}

#[wasm_bindgen]
pub fn store_val_at_idx_zero(value: u8) {
    let mut buffer = WASM_MEMORY_BUFFER.lock().unwrap();
    buffer[0] = value;
}

#[wasm_bindgen]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
    let buffer = WASM_MEMORY_BUFFER.lock().unwrap();
    buffer.as_ptr()
}

#[wasm_bindgen]
pub fn read_wasm_memory_buffer_at_idx_one() -> u8 {
    let buffer = WASM_MEMORY_BUFFER.lock().unwrap();
    buffer[1]
}

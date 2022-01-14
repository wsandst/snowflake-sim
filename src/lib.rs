use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct SnowflakeSimContext {
    test : u64,
}

#[wasm_bindgen]
impl SnowflakeSimContext {
    pub fn new() -> SnowflakeSimContext {
        SnowflakeSimContext { test: 3 }
    }
    
    pub fn get_str(&mut self) -> String {
        return String::from("Yay!");
    }
}
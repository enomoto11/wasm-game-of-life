mod utils;

use wasm_bindgen::prelude::*;

// Rustのexternは「どの言語の呼び出し規約（ABI）を使うか」を指定します。
// ここではC言語のABI（Application Binary Interface）を指定しています。
// Rustにはextern "JS"は存在せず、wasm-bindgenがこのextern "C"の関数をJavaScriptに橋渡ししてくれます。
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello {}!", s));
}

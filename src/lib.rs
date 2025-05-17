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

// u8としているのは u8型の変数が1バイトであり、ここではenumを1バイトの値として扱いたいから
// https://doc.rust-lang.org/reference/type-layout.html#primitive-data-layout
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

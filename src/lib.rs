use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn say(s: &str) -> String {
//   println!("The Rust function say() received {}", s);
//   let r = String::from("hello ");
//   return r + s;
// }

#[wasm_bindgen]
pub fn say() {
  let mut y: i32 = 1;
  while y < 10 {
    let mut x: i32 = 1;
    while x < 10 {
      print!("{}*{}={:?} \t", x, y, x * y); // 同行内打印
      x += 1;
      if x > y {
        break;
      }
    }
    println!(""); //换行
    y += 1;
  }
}


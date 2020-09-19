use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}

pub fn main(s: &str) -> String {
  let mut y: i32 = 1;
  while y < 10 {
    let mut x: i32 = 1;
    while x < 10 {
      let r = String::from("{}*{}={:?} \t", x, y, x * y);
      print!(r); // 同行内打印
      x += 1;
      if x > y {
        break;
      }
    }
    println!(""); //换行
    y += 1;
  }
}


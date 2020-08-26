fn main() {
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

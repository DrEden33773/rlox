#[test]
fn test() {
  let a = "str";
  let b = "aaaaaaaaaa";
  println!("{} < {} = {}", a, b, *a < *b);
}

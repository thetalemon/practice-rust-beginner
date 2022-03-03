fn main() {
  // printlnで遊ぼう
  let answer: &str= "cram";
  println!("How can a clam {} in a clean cream can?", answer);
  println!("{number:>width$}: 空白揃え", number=20, width=6);
  println!("{number:0>width$}: 0埋め", number=20, width=6);
  println!("{}(10進数), {:b}(2進数)", 2, 2);

  // 再代入をしよう
  // mutをつけると再代入可能になる　mutableのmut？
  let mut x = 1;
  // ↑のコードはエディタで見ると:i32が自動表示される。これは符号付き整数型のことらしい
  // 初期でモノを代入するときは型を自動で推測してくれるっぽい。
  println!("{number:>width$}", number=x, width=6);
  x = x + 1;
  println!("{number:>width$}", number=x, width=6);

  // 文字列
  let this_is_str = "これは&str。";
  let this_is_string = "これはString。".to_string();
  let mut this_is_mutable_string = "これはmutableなString。".to_string();
  // ↓はコンパイルエラーになる
  // this_is_string.push_str("これで文字を追加できる");
  this_is_mutable_string.push_str("これで文字を追加できる");
  println!("{}, {}",this_is_str, this_is_string);
  println!("{}",this_is_mutable_string);
  
}

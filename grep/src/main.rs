use rayon::prelude::*;
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
  #[structopt(name = "PATTERN")]
  pattern: String,
  // Vec（ベクタ）にすると複数入る Arrayみたいなもの
  #[structopt(name = "FILE")]
  path: Vec<String>,
}

// これはstructoptがない場合の実装
// impl GrepArgs {
//   fn new (path: String, pattern: String) -> GrepArgs {
//     GrepArgs {path, pattern}
//   }
// }

fn grep(content: &str, pattern: &str, path: &str) {
  for line in content.lines() {
    if line.contains(pattern) {
      println!("{}: {}", path, line);
    }
  }
}

fn run(state: GrepArgs) {
  // iter()でベクタをイテレートできる　Array()みたいなもの
  // rayonをいれてpar_iterにするとforを簡単に並列化できる
  state
    .path
    .par_iter()
    .for_each(|file| match read_to_string(file) {
        // &にしないと所有権を移譲してしまうので、それ以降元の関数で使えなくなってしまう
        // &であれば参照権限を付与するだけなので問題なし
        Ok(content) => grep(&content, &state.pattern, file),
        Err(reason) => println!("{}", reason),
    });
}

fn main() {
  run(GrepArgs::from_args());

  // -- tructoptがない場合の実装  ここから --
  // let pattern = std::env::args().nth(1);
  // let path = std::env::args().nth(2);
  // match(pattern, path) {
  //   (Some(pattern), Some(path)) => run(GrepArgs::new(path, pattern)),
  //   _ => println!("pattern or path is not specified!")
  // }
  // --tructoptがない場合の実装 ここまで --
}

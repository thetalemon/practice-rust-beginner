use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]

struct GrepArgs {
  #[structopt(name = "PATTERN")]
  pattern: String,
  #[structopt(name = "FILE")]
  path: String,
}

// これはstructoptがない場合の実装
// impl GrepArgs {
//   fn new (path: String, pattern: String) -> GrepArgs {
//     GrepArgs {path, pattern}
//   }
// }

fn grep(content: String, pattern: String) {
  for line in content.lines() {
    if line.contains(pattern.as_str()) {
      println!("{}", line);
    }
  }
}

fn run(state: GrepArgs) {
  match read_to_string(state.path) {
    Ok(content) => grep(content, state.pattern),
    Err(reason) => println!("{}", reason),
  }
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

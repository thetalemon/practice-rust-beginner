// これはRustで参照渡ししたものを編集できるのかを試したかったやつなのだが、
// 『mutableなものを参照渡し』自体が高難度すぎて『すごいがんばらないとたぶんむずかしい』という結論に至った
// 逆に、親でimmutableで定義していても、子に渡す時にmutableにできるのは面白い

struct User {
  name: String,
}

impl User {
  fn new(name: String ) -> User {
    User { name }
  }
  fn set_name(&mut self, name: String) {
    self.name = name
  }
}

fn func_a(mut user_a: User) {
  user_a.set_name("haruko".to_string());
  println!("{}", user_a.name);  
}


fn main() {
  let user_a: User = User::new("hanako".to_string());
  func_a(user_a);
}

# Rust入門
- [この資料](https://speakerdeck.com/helloyuk13/rusthanzuon-at-rust-ca-1-day-youth-boot-camp)を参考に基本を覚えていく

# ここから先は勉強したことのメモ

## いきなり動かなくて泣いた
- [Montereyにあげた後はxcode-select installを再度叩いておく必要があるっぽい](https://qiita.com/ka2kama/items/f1ad9dd878d152d69472)
- ↑で無事動いた

## 1日目開発(hello_rustディレクトリ / P.62まで)
- エディタ上でGUIでrunできるの便利すぎてずるい
- &普段あんまり使わなさすぎて迷子になる
- Rustにおいてはletでもイミュータブルらしい
- printlnは`{}`が変数を代入するプレースホルダらしい。Cの`%s`みたいなもんか
  - なんか結構いろんな使い方ができるみたい。[公式ドキュメントのprintの項](https://doc.rust-lang.org/rust-by-example/hello/print.html)に色々書いてある
    - width指定は面白い。テーブル作るのめちゃくちゃ楽になりそう。jsにも欲しい。
  - print動かしてるだけでこんな楽しいの久しぶり🦀

### 資料 P.50らへん、文字列型の話
- Stringと&strは違う
  - Stringはリサイズ可能でミュータブル
  - &strはイミュータブル
- だいたいミュータブルにしたいかどうかで選べばOKっぽい
- 違いとしてはメモリの確保方法。&strは厳密に最初の宣言文しか確保しないが、Stringはちょっと余分にとる。メモリ的には&strのが節約になる。はず。
- 相互に変換はできるので、必要になるまではイミュータブルにしておくのもアリだろうか


## 2日目開発(fizzbuzzディレクトリ / P.63 ~ P.77 )
- Rust三項演算子ないみたいやった！すてき！その姿勢がすてき！
  - 正直三項演算子を使わないためにif文だけの関数とか書くから、私はしっくりくる書き方に常に倒せて嬉しい
- `0..100`でPythonにおける`range(100)`と同じになるのね
  - 実際range使うシーンってFEだとあんまないけど、あるとありがたい気持ち
- `{}`で囲ってあっても、`;`なければそれが末尾の文と判断するので`return`なくてもいいらしい
  - `;`つけて`return`してもいいらしい。
  - 個人的には`return`省略を許容しないルールを導入したいな。末尾の`;`有無で`return`かどうかを判断するのは人類にはむずかしいとおもう！
- P.71~ やべー！関数型プログラミング何もわからん。
  - foldはjsでいうとreduceかな？
  - 関数型の方がじゃっかん最適化がききやすい、って程度なので、手続型でかいてもいいらしい。
  - でもどこかでしっかり履修したいな関数型プログラミング
- キリもいいのでここまで。

## 3日目開発(catディレクトリ / P.78 ~ P. )
- パターンマッチ面白いかも。簡素なswitchっぽい？

- Result型（enumの一種らしい）
  - Ok(content), Err(reason)はOk.getContent(), Err.getReason()みたいな感じで覚えていい…のか…？
    - ともあれ、content / reasonのように第一引数？に理由が代入されるらしい。アロー関数っぽいものと理解したらいいかな？
- Ok/Errを後でハンドリングしたい場合は末尾に`?`をつければいいらしい
  - そもそもResult型がmatchの引数になるときは、OkとErrがペアにならないとコンパイル通らないみたい。
  - then/catchよりも明示的でいいかも。Goもthrowできないけど、throwできない系の流れはイイね。
```rust
  match read_to_string(path) {
    Ok(content) => print!("{}", content),
    Err(reason) => println!("{}", reason),
  }
```

- Some/Noneはこんな感じ。
```rust 
  match std::env::args().nth(1) {
    Some(path) => run_cat(path),
    None => println!("No path is specified!"),
  }
```

- あたいがある場合だけ処理したい、はこんな感じにできるらしい。
```rust
  if let Some(path) = std::env::args().nth(1) {
    run_cat(path);
  }
```

- 数値型。文字列型でも似たようなものが書ける
```rust
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    // これはその他
    _ => println!("anything"),
  }
```

- nthってCSS以外ではじめてみるけど何の意味がググったら『N番目』って言われました。12thのthだった。

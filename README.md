# Rust入門
- [この資料](https://speakerdeck.com/helloyuk13/rusthanzuon-at-rust-ca-1-day-youth-boot-camp)を参考に基本を覚えていく

# ここから先は勉強したことのメモ

## いきなり動かなくて泣いた
- [Montereyにあげた後はxcode-select installを再度叩いておく必要があるっぽい](https://qiita.com/ka2kama/items/f1ad9dd878d152d69472)
- ↑で無事動いた

## 1日目開発(hello-rustディレクトリ / P.62まで)
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


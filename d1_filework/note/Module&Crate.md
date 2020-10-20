# module

Rust package は
- lib: ライブラリ
- bin: 実行ファイル

の2つの部分を持つことができる。


```toml
[[bin]]
name = 'd1_load_trans'
path = 'src/main.rs'

[lib]
name = 'd1_filework'
path = 'src/lib.rs'

```

binは複数持つことができる 
`[[]]`はTOMLのリスト表現
2つ以上のbinがある場合、`cargo run --bin <name>`オプションを付けないと
実行できない
(1つのときは`main.rs`がある場合省略できる)


# mod

ローカルファイルインポート

Rust においては File = Module

`mod`キーワードは、ファイルをモジュールとしてロードする役割がある。
ecmascript の `import`や、commonjsの`require`近い

## inner module
OCamlのように、ファイルモジュール内に更にモジュールを定義することも可能

```rs
// lib.rs
pub mod error;
mod inner_module {}

// lib.rs 内部で、更にerror モジュール内のアイテムにネームスペース無しでリクエストしたい場合
use error::*;
```

# extern crate

外部モジュール import 
`cargo.toml`で依存を追加したり、`lib.rs`で別れている場合に`main.rs`を書く際に
それらのパッケージへの依存を記述する。

省略して `use`　をいきなり書き始めても使えるが、
以下の利点があるらしい。
- lib.rs を書く場合、その外部crateをモジュール以下にインポートする
- `extern crate * as alias`で別名を付けられる


## crate::, ::

```rs
use ::extern_library;
use crate::local_library;
```
こういう書き方もある

crate = ローカル crate へのエイリアス
`  ::` = 依存関係のrootからの絶対パス




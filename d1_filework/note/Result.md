# serde

このプロジェクトでは、3つの依存関係を使う

- serde
- serde_derive
- serde_json

https://serde.rs/

SERializing and DEsirializing rust data structure
Rust のデータ構造をシリアライズ、デシリアライズするための crate

serde_derive は `derive`マクロによる自動実装
serde_json は Json シリアライザ/デシリアライザ

多分、S式のやつとかもありそう
- JSON
- Bincode
- CBOR
- YAML
- TOML
- MessagePack
- S-expressions
- FlexBuffer

シリアライズは、Webプログラミングにおいて通信フォーマットとプログラミング固有のデータ構造をマッピングするのに重要


## derive(Serialize, Desirialize)

`use serde_deive::{Serialize, Desirialize}` しておく

```rs
#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream { ... }

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {...}
```

元のコードはこんな感じ
`macro` は`TokenStream`を引数にして`TokenStream`を返す関数で実装されている

`#[proc_macro_derive(Serialize, attribute(serde))] fn ...`

直後に続く関数を使って、`Serialize`マクロを実装する。
`serde`属性がついた`cargo build`でビルドする。



## derive(Debug)
構造体の各フィールドを、`{:?}`で出力するように自動実装する。

# File を開く => Json Decode までのエラーハンドリング

```rs
fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };
    Ok(t)
}
```

match 式を使ってエラーハンドリング
`return`を使って早期リターンすると、match式のブランチの途中で値を統一しなくてもいい(戻り値の型に合わせる)


# Combinator関数を使って、Matchの使用を避ける

matchを使うと明示的にはなるが、コード的には多少長くなる。
`std::result`モジュール内に定義されている、Resultを操作する関数を
使ってより簡潔に書ける

- `and_then()` : `Result<T, E> -> (T -> U) -> Result(U, E)`
- `map_err()` : `Result<T, E> -> (E -> F) -> Result(T, F)`

Result の中身が Ok のときだけ、Okの中身に closure を適用する。
map_errはその逆

Ocamlにおける mapOk, mapErr みたいな

Err の中身を String に統一したいので、エラーは `e.to_string()`でStringにする


### pub で未使用warningを消す

`pub`が付いてる関数は、一度も使われなくても警告が出ない


# From trait

`From<T>` trait は、from 関数を持つ trait
`from: T -> Self`

これを実装すると、エラーから別の独自のエラーに変換できる
(エラーだけに使う Trait ではない)

今回は、`std::io::Error`と`serde_json::error`の2種類があるので
一つのエラータイプにまとめる

```rs
#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}
impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}
```

enum便利

# ? macro
`result?` とすると、result が Error だった場合に
すぐにreturnする。

以下2つは同じコード

```rs
pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(err) => return Err(err.into()),
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(err) => return Err(err.into()),
    };
    Ok(t)
}
```

```rs
fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    let json_str = std::fs::read_to_string(fname)?;
    let json = serde_json::from_str(&json_str)?;
    Ok(json)
}
```

`?` = `match result { Ok(v) => v, Err(err) => return Err(err.into()) };`
OKだったら中身を返す、Errだったら戻り値型のE型にinto()してreturn

## Self::from() & t.into()

`From` trait を実装すると、変換元の構造体or enum で`into()`メソッドが使えるようになる




# エラー処理のパターン
- match
- Result Combinator 関数
- ? (& From)



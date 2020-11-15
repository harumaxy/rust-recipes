# The Rust Recipie Course

- Error handling with Result & Option
- Iterators
- Concurrency and Lifetimes
- Simple Patterns
- Future with async / await
- Database with Diesel
- WebServer with Rocket
- FFI

# 専門用語

- Macro : code を書く code、プリプロセッサ
- Iterators : 似たような処理を繰り返すためのツール
- I/O : input / output
- Asynchronous : すぐに終わらない処理
- Future : async なコードを書くためのヘルパー trait


## Future  を詳しく

- rust 非同期処理のメインメカニズム
- `Tokio` crate に管理される
- 最近は stable で std::future に取り込まれた？
  - このコースの初版が 11ヶ月くらい前っぽい
- multi thread でも安全に動作

## このコースの目標

- Lifetime system を理解する
- Generic を理解する
- 

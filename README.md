# The Rust Recipie Course

1. Error handling with Result & Option
2. Iterators
3. Concurrency and Lifetimes
4. Simple Patterns
   - デザインパターンのクラスの実装のこと？
   - Factoryとかß
5. Future with async / await
6. Database with Diesel
7. WebServer with Rocket
8. FFI

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
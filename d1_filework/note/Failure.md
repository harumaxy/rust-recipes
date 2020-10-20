# Failure crate

新しいエラータイプを作る

`failue`をcargo.tomlに追加して使う
独自のエラータイプを実装するための便利なユーティリティ

今回の方法だと、エラーごとにいちいち実装しないといけないので
独自のエラーを拡張するのが面倒

Failure を利用すると、Backtrace を使える。

実行時に、`RUST_BACKTRACE=1 cargo run`で実行すると
バックトレースを出力する。



使ったほうがいいらしい

使うときは、アプリケーション内のすべての`Result<T, E>`を `E = failure::Error` に置き換える
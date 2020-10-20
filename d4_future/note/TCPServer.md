# Simple TCP Server

`tokio::net`
`tokio::io`
を使って作れる


# features

Rust では、いらないライブラリはランタイムに含めないようにする文化がある(軽さのため)
なので、同じライブラリに複数の機能がある場合は、それぞれをインポートする・しないを選択することができる。

すなわち、Cargo.tomlに辞書形式で `features`の配列を指定する必要がある

通常は`full`でok

# tokio::main

通常は、main関数を `async fn`にすることは出来ない

ただし、`#[tokio::main] fn main(){}` とすることで
main関数を非同期関数にできる
(executerも必要なし)

# nc (netcat)
net を concat する。
ネットワーク開発のスイスアーミーナイフ

基本
`nc [option]  <host> <port>`

- `-l` : リッスンモード。サーバーになる

普通のとリッスンモードのタスク２つで、エコーサバーみたいな感じにできる


# Stream vs Future

Stream: `Poll<Option<T>>` を返す。複数のFuture の集まり、イテレーションできる。
Future: `Poll<T>` を返す。一回返したら終わり。

# async block が解決するもの

= Poll type (Future の状態を返す)
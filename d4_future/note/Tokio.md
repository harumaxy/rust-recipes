# Tokio crate

tokio は Future を実行するためのランタイム

Rust の非同期処理は、言語機能ではなくライブラリで提供されているため、
Future を実行するためのランタイムも別のライブラリとして管理されている

色々あるらしい


- tokio
- async-std

tokio のほうが成熟していておすすめらしい

https://qiita.com/legokichi/items/53536fcf247143a4721c

# Tokio

https://tokio.rs/

`Tokio is an scynchronous runtime for the Rust.`

ネットワークアプリケーションを実装するためのランタイム

- Reliable
- Fast
- Easy
- Flexible

エラーハンドリングや、Future の Executer として
優秀。
ほぼデファクトっぽい

# The Stack

- Runtime
- Hyper: HTTP client & server (HTTP1/2 両対応)
- Tonic : gRPC client & servwer を作るためのボイラープレート不要なライブラリ
- Tower : retry, load-balancing, filtering, request-limiting などなどのモジュール
- Mio : ミニマルで携帯性の高い OS evented I/O API (かなりベースの部分)
- Tracing : ログ、監視
- Bytes : ネットワーキングアプリのコアをなす、バイトストリームを操作するためのユーティリティ


# compat , io-compat

`Future` を `AsyncRead, AsyncWrite` に変換する
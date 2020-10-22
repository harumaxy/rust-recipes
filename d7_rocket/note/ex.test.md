# Responder とは？

`Responder` trait を実装するすべての構造体

# Handler とは？

`Responder` を返す関数
(多分、Result も Responder を実装されてる)


# NamedFile は何？
　
`Responder`

# maud とは？

html template (macro)を Rust code にするライブラリ

# rocket.mount() はなにをする？

handler を Rocket app に対してリスティング氏、
（Request に対して）適切なものを選択させる

# Request の特定の部分に、handler内でアクセスするには？

`Extractor`をハンドラーの引数に含める

- `FormData` : `post`マクロの `data` を引数名を指定する
- `Cookies`
- `State<T>`
- `DPool` : manage で state 変数を追加する必要あり

# state を各リクエストのプロセス間を超えて管理するには？

1. アクセスしたいデータを store するタイプを作る `struct A() = Arc<Mutex<T>>`
2. 変更したいデータの場合、Mutex で囲む
   - read only なら囲まなくていい
3. Store type のインスタンスを `.manage()`で追加

# Rocket に Diesel への接続を含めるには？

- `rocket_contribe`の features=["diesel"] をインストール
- Connection Object に `#[database]`マクロを実装
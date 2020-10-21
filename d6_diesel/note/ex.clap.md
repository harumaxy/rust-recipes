# clap

cli args , 及びヘルプメッセージを効率よく実装するためのライブラリ
- macro
- struct
- DSL

```rs
extern crate d6_doodle;

use clap::{clap_app, crate_version};

fn main() -> Result<(), failure::Error> {
    let clap = clap_app!(doodle_cli =>
      (about: "A cli for the doodle database app")
      (version:crate_version!()) (author: "Max")
      (@subcommand new_user =>
        (@arg name:+required "the name of the new user")
        (@arg password:+required "the new user's password")
      )
    )
    .get_matches();

    if let Some(sub) = clap.subcommand_matches("new_user") {
        let user = new_user(
            sub.value_of("name").unwrap(),
            sub.value_of("password").unwrap(),
        );
    }
    Ok(())
}
```


# `clap_app!` macro

アプリケーション起動時に、コマンドラインから取得する引数をマッチング、定義するDSLを処理するマクロ

```rs
clap_app!(dsl);
```

# DSL

S式っぽい

```rs
clap_app!(<main_command_name> => 
  (about: "")
  (version: "")
  (author: "")
  (@subcommand: <sub_command_name> => 
    (@arg name:+required "")
    (@arg password: "")
  )
);
```

他にも色々ありそう(description とか type 指定とか)

# `crate_version!()` macro

`Cargo.toml`に書いてある crate のバージョンを取ってきてくれる？
(多分文字列)


# @subcommand

`@subcommand =>`で、何も`@arg`を書かないと引数なしサブコマンドになる
main直下に複数並べられる(List)


# flag

```rs
(@subcommand new_poll =>
  (@arg question:+required "the question")
  (@arg options:+required "the options (Comma separated)")
  (@arg user_id:-u +takes_value +required "the owner of the poll")
)
```

`-u` と `+takes_value` をつけると、フラグ引数を付けられる
(`+takes_value`なしだとただの bool ?)

`cargo run -- new_poll "how old are you" "<5,<10,<15,>=15" -u 1`


# cargo run --

`--` をつけると、`cargo`コマンドに対する引数ではなく
`run`で実行するプログラムに対する引数になる
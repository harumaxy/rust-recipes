# How to use Lifetame variables

ライフタイム変数を使うべき場面
`&`参照型がデータ構造に含まれるとき

```rs
pub struct NewUser<'a> {
    name: &'a str,
    password: String,
}
```
参照が構造体に含まれる場合、もととなる参照のオブジェクトが開放されると、
この struct の 参照が指す先がなくなる。

参照しているデータが開放されるとともに、このstruct も開放する or 使えなくするために
ライフタイム変数が必要

## ライフタイム付き参照を受け取る

関数でライフタイム変数を使う場合というのは、
`ライフタイム変数を含む構造体を作成するとき`

```rs
pub fn new_user<'a>(name: &'a str, password: &str) -> NewUser<'a> {
    NewUser {
        name,
        password: bcrypt::hash(password, 7).unwrap(),
    }
}
```

`NewUser`は、文字列への参照をメンバに含む
もととなる文字列refを引数として取って `Newuser`を作成する場合、
ライフタイムはその文字列への参照と同じ寿命を共有しないといけない
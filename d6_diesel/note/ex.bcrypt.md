# bcrypt

NewUser を作成するときのコード

```rs
pub fn new_user<'a>(name: &'a str, password: &str) -> NewUser<'a> {
    NewUser {
        name,
        password: bcrypt::hash(password, 7).unwrap(),
    }
}
```

強度7のハッシュ化をかける。


User のパスワードが正しいかどうか判定するコード

```rs
impl User {
    pub fn verify_pass(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
}
```

多分、パスワードを同様にハッシュして等しいかどうかを判定してる


# より安全なパスワード

ハッシュ化アルゴリズムは、パスワードをDB上で難読化させるためだけのもの。
実際にはこれを盗み見ればわかってしまう。

それ + ソルトを加えることで、
より安全なパスワードを作れる

# easy_password crate


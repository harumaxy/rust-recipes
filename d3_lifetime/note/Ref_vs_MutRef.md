```rs
#[derive(Debug)]
pub struct StringHolder<'a> {
    s: &'a str,
    t: &'a str,
}

fn main() {
    let mut s = make_str(7);
    s = to_people(s);
    let p = part(&s);

    let tog = two_strings(p, &s);

    println!("Tog = {:?}", tog);
    s.push_str("anything"); // error
    println!("Tog = {:?}", tog)
}
```
上のコードはエラー

1. immutable ref (&) が発生
2. mutable ref (&mut) が発生
3. immutable ref を使う

この流れになるとエラーになる。
多分、最初に発生したimmutable ref が変更されてしまっている可能性があり、
データが矛盾するかもしれないため。

この規則は、 immutable ref (&) のライフタイムの間続く
元の変数がどこかの関数などで consume  されてたりすると、別のエラーになる(ライフタイムの期限切れ)


# Lifetime variable

type variable と同じ場所に書く
アポストロフィ ' にアルファベットを続ける
大抵は a, b ,c ...

```rs
pub fn two_strings<'a>(s: &'a str, t: &'a str) -> StringHolder<'a> {
    StringHolder { s, t }
}
```
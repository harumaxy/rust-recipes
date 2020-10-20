# Reference Counting


```rs
fn make_with_life<'a>(fname: &str) -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
    // ローカル変数への参照は返せない(ライフタイムがここで終わってしまうため)
    let s = std::fs::read_to_string(fname)?;
    Ok((WithLife { s: &s }, WithLife { s: &s }))
}
```

ライフタイムが、引数によって外部からもたらされない場合に、
関数スコープ内でライフタイムが終わるローカル変数の参照を返すと
エラー


## Return Box struct

値そのものや、値の参照を返すのでではなく、
値を Heap に入れて、そのポインタを返す `Box Struct`を返すことで
Lifetime のスコープにまつわる問題を解決することができる。

```rs
#[derive(Debug)]
pub struct NoLife {
    s: Rc<String>,
}

fn make_no_life(fname: &str) -> Result<(NoLife, NoLife), std::io::Error> {
    let s = std::fs::read_to_string(fname)?;
    let r = Rc::new(s);
    Ok((NoLife { s: r.clone() }, NoLife { s: r }))
}
```

それぞれ、Reference Counter のクローン

ただし、`Rc`の中身は Immutable なので、
変更することが出来ない

## Mutex vs Refcell

Rc が Immutable な Box オブジェクトなら、
Mutable な Box に変えればいい

基本的に、Rust には `&` と `&mut` の参照は同時に存在できない。
Cell は、複数の参照を持ちつつ内部の値を変更するという Rust の借用権ルールを緩和するためのオブジェクト

シングルスレッド
- Cell
- RefCell
  - 同期が実装されてないので、そちらは Mutex に任せたほうがいいい

マルチスレッド
- Mutex
- RwLock


# RefCell


内部のデータが借用されたかを監視する。

`refcell.`
- `bollow_mut()`
- `bollow()`


```rs
let (n1, n2) = make_no_life("test_data/v3_data.txt")?;

let mut s = n1.s.borrow_mut();
s.push_str(" What i it was even bigger");

println!("n1 = {:?}", n1);
println!("n2 = {:?}", n2);
println!("s == {}", s);
drop(s);
println!("n1 = {:?}", n1);
println!("n2 = {:?}", n2);
```

# Arc vs Rc

- Reference Counter
  - 同期が実装されてない
  - マルチスレッドになると、カウントがおかしくなる
- Atomic Reference Counter
  - 同期が実装されていて、複数スレッドで共有しても参照カウントが狂わない

シングルスレッド向け = `Rc`
マルチスレッド向け = `Arc`
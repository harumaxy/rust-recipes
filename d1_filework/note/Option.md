# Option

Option の扱いになれる

## get_first_of_vec

要するに find

```rs
fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
    let trans = get_transactions(fname).ok()?;
    trans.into_iter().find(|t| t.from == uname)
}
```

動画だと、`for in`でやってた

## iter 変換メソッド

`vec.`
- `iter()`
- `iter_mut()`
- `into_iter()`

iter() と into_iter() は、元のコレクションの参照を使って iterator を作る
`into_iter()`は、データ所有権をIterに変更するので、もととなったデータにはアクセスできなくなる。

`iter().find()`した結果の中身が、参照になるか元データになるかの違い


## option.ok_or(E)

Option を Rsult 型に変換するメソッド
`?` と組み合わせて、見つかったらOK, 見つからなかったら Error にできる。

## main の戻り値を Result に

```rs
fn main() -> Result<(), TransactionError> {

}
```
main関数でも Result を戻り値にできる
`?`マクロも使える



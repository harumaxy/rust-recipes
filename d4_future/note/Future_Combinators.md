# Future Combinator 

# FutureExt 

`FutureExt` trait
Futureに method を chain, compose するトレイト

Extra method だから FutureExt

- `then`
- `into_stream`
- `flatten`

下の一文が必要

```rs
use futures::future::FutureExt;
```

## then

`Future -> F -> Then<Self, Fut, F>`

Future と F (function) を取って Then を返す。
Promise の then と同じ

```rs
let f = SimpleFuture { n: 10 };
let v = block_on(f.map(|n| n + 1).then(|v| SimpleFuture { n: v }));
```

## futures::channel module & channel::oneshot

futures::chanel は、別スレッドのFuture と通信するためのやつ

- oneshot: 一回受け取ったら終了
- mpsc : 複数回受け取れる、sender がすべて開放されたら終了


```rs
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::future::FutureExt;
#[test]
fn test_future_returns_a_value() {
    let (ch_s, ch_r) = oneshot::channel();
    let f = SimpleFuture { n: 10 };
    let v = block_on(f.map(move |v| ch_s.send(v + 5)));
    let result = block_on(ch_r).unwrap();
    assert_eq!(result, 15);
}
```


# async : Future を作るもっと簡単な方法

先程は、stcut に `Future` trait を実装して、
構造体を初期化することで Future を作った。

`async fn`で非同期関数を作れば、
もっとかんたんに Future が作れる

`async fn`の戻り値は自動的に Future にラップされる

# Pin trait : move が起こらないようにする。

```rs
let f = simpleexec(10);
let v = block_on(f.map(move |v| ch_s.send(v + 5)));
```

Future の内部的に使われている構造体、`Pin`は
これでラップしたデータが move しないことを保証する。

なので、　move セマンティクスをつけた closure で実行しても、
Future の中身の値は移動しない


# async block

async block = 引数なし無名`async fn`
ブロック内部のルールは、`async fn`と同じ

```rs
#[test]
fn test_async_send() {
    let (ch_s, ch_r) = oneshot::channel();
    block_on(async move {
        let v = simpleexec(10).await;
        ch_s.send(v)
    })
    .unwrap();  // Result型なので処理するか握りつぶさないといけない

    let fin = block_on(async move {
        let res = ch_r.await.unwrap();
        res + 5
    });
    assert_eq!(fin, 25)
}
```

`futures::executer::block_on()`の引数にもできるし、
関数の戻り値にもできる
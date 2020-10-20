# Closure as parameter

```rs
#[derive(Debug)]
pub struct HideVec {
    v: Vec<String>,
}

impl HideVec {
    pub fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(String::new())
        }
        HideVec { v }
    }
    pub fn edit_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut String),
    {
        for item in &mut self.v {
            f(item);
        }
    }
}
```

closure は、型がゆるいので
closure を引数にして、関数の中で別の関数(map, filter, fold)に適用するときは
型をしっかり明示してやる必要がある

- FnOnce
- FnMut
- Fn

# fn ... where Type:Trait ...

型パラメータを使う関数では、ある型があるTraitだった場合の動作を
個別にかける。

特に、FnOnce, FnMut, Fn の使い分けはしっかり書いたほうがいい
(3つの中から一つだけでもいい)
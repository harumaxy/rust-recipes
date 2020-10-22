# Rust 内で定義してない関数の定義はどこに書く？

`extern`ブロックに、関数の signature だけ書く

# なぜ C を external language として扱っているの？

1. Cにはよく洗練された`ABI` (Application Binary Interface)がある。
2. C は Rust より古いので、Rust側が合わせなければいけない
3. C コンパイラは Rust のことを気にかけていない
4. Rust と C++ の間には、安定したABIがまだない
5. ABIの提供より、優先すべきことが Rust の開発にある


# *mut (raw pointer) と &mut の違い

`*mut` 自体は Rust の要素

1. `*mut`は safe rust では追跡できない
2. `*mut` にはライフタイムの情報がない(手動で開放しなければならない)

`*mut` = raw pointer = Cのポインタ

# なぜ、libloading 関数は `unsafe` なfunction を返す？

Rust 以外の言語のメモリ安全性を、Rustは保証しないから
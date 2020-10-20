# Future とは
- Cheap threads
  - cpuが弱いパソコンでも並列処理する方法
- Function を持ったObject
- 何かを待ち受ける
- 構造体の中に隠されている
- JSのPromiseみたいなもの


# Future vs Promise, Go function, Green threads

後者は、言語に埋め込みの機能
Rustでは、Future はライブラリとして提供されている

ゼロコスト抽象化の観点から、
Go や Js Promise のように、言語自体にマルチスレッドの思想が組み込まれていると、
シングルスレッドのプログラムを書いていても、ランタイムにマルチスレッドが入ってくる。
使わないのにランタイムが重くなる

# Creating A Future


# TODO アプリ

jsonファイルをストレージとする、めちゃくちゃシンプルなTODOアプリ

# ビルドごとのバイナリのサイズ

* target/debug は通常のデバッグビルド
* target/release は通常のリリースビルド
* x86_64-unknown-linux-musl/release はmuslをリンクしたビルド。glibcにリンクしていないのでポータブル。

```
$ ls -lh ./target/{debug,release,x86_64-unknown-linux-musl/release}/rusty-journal
-rwxr-xr-x 2 tamanobi tamanobi  16M Apr  7 22:58 ./target/debug/rusty-journal
-rwxr-xr-x 2 tamanobi tamanobi 4.0M Apr  7 23:02 ./target/release/rusty-journal
-rwxr-xr-x 2 tamanobi tamanobi 4.5M Apr  7 23:07 ./target/x86_64-unknown-linux-musl/release/rusty-journal
```

小さなTODOアプリだが、リリースビルドだとしても4.0Mと大きめのバイナリになっている。静的リンクをしているため、muslビルドだと、0.5M増している。


# see also

https://docs.microsoft.com/ja-jp/learn/paths/rust-first-steps/

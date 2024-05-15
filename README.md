# RustPractice
practice for Rust

## Install with Mac
### 1. Rust管理ツール`rustup`のインストール
```
$ brew install rustup-init
```
![](https://formulae.brew.sh/formula/rustup-init#default)

### 2. 初期化
```
$ rustup-init
```
インストールの際に1を入力する
```
$ rustup-init
...
...
   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1
```

### 3. 確認
以下のコマンドが実行できればインストール成功
```
$ cargo --version

$ rustc
```
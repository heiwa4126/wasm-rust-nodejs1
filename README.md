# wasm-rust-nodejs1

RustでWebAsemblyを作り、nodeから利用してみるサンプル。


# 手順

事前に
```bash
cargo install wasm-pack
```
などで wasm-pack を入れる。

```bash
mkdir nodejs1; cd nodejs1
mkdir greeting
cd greeting
cargo init --lib
cd ..
mkdir use-greeting
npm init -y
cd greeting
```

Cargo.toml を編集

src/lib.rs を編集

```bash
wasm-pack build --target nodejs
# or
wasm-pack build --target nodejs --release
```
でpkg/の下にパッケージができる。

```bash
cd pkg
npm ln
```
で  ~/.npm-global/lib/node_modules/ の下にsymlinkができる。


使う側に移動
```bash
cd ../../use-greeting
```

import使いたかったので、package.json に `"type": "module"` 追加

```bash
npm ln greeting
```

index.js を編集

```bash
node index.js
```

で出来上がり。


## このレポジトリのクローン直後から動かすには

プロジェクトルートから

```bash
cd greeting
wasm-pack build --target nodejs
cd pkg
npm ln
cd ../../use-greeting
npm i
node index.js
```

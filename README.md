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
node .
```


## メモ

```bash
wasm-pack build --target nodejs
```
と
```bash
wasm-pack build --target web
```
の違い。*.wasmは同じのが生成される。
サポートのjs,ts,package.jsonが違う。


## メモ2

`npm link` が globalにリンクする、`npm i`すると消える、などの問題があって、
単に `node_modules/`の下にリンク作ってほしいのだけど。


`npm i ../greeting/pkg` が使えそうなのに、コピーになってしまう。

[npm-install | npm Docs](https://docs.npmjs.com/cli/v9/commands/npm-install?v=true) の
`npm install <folder>:` には `<folder> がプロジェクトのルート外にある場合、npm はパッケージの依存関係を <folder> ディレクトリにインストールしませんが、 <folder> へのシンボリックリンクを作成します。` とあるのだけど、どうしてもコピーになってしまう。

`--install-links`もダメだし。

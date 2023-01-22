# practice-rust

## 周辺ツール

- rustup: Rust のバージョンと関連するツールを管理する
  - `rustup update`: Rust を最新版へ更新する
  - `rustup self uninstall`: Rust と rustup をアンインストールする
- [Cargo](https://github.com/rust-lang/cargo): Rust のビルドシステム兼パッケージマネージャ
- [clippy](https://github.com/rust-lang/rust-clippy): よくある間違いを見つけて Rust のコードを改善するためのリンター

### Cargo

- `cargo new`: プロジェクトを作成する
- `cargo build`: プロジェクトをビルドする
  - `--release`: 最適化した状態でプロジェクトをコンパイルするオプション
- `cargo run`: プロジェクトのビルドと実行を 1 ステップで行う
- `cargo check`: バイナリを生成せずにプロジェクトをビルドしてエラーがないか確認する

Cargo はビルドの成果物をコードと同じディレクトリに保存するのではなく、`target/debug` ディレクトリに格納する。

## お作法

- ファイル名
  - 2 単語以上使っているなら、アンダースコアで区切る
    - helloworld.rs ではなく hello_world.rs とする

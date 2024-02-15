# practice-rust

## Reference

- [Rust の練習帳](https://github.com/kyclark/command-line-rust)

## Memo

- rustup: Rust のバージョンと関連するツールを管理する
  - `rustup update`: Rust を最新版へ更新する
  - `rustup self uninstall`: Rust と rustup をアンインストールする
- [Cargo](https://github.com/rust-lang/cargo): Rust のビルドシステム兼パッケージマネージャ
  - `cargo new`: プロジェクトを作成する
    - `--libs`: ライブラリクレートを作成する (デフォルトはバイナリクレート)
  - `cargo build`: プロジェクトをビルドする
    - `--release`: 最適化した状態でプロジェクトをコンパイルするオプション
  - `cargo run`: プロジェクトのビルドと実行を 1 ステップで行う
  - `cargo check`: バイナリを生成せずにプロジェクトをビルドしてエラーがないか確認する
- [clippy](https://github.com/rust-lang/rust-clippy): よくある間違いを見つけて Rust のコードを改善するためのリンター

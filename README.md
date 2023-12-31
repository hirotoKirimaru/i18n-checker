# i18n-file-checker

I18n のファイルチェッカーを作りたい。

# 仕様

CLIで使用できるようにしたい。配布しやすいように、GoかRustで作成するのが良いかも。

1. lint
2. diff
3. export path, jsonPath etc.
4. unused key
5. message lint(text lintをかけてメッセージの表記ゆれを減らしたい)

# Memo

```bash
# Rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# GCC install
sudo apt install build-essential
```
```bash
# 新プロジェクト
cargo new i18n-checker

cd i18n-checker
cargo run
```

```bash
# build

cargo build --release

```

```bash
cd target/release/i18n-linter
```
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
```
```bash
# 新プロジェクト
cargo new i18n-linter

cargo run
```
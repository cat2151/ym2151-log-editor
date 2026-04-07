# コミット前にやること

1. **コードフォーマット**: `cargo fmt` を実行して一貫したフォーマットを確保
2. **リンティング問題修正**: `cargo clippy --all-targets -- -D warnings` を実行して警告に対処
3. **ビルド成功**: `cargo build` (または `cargo build --release`) を実行
4. **テスト実行**: `cargo test` を実行して全テストが通ることを確認
5. **Windows互換性チェック**: Windows向けクロスコンパイルチェックを実行

```bash
# Windows GNUターゲットを追加
rustup target add x86_64-pc-windows-gnu

# 警告をエラーとして扱う
export RUSTFLAGS="-D warnings"

# Windows向けにコンパイルチェック
cargo check --all-targets --target x86_64-pc-windows-gnu
```

# userからの指示
- プルリクエストは日本語で書く
- 作業報告は、プルリクエストのコメントに書く。document作成禁止
  - DRY原則に準拠し、「codeやbuild scriptと同じことを、documentに書いたせいで、そのdocumentが陳腐化してハルシネーションやuserレビューコスト増大や混乱ほか様々なトラブル原因になる」を防止する
  - なおissue-notes/は、userがissueごとの意図を記録する用途で使う
- cat2151のライブラリはrev固定禁止

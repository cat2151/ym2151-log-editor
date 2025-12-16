
# ym2151-log-editor用 Copilot インストラクション（日本語）

このドキュメントは、Rust製プロジェクト `ym2151-log-editor` でAIコーディングエージェントが作業する際のガイドラインです。生産性と一貫性を高めるため、以下のルール・ワークフローに従ってください。

## プロジェクト概要
- **目的:** YM2151シンセサイザーのイベントログ（JSON）をターミナル上（TUI）で編集・閲覧するツール。主に時刻編集・イベント確認に特化。
- **言語:** Rust（2021エディション）
- **プラットフォーム:** Windows中心、crossterm/ratatuiでクロスプラットフォーム対応。
- **参考:** [ym2151-tone-editor](https://github.com/cat2151/ym2151-tone-editor)

## アーキテクチャ・主要ファイル
- `src/main.rs`: エントリーポイント。ターミナル初期化、イベントループ、コマンドラインからのファイル読み込み。
- `src/app.rs`: アプリ状態、ビジネスロジック、ファイルI/O、ナビゲーション、時刻表示モード切替。
- `src/models.rs`: データモデル（`Ym2151Event`, `Ym2151Log`）、KeyONイベント判定。
- `src/ui.rs`: UI描画（ヘッダ・イベントリスト・フッタ）、キーボードショートカット表示。
- `test_data/sample.json`: 開発・テスト用のサンプルYM2151ログ。
- `IMPLEMENTATION_PLAN.md`: ロードマップ、機能フェーズ、設計メモ。

## データモデル
- **イベント:** `{ "time": f64, "addr": String, "data": String }`（詳細は `models.rs`）
- **KeyON:** `addr == "08"` のイベントはUI上で `KeyON` として表示。
- **時刻モード:**
  - 累積（デフォルト）: 前イベントからの差分（編集用）
  - タイムスタンプ: 開始からの絶対時刻（保存用）

## 開発ワークフロー
- **ビルド:**
  - 開発用: `cargo build`
  - リリース: `cargo build --release`
- **実行:**
  - ファイル指定: `cargo run -- test_data/sample.json` または `./target/release/ym2151-log-editor your_log.json`
- **キーボード操作（TUI）:**
  - `↑/↓`: イベント移動
  - `T`: 時刻表示モード切替
  - `S`: ファイル保存（常にタイムスタンプ形式で保存）
  - `Q`/`ESC`: 終了
- **テスト:** サンプルJSONで手動テスト（自動テスト未実装）

## プロジェクト固有のルール・慣習
- **表示:** レジスタ `0x08` は常にUI上で `KeyON` に変換。
- **保存:** 表示モードに関わらず、保存時は常に絶対時刻（タイムスタンプ）形式。
- **ナビゲーション:** イベントリストの `selected_index` と `scroll_offset` を維持。
- **編集不可:** 現バージョンは時刻モード切替・保存以外は読み取り専用。
- **エラー:** ファイル読み書きエラーはstderrに出力（TUIには表示しない）。

## 依存クレート
- `ratatui`（TUI）、`crossterm`（ターミナル）、`serde`/`serde_json`（シリアライズ）

## 今後の拡張予定
- 編集（時刻・アドレス・データ）、挿入/削除、Undo/Redo、音声プレビュー、バッチ操作等（詳細は `IMPLEMENTATION_PLAN.md`）

## 機能追加例
- 新しいビジネスロジックは `app.rs` に実装。
- データモデル変更は `models.rs`。
- UI変更は `ui.rs`。
- キー操作追加時は `main.rs`（イベントループ）と `ui.rs`（フッタ表示）両方を修正。

---
詳細は `README.md` および `IMPLEMENTATION_PLAN.md` を参照。内容は簡潔かつ実際のコードベースに即して最新に保つこと。

## コード品質とリンティング

### リンターの実行

コミット前に必ずリンターを実行:

```bash
# コード自動修正
cargo fix --all-targets --allow-dirty

# コードをフォーマット
cargo fmt

# リンターでコード品質をチェック（警告をエラーとして扱う）
cargo clippy --all-targets -- -D warnings
```

### コミット前チェックリスト

コミットまたはコードレビュー要求前に:

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
- PRコメント
  - 作業報告は、プルリクエストのコメントに書く。document作成禁止
    - DRY原則に準拠し、「codeやbuild scriptと同じことを、documentに書いたせいで、そのdocumentが陳腐化してハルシネーションやuserレビューコスト増大や混乱ほか様々なトラブル原因になる」を防止する
    - なおissue-notes/は、userがissueごとの意図を記録する用途で使う
- 日本語
  - PRコメントは日本語で書く

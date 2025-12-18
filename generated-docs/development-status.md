Last updated: 2025-12-19

# Development Status

## 現在のIssues
- [Issue #23](../issue-notes/23.md) Windows GNUクロスコンパイルチェックが失敗しており、ビルドエラーの調査と依存関係の互換性確保が必要です。
- [Issue #20](../issue-notes/20.md) プレビュー演奏機能はカーソル位置までではなく、JSONデータ全体を演奏対象とするように変更する必要があります。
- [Issue #19](../issue-notes/19.md) `addr 0x08 KEYON` 表示について、`data`のbit3,4,5,6が全て0の場合には`KEYOFF`と表示するロジックを実装する必要があります。

## 次の一手候補
1. [Issue #23](../issue-notes/23.md): Windows GNUクロスコンパイル失敗の原因調査と修正
   - 最初の小さな一歩: 失敗したワークフローの実行ログ `https://github.com/cat2151/ym2151-log-editor/actions/runs/20340786044` を詳細に分析し、特定のエラーメッセージやスタックトレースから根本原因を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: .github/workflows/call-rust-windows-check.yml, Cargo.toml, Cargo.lock, (および過去のワークフロー実行ログ)

     実行内容: 過去の「Windows GNU cross-compilation check」ワークフロー（特に https://github.com/cat2151/ym2151-log-editor/actions/runs/20340786044 の失敗ログ）を詳細に分析し、ビルドエラーの根本原因（例: 依存関係の欠落、コンパイルフラグの問題、特定のcrateの互換性問題など）を特定してください。

     確認事項: ワークフローが利用しているrust-toolchainのバージョン、target triple、Cargo.tomlで定義されている依存関係、およびGitHub Actionsのランナー環境を確認してください。

     期待する出力: エラーの根本原因、およびその修正のための具体的な提案（例: Cargo.tomlの変更案、ワークフロー設定の調整案、特定の依存関係のアップデートまたはダウングレード）をMarkdown形式で出力してください。
     ```

2. [Issue #19](../issue-notes/19.md): addr 0x08 KEYON表示の改善
   - 最初の小さな一歩: `src/ui.rs` または `src/models.rs` 内で `addr 0x08` のデータ表示ロジックを特定し、`data`のbit3,4,5,6が0であるかを判定する条件を追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ui.rs, src/models.rs (必要であれば src/preview.rs)

     実行内容: `ym2151-log-editor` アプリケーション内で、`addr 0x08`（OPMレジスタのアドレス）に対するデータ表示ロジックを分析してください。特に `KEYON/KEYOFF` の表示に関連する部分に着目し、`data` のbit3からbit6がすべて0である場合に「KEYOFF」と表示するよう修正案を提案してください。

     確認事項: 現在の `KEYON/KEYOFF` 表示がどの関数で処理されているか、および `data` の値がどのように取得・解釈されているかを確認してください。既存の表示ロジックとの整合性を保つこと。

     期待する出力: `src/ui.rs` または `src/models.rs` の修正差分をMarkdown形式のコードブロックで出力し、変更内容の詳細な説明を加えてください。
     ```

3. [Issue #20](../issue-notes/20.md): preview演奏機能をJSON全体に拡張
   - 最初の小さな一歩: `src/preview.rs` を調査し、現在のプレビュー演奏機能がどのように演奏範囲（カーソル位置など）を決定しているかを特定し、JSONデータ全体を対象とするための修正点を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/preview.rs, src/app.rs (演奏トリガー関連)

     実行内容: `ym2151-log-editor` のプレビュー演奏機能が、現在カーソル位置からの演奏に限定されているか、またはJSON全体を演奏するロジックに変更可能かを `src/preview.rs` を中心に分析してください。JSON全体を演奏対象とするための具体的な修正箇所と実装方針を提案してください。

     確認事項: プレビュー演奏を開始するトリガー（UIイベント）、演奏データを処理する部分、および演奏範囲を決定する変数の使われ方を確認してください。既存のアプリケーションフローを壊さないように注意してください。

     期待する出力: プレビュー演奏機能をJSON全体に拡張するための `src/preview.rs` のコード変更案、または変更が必要な他のファイル（`src/app.rs`など）とその修正方針をMarkdown形式で出力してください。

---
Generated at: 2025-12-19 07:04:45 JST

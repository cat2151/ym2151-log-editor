Last updated: 2025-12-17


# プロジェクト概要生成プロンプト（来訪者向け）

## 生成するもの：
- projectを3行で要約する
- プロジェクトで使用されている技術スタックをカテゴリ別に整理して説明する
- プロジェクト全体のファイル階層ツリー（ディレクトリ構造を図解）
- プロジェクト全体のファイルそれぞれの説明
- プロジェクト全体の関数それぞれの説明
- プロジェクト全体の関数の呼び出し階層ツリー

## 生成しないもの：
- Issues情報（開発者向け情報のため）
- 次の一手候補（開発者向け情報のため）
- ハルシネーションしそうなもの（例、存在しない機能や計画を勝手に妄想する等）

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Project Overview

## プロジェクト概要
[以下の形式で3行でプロジェクトを要約]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 技術スタック
[使用している技術をカテゴリ別に整理して説明]
- フロントエンド: [フロントエンド技術とその説明]
- 音楽・オーディオ: [音楽・オーディオ関連技術とその説明]
- 開発ツール: [開発支援ツールとその説明]
- テスト: [テスト関連技術とその説明]
- ビルドツール: [ビルド・パース関連技術とその説明]
- 言語機能: [言語仕様・機能とその説明]
- 自動化・CI/CD: [自動化・継続的統合関連技術とその説明]
- 開発標準: [コード品質・統一ルール関連技術とその説明]

## ファイル階層ツリー
```
[プロジェクトのディレクトリ構造をツリー形式で表現]
```

## ファイル詳細説明
[各ファイルの役割と機能を詳細に説明]

## 関数詳細説明
[各関数の役割、引数、戻り値、機能を詳細に説明]

## 関数呼び出し階層ツリー
```
[関数間の呼び出し関係をツリー形式で表現]
```
```


以下のプロジェクト情報を参考にして要約を生成してください：

## プロジェクト情報
名前: 
説明: # ym2151-log-editor

YM2151イベントログエディタ（TUIインターフェース）。Rust製。

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## 概要

YM2151シンセサイザーのイベントログ用ターミナルベースのJSONエディタです。YM2151イベントデータの可視化と編集を支援し、特にタイミング調整とイベント検査に焦点を当てています。

## 機能

- **JSON可視化**: YM2151イベントログを読みやすい形式で表示
- **KeyON表示**: レジスタ0x08のイベントを「KeyON」として表示し、わかりやすくします
- **時刻表示モード**: 累積時間（wait）と絶対タイムスタンプを切り替え
  - 累積モード: イベント間のデルタ時間を表示（編集に便利）
  - タイムスタンプモード: 開始からの絶対時間を表示（内部ストレージ形式）
- **ナビゲーション**: 矢印キーでイベントを閲覧
- **ファイル操作**: JSONファイルの読み込みと保存

## クイックスタート

### インストール

Rust 1.70以降が必要です。

```bash
# リポジトリのクローン
git clone https://github.com/cat2151/ym2151-log-editor.git
cd ym2151-log-editor

# ビルド
cargo build --release

# JSONファイルを指定して実行
cargo run -- path/to/your/file.json
```

### 使用方法

```bash
# ファイルを指定してエディタを起動
./target/release/ym2151-log-editor your_log.json
```

### キーボード操作

| キー | 動作 |
|-----|-----|
| ↑/↓ | イベント間を移動 |
| / または ENTER | 現在行の前に新しいイベントを挿入 |
| DELETE | 現在のイベントを削除 |
| 0-9 | 待機時間を設定（0-9ミリ秒、累積モードのみ） |
| P | 開始から現在イベントまでのプレビュー再生 |
| T | 時刻表示モードの切り替え（累積 ↔ タイムスタンプ） |
| S | ファイル保存 |
| Q または ESC | アプリケーション終了 |

## JSON形式

エディタはJSON形式のYM2151イベントログを扱います：

```json
{
  "events": [
    {
      "time": 0.0,
      "addr": "20",
      "data": "4F"
    },
    {
      "time": 0.01,
      "addr": "08",
      "data": "78"
    }
  ]
}
```

- `time`: 絶対タイムスタンプ（秒単位）
- `addr`: YM2151レジスタアドレス（16進数）
- `data`: レジスタデータ値（16進数）

## 時刻表示モード

### 累積モード（デフォルト）
前のイベントからの待ち時間（デルタ）を表示します。タイミングを編集する際に便利で、イベント間の遅延を確認および調整できます。

例：
```
0.000000  20  4F    ← 最初のイベント（時刻0）
0.010000  40  16    ← 前から10ms後
0.010000  KeyON  78 ← 前から10ms後
```

### タイムスタンプモード
開始からの絶対時間を表示します。これはファイル保存時に使用される内部形式です。

例：
```
0.000000  20  4F    ← 開始から0ms
0.010000  40  16    ← 開始から10ms
0.020000  KeyON  78 ← 開始から20ms
```

**T**キーを押してこれらのモードを切り替えます。

## KeyON表示

レジスタ0x08（KeyON/KeyOFFレジスタ）のイベントは、読みやすさを向上させるため「08」の代わりに「KeyON」として表示されます：

```
0.010000  KeyON  78  ← キーオン/オフイベントを簡単に識別
0.500000  KeyON  00
```

## 開発

### プロジェクト構造

```
src/
├── main.rs       - エントリーポイントとイベントループ
├── app.rs        - アプリケーション状態とロジック
├── models.rs     - データ構造（Ym2151Event、Ym2151Log）
└── ui.rs         - UI描画
```

### ビルド

```bash
cargo build          # 開発用ビルド
cargo build --release # 最適化ビルド
```

### テスト

`test_data/sample.json`にサンプルテストデータが用意されています：

```bash
cargo run -- test_data/sample.json
```

## ドキュメント

詳細な実装計画と今後のロードマップについては[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)を参照してください。

## 依存関係

- [ratatui](https://ratatui.rs/) 0.29 - ターミナルUIフレームワーク
- [crossterm](https://github.com/crossterm-rs/crossterm) 0.29 - ターミナルバックエンド
- [serde](https://serde.rs/) 1.0 - シリアライゼーションフレームワーク
- [serde_json](https://github.com/serde-rs/json) 1.0 - JSONサポート

## 関連プロジェクト

- [ym2151-tone-editor](https://github.com/cat2151/ym2151-tone-editor) - YM2151音色エディタ（参考実装）
- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server) - YM2151ログ再生サーバー

## ライセンス

詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 今後の拡張機能

- 挿入したイベントのレジスタアドレスとデータ値の編集
- アンドゥ/リドゥ機能
- オーディオプレビュー統合
- 一括時間スケーリング
- イベントフィルタリングと検索

完全なロードマップについては[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)を参照してください。

## projectが目指すもの
- waitの最低限のedit
- eventの最低限のedit
- eventの最低限の可視化

## projectが目指さないもの（スコープ外）
- 高機能。
  - MML入力によるevent挿入。
  - 高度な可視化。8chの並列表示。DAWのevent editorにある全ての機能以上のものを実現。
  - DAW機能。ピアノロール表示と編集。五線譜表示と編集。dataのオートメーション表示と編集。MIDI INによるevent挿入。
  - query。eventをqueryすることで実現できる高度なedit。noteだけ、指定eventだけ、等をfilterしてeditし、かつevent増減でもevent間の依存関係を崩さない高度にインテリジェントな自動event依存関係認識。
  - 逆コンパイル。event内容を解析し、SMFやMMLへの高度にインテリジェントな逆変換。timeからBPMとmeasure、beat、tickを算出し、その成功率100%。


依存関係:
{}

## ファイル階層ツリー
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📖 IMPLEMENTATION_PLAN.md
📄 LICENSE
📖 README.ja.md
📖 README.md
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 19.md
  📖 20.md
  📖 21.md
📁 src/
  📄 app.rs
  📄 event_editor.rs
  📄 file_io.rs
  📄 main.rs
  📄 models.rs
  📄 navigation.rs
  📄 preview.rs
  📄 time_display.rs
  📄 ui.rs
📁 test_data/
  📊 minimal.json
  📊 sample.json

## ファイル詳細分析
**googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

## 関数呼び出し階層
関数呼び出し階層を分析できませんでした

## プロジェクト構造（ファイル一覧）
IMPLEMENTATION_PLAN.md
README.ja.md
README.md
googled947dc864c270e07.html
issue-notes/19.md
issue-notes/20.md
issue-notes/21.md
test_data/minimal.json
test_data/sample.json

上記の情報を基に、プロンプトで指定された形式でプロジェクト概要を生成してください。
特に以下の点を重視してください：
- 技術スタックは各カテゴリごとに整理して説明
- ファイル階層ツリーは提供された構造をそのまま使用
- ファイルの説明は各ファイルの実際の内容と機能に基づく
- 関数の説明は実際に検出された関数の役割に基づく
- 関数呼び出し階層は実際の呼び出し関係に基づく


---
Generated at: 2025-12-17 07:04:30 JST

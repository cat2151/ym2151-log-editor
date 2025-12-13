# ym2151-log-editor

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

- 時刻値の編集（増減）
- レジスタアドレスとデータ値の編集
- イベントの挿入/削除
- アンドゥ/リドゥ機能
- オーディオプレビュー統合
- 一括時間スケーリング
- イベントフィルタリングと検索

完全なロードマップについては[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)を参照してください。
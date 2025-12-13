# ym2151-log-editor

Jumpstartのゴール：
　土台が作成されること。参考：　https://github.com/cat2151/ym2151-tone-editor/

　できれば実装計画書が作成されること。

　できれば最初のシンプルなRust実装がされること。

アプリ概要：
　JSON editor。TUI。Rust。Windows。利用Rustクレートは https://github.com/cat2151/ym2151-tone-editor/ と同様。
　JSON内容はYM2151 event で、https://github.com/cat2151/ym2151-tone-editor/ を参照のこと。
　JSON表示、 YM2151 event部の 0x00～0xFF をKeyONに変換して表示。
　JSONのtime部分の表示は、累積時間と、時刻とを、Tキーでtoggle切り替えできる。
　JSON保存時は、内部データである時刻で保存する。
　アプリの主な用途は、JSON可視化と、time部分をwaitとして編集するときにwait増減を楽にする用。

Last updated: 2025-12-16

# 開発状況生成プロンプト（開発者向け）

## 生成するもの：
- 現在openされているissuesを3行で要約する
- 次の一手の候補を3つlistする
- 次の一手の候補3つそれぞれについて、極力小さく分解して、その最初の小さな一歩を書く

## 生成しないもの：
- 「今日のissue目標」などuserに提案するもの
  - ハルシネーションの温床なので生成しない
- ハルシネーションしそうなものは生成しない（例、無価値なtaskや新issueを勝手に妄想してそれをuserに提案する等）
- プロジェクト構造情報（来訪者向け情報のため、別ファイルで管理）

## 「Agent実行プロンプト」生成ガイドライン：
「Agent実行プロンプト」作成時は以下の要素を必ず含めてください：

### 必須要素
1. **対象ファイル**: 分析/編集する具体的なファイルパス
2. **実行内容**: 具体的な分析や変更内容（「分析してください」ではなく「XXXファイルのYYY機能を分析し、ZZZの観点でmarkdown形式で出力してください」）
3. **確認事項**: 変更前に確認すべき依存関係や制約
4. **期待する出力**: markdown形式での結果や、具体的なファイル変更

### Agent実行プロンプト例

**良い例（上記「必須要素」4項目を含む具体的なプロンプト形式）**:
```
対象ファイル: `.github/workflows/translate-readme.yml`と`.github/workflows/call-translate-readme.yml`

実行内容: 対象ファイルについて、外部プロジェクトから利用する際に必要な設定項目を洗い出し、以下の観点から分析してください：
1) 必須入力パラメータ（target-branch等）
2) 必須シークレット（GEMINI_API_KEY）
3) ファイル配置の前提条件（README.ja.mdの存在）
4) 外部プロジェクトでの利用時に必要な追加設定

確認事項: 作業前に既存のworkflowファイルとの依存関係、および他のREADME関連ファイルとの整合性を確認してください。

期待する出力: 外部プロジェクトがこの`call-translate-readme.yml`を導入する際の手順書をmarkdown形式で生成してください。具体的には：必須パラメータの設定方法、シークレットの登録手順、前提条件の確認項目を含めてください。
```

**避けるべき例**:
- callgraphについて調べてください
- ワークフローを分析してください
- issue-noteの処理フローを確認してください

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Development Status

## 現在のIssues
[以下の形式で3行でオープン中のissuesを要約。issue番号を必ず書く]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 次の一手候補
1. [候補1のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

2. [候補2のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

3. [候補3のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```
```


# 開発状況情報
- 以下の開発状況情報を参考にしてください。
- Issue番号を記載する際は、必ず [Issue #番号](../issue-notes/番号.md) の形式でMarkdownリンクとして記載してください。

## プロジェクトのファイル一覧
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
- .github/actions-tmp/.github/workflows/rust-windows-check.yml
- .github/actions-tmp/.github/workflows/translate-readme.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/callgraph.ql
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/codeql-pack.lock.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/qlpack.yml
- .github/actions-tmp/.github_automation/callgraph/config/example.json
- .github/actions-tmp/.github_automation/callgraph/docs/callgraph.md
- .github/actions-tmp/.github_automation/callgraph/presets/callgraph.js
- .github/actions-tmp/.github_automation/callgraph/presets/style.css
- .github/actions-tmp/.github_automation/callgraph/scripts/analyze-codeql.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/callgraph-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-codeql-exists.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-node-version.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/common-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/copy-commit-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/extract-sarif-info.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/find-process-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generate-html-graph.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generateHTML.cjs
- .github/actions-tmp/.github_automation/check_recent_human_commit/scripts/check-recent-human-commit.cjs
- .github/actions-tmp/.github_automation/project_summary/docs/daily-summary-setup.md
- .github/actions-tmp/.github_automation/project_summary/prompts/development-status-prompt.md
- .github/actions-tmp/.github_automation/project_summary/prompts/project-overview-prompt.md
- .github/actions-tmp/.github_automation/project_summary/scripts/ProjectSummaryCoordinator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/GitUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/IssueTracker.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/generate-project-summary.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/CodeAnalyzer.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectAnalysisOrchestrator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataCollector.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataFormatter.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectOverviewGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/BaseGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/FileSystemUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/ProjectFileUtils.cjs
- .github/actions-tmp/.github_automation/translate/docs/TRANSLATION_SETUP.md
- .github/actions-tmp/.github_automation/translate/scripts/translate-readme.cjs
- .github/actions-tmp/.gitignore
- .github/actions-tmp/.vscode/settings.json
- .github/actions-tmp/LICENSE
- .github/actions-tmp/README.ja.md
- .github/actions-tmp/README.md
- .github/actions-tmp/_config.yml
- .github/actions-tmp/generated-docs/callgraph.html
- .github/actions-tmp/generated-docs/callgraph.js
- .github/actions-tmp/generated-docs/development-status-generated-prompt.md
- .github/actions-tmp/generated-docs/development-status.md
- .github/actions-tmp/generated-docs/project-overview-generated-prompt.md
- .github/actions-tmp/generated-docs/project-overview.md
- .github/actions-tmp/generated-docs/style.css
- .github/actions-tmp/googled947dc864c270e07.html
- .github/actions-tmp/issue-notes/10.md
- .github/actions-tmp/issue-notes/11.md
- .github/actions-tmp/issue-notes/12.md
- .github/actions-tmp/issue-notes/13.md
- .github/actions-tmp/issue-notes/14.md
- .github/actions-tmp/issue-notes/15.md
- .github/actions-tmp/issue-notes/16.md
- .github/actions-tmp/issue-notes/17.md
- .github/actions-tmp/issue-notes/18.md
- .github/actions-tmp/issue-notes/19.md
- .github/actions-tmp/issue-notes/2.md
- .github/actions-tmp/issue-notes/20.md
- .github/actions-tmp/issue-notes/21.md
- .github/actions-tmp/issue-notes/22.md
- .github/actions-tmp/issue-notes/23.md
- .github/actions-tmp/issue-notes/24.md
- .github/actions-tmp/issue-notes/25.md
- .github/actions-tmp/issue-notes/26.md
- .github/actions-tmp/issue-notes/27.md
- .github/actions-tmp/issue-notes/28.md
- .github/actions-tmp/issue-notes/29.md
- .github/actions-tmp/issue-notes/3.md
- .github/actions-tmp/issue-notes/30.md
- .github/actions-tmp/issue-notes/4.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-rust-windows-check.yml
- .github/workflows/call-translate-readme.yml
- .gitignore
- Cargo.lock
- Cargo.toml
- IMPLEMENTATION_PLAN.md
- LICENSE
- README.ja.md
- README.md
- generated-docs/project-overview-generated-prompt.md
- googled947dc864c270e07.html
- src/app.rs
- src/main.rs
- src/models.rs
- src/ui.rs
- test_data/minimal.json
- test_data/sample.json

## 現在のオープンIssues
## [Issue #16](../issue-notes/16.md): Refactor app.rs into single-responsibility modules
The `app.rs` module violated the Single Responsibility Principle by handling application state, file I/O, navigation, time calculations, event editing, and preview functionality in a single 690-line file.

## Changes

Split `app.rs` into focused modules:

- **`file_io.rs`** - JSON serialization (loa...
ラベル: 
--- issue-notes/16.md の内容 ---

```markdown

```

## [Issue #15](../issue-notes/15.md): app.rs をリファクタリングし、単一責任の原則に従って分割する

ラベル: 
--- issue-notes/15.md の内容 ---

```markdown

```

## [Issue #2](../issue-notes/2.md): ドッグフーディングする

ラベル: 
--- issue-notes/2.md の内容 ---

```markdown

```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/15.md
```md
{% raw %}
# issue project_summary scripts cjs を分解し、できるだけ1ファイル200行未満にし、agentによるメンテをしやすくする #15
[issues #15](https://github.com/cat2151/github-actions/issues/15)

# 状況
- agentに、最初の小さな一歩のAgent実行プロンプトを実行させた
- 結果、以下を得た：
    - project_summary_cjs_analysis.md
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
- 結果
    - 生成させたpromptをレビューした
    - promptを修正した
    - agentに投げた
    - 結果、GitUtils.cjsを切り出しできた
    - それをリファクタリングミスがないかチェックさせた
    - agentによるチェック結果は合格だった
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
- 結果
    - 生成させたpromptをレビューした
        - promptの対象ファイルから project_summary_cjs_analysis.md が漏れていることがわかったので修正した
    - promptを修正した
    - agentに投げた
    - 結果、FileSystemUtils.cjsを切り出しできた
    - それをリファクタリングミスがないかチェックさせた
    - agentによるチェック結果は合格だった
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
- 結果
    - 生成させたpromptをレビューした
    - 今回は低品質、NG、と判断した
    - 判断基準は、project_summary_cjs_analysis.md と乖離してしまっている点。今回はハルシネーションを含んだplanである、と判断した
    - 人力でpromptを書き、planさせ、plan結果をレビューし、agentに投げた
    - 結果、CodeAnalyzer.cjsとProjectAnalyzer.cjsを切り出しできた
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
    - 備考、課題、Geminiに生成させているdocumentは2つある。かなり位置づけが違うものである。
        - projectのソースファイル分析。
        - projectのissues分析。
        - この2つについて、class, cjs, yml まで分割をするかを、あとで検討する。
        - おそらく、class分割どまりとし、ソースファイル分析結果をissues分析の参考資料としてGeminiのcontextに与える改善をする、がよい、と想定しておく。
- 課題、エラーで落ちた。昨日は落ちてない。
    - 原因、昨日のagentのリファクタリング時に、ハルシネーションで、
        - codeが破壊されていた
        - run メソッドが削除されていた
        - 一つ前のrevisionにはrun メソッドがあった
        - ほかにもcode破壊があったのかは不明、調査省略、明日の日次バッチをtestと調査として利用するつもり
- どうする？
    - 単純に一つ前のrevisionからrun メソッドを復活させ、明日の日次バッチをtestと調査として利用する
- 再発防止策は？
    - ノーアイデア。昨日それなりにagentにチェックをさせたはずだが根本的な大きなミスが発生していた。
    - 構文チェックは通っていたが、問題を検知できなかった。
    - チェックが機能していない、あるいは機能として不足している。
    - 分析。変更量が大きかったぶんミスのリスクが増えていた。
    - 対策案。もっと小さく一歩ずつ変更させる。
    - 対策案。リファクタリング時、いきなりメソッド削除をさせない。
        - まず全cjsの全メソッドのlistをさせる。
        - のち、削除対象の重複メソッドのlistをさせる。
        - そして削除planをさせる。
        - のち、削除させる。
        - さらに削除後のメソッドlistをさせる。
        - そして削除しすぎていないかを削除前後のlist比較でチェックさせる。
        - これでrunまで削除してしまうのを防止できるかもしれない。
        - これは人力からみると、おかしな話である。人力なら1つずつ移動をするだけであり、ミスのしようがない。
        - LLMの典型的なハルシネーション問題の一つである、と認識する
- 結果は？
    - test green
    - run メソッドの人力復活は成功した
    - 日次バッチで生成した次の一手のpromptを投げた
    - リファクタリング成功した。ProjectSummaryGenerator を切り出した
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - 先に、2つのdocument生成を、1つずつ生成できるよう疎結合にリファクタリング、をしたほうがよさそう
    - agentにそれを投げた
    - 成功した、と判断する
    - 課題、`BaseSummaryGenerator.cjs` は、baseの機能と、`ProjectOverviewGenerator.cjs`専用の機能とが混ざっている。
        - baseに集約すべきは、`ProjectSummaryCoordinator.cjs`と`ProjectOverviewGenerator.cjs`とが必ずどちらも使う機能、である、と考える。
        - 対策、明日以降それをagentに投げる
    - `project_summary_cjs_analysis.md` は削除とする。役目が完了した、と判断する。リファクタリング前のソース構造の分析documentであり、今は存在しているとわかりづらくなる。シンプル優先のため削除とする。
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - `BaseSummaryGenerator.cjs` を切り出したのは成功した、と判断する
    - `BaseSummaryGenerator.cjs` を2分割するため、agentにplanさせた
    - レビューした
    - agentに2分割させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - `BaseSummaryGenerator.cjs` を2分割は成功した、と判断する
    - issue track機能構造をリファクタリングし、以下にする
        - development status generator : baseを継承する
        - issue tracker : 汎用関数群
    - agentに実施させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - DevelopmentStatusGeneratorとissue trackerのリファクタリングは成功した、と判断する
    - ProjectOverview生成機能のリファクタリングをする
    - agentに実施させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - ProjectOverview生成機能のリファクタリングは成功した、と判断する
    - 課題、overviewと、developmentStatusとが混在し、dirが読みづらい。
    - 対策、shared/、overview/、development/、の3つのdirに切り分ける
    - agentに分析、planさせ、レビューし、planさせ、実施させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - shared/、overview/、development/、の3つのdirに切り分けるリファクタリングは成功した、と判断する
    - agentに、agentがメンテしやすいか？の観点からレビューさせた
    - 詳細は割愛
        - `> 最優先で取り組むべきは 設定管理の一元化 と エラーハンドリングの統一 です。これにより、Agentにとって予測可能で理解しやすいコードベースになります。`
        - それは別issueで、設定変更をマストでやるので、OKと判断する
- これでagentによるメンテは十分しやすくなった、と判断する
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/16.md
```md
{% raw %}
# issue issue-note / project-summary / translate / callgraph をtonejs-mml-to-jsonから呼び出す #16
[issues #16](https://github.com/cat2151/github-actions/issues/16)

# これまでの課題
- issue-note / project-summary / translate / callgraph は、github-actions リポジトリ上ではtest greenである。
- だが他のリポジトリにおいて動作するか？が可視化不足である。

# 対策
- issue-note / project-summary / translate / callgraph をtonejs-mml-to-jsonから呼び出す
- 詳しく
    - まず、現状、tonejs-mml-to-json でその4つのworkflowがどうなっているか、このmdに可視化する
    - 例えば、既に呼び出している、呼び出していない、tonejs-mml-to-jsonにある古いworkflowを呼び出している

# 調査結果
- まず、現状、tonejs-mml-to-json でその4つのworkflowがどうなっているか、このmdに可視化する
    - 結果：
        - issue-note
            - tonejs-mml-to-jsonにある古いworkflowを呼び出している
        - project-summary
            - tonejs-mml-to-jsonにある古いworkflowを呼び出している
        - translate
            - tonejs-mml-to-jsonにある古いworkflowを呼び出している
        - callgraph
            - tonejs-mml-to-jsonにある古いworkflowを呼び出している

# どうする？
- issue-note
    - github-actions リポジトリにある、call-issue-note.yml をcpして使うようにする、まず単純cpして動くかを確認する
- project-summary
    - github-actions リポジトリにある、call-daily-project-summary.yml をcpして使うようにする、まず単純cpして動くかを確認する
- translate
    - github-actions リポジトリにある、call-translate-readme.yml をcpして使うようにする、まず単純cpして動くかを確認する
- callgraph
    - github-actions リポジトリにある、call-callgraph.yml をcpして使うようにする、まず単純cpして動くかを確認する

# 状況
- issue-note
    - tonejs-mml-to-jsonリポジトリにて、test green
    - issue-noteについては当issueのタスクは完了した、と判断する
- project-summary
    - tonejs-mml-to-jsonリポジトリにて、test green
    - project-summaryについては当issueのタスクは完了した、と判断する

# 状況
- translate
    - github-actions リポジトリにある、call-translate-readme.yml をcpして使うようにする、まず単純cpして動くかを確認する
        - 状況
            - 単純cpした
            - ソース机上レビューした。OK
            - トリガーはREADME.ja.mdのcommit
            - testは省略とする
            - もし今後README.ja.mdのcommit時にうまく動作しないとしても、そのとき対処すればOK、と判断する
    - translateについては当issueのタスクは完了した、と判断する

# どうする？
- callgraph
    - github-actions リポジトリにある、call-callgraph.yml をcpして使うようにする、まず単純cpして動くかを確認する

# 結果
- callgraph
    - tonejs-mml-to-jsonリポジトリにて、test red
    - logをみても情報不足なため、まずloggerを修正する
    - 結果、わかった、運用ミス、対象srcの指定の考慮漏れ
    - どうする？
        - 対象srcを指定する。tonejs-mml-to-jsonリポジトリにて進める
    - 結果
        - test green
    - callgraphについては当issueのタスクは完了した、と判断する

# 状況
- github-actions以外のリポジトリとして、
    - tonejs-mml-to-jsonリポジトリにおいて、
        - issue-note / project-summary / translate / callgraph がtest greenとなった。
        - closeできる、と判断する。

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/2.md
```md
{% raw %}
# issue GitHub Actions「関数コールグラフhtmlビジュアライズ生成」を共通ワークフロー化する #2
[issues #2](https://github.com/cat2151/github-actions/issues/2)


# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-callgraph_enhanced.yml
まずplanしてください
```

# 結果
- indent
    - linter？がindentのエラーを出しているがyml内容は見た感じOK
    - テキストエディタとagentの相性問題と判断する
    - 別のテキストエディタでsaveしなおし、テキストエディタをreload
    - indentのエラーは解消した
- LLMレビュー
    - agent以外の複数のLLMにレビューさせる
    - prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューしてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 共通ワークフロー

# GitHub Actions Reusable Workflow for Call Graph Generation
name: Generate Call Graph

# TODO Windowsネイティブでのtestをしていた名残が残っているので、今後整理していく。今はWSL act でtestしており、Windowsネイティブ環境依存問題が解決した
#  ChatGPTにレビューさせるとそこそこ有用そうな提案が得られたので、今後それをやる予定
#  agentに自己チェックさせる手も、セカンドオピニオンとして選択肢に入れておく

on:
  workflow_call:

jobs:
  check-commits:
    runs-on: ubuntu-latest
    outputs:
      should-run: ${{ steps.check.outputs.should-run }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 50 # 過去のコミットを取得

      - name: Check for user commits in last 24 hours
        id: check
        run: |
          node .github/scripts/callgraph_enhanced/check-commits.cjs

  generate-callgraph:
    needs: check-commits
    if: needs.check-commits.outputs.should-run == 'true'
    runs-on: ubuntu-latest
    permissions:
      contents: write
      security-events: write
      actions: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Set Git identity
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

      - name: Remove old CodeQL packages cache
        run: rm -rf ~/.codeql/packages

      - name: Check Node.js version
        run: |
          node .github/scripts/callgraph_enhanced/check-node-version.cjs

      - name: Install CodeQL CLI
        run: |
          wget https://github.com/github/codeql-cli-binaries/releases/download/v2.22.1/codeql-linux64.zip
          unzip codeql-linux64.zip
          sudo mv codeql /opt/codeql
          echo "/opt/codeql" >> $GITHUB_PATH

      - name: Install CodeQL query packs
        run: |
          /opt/codeql/codeql pack install .github/codeql-queries

      - name: Check CodeQL exists
        run: |
          node .github/scripts/callgraph_enhanced/check-codeql-exists.cjs

      - name: Verify CodeQL Configuration
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs verify-config

      - name: Remove existing CodeQL DB (if any)
        run: |
          rm -rf codeql-db

      - name: Perform CodeQL Analysis
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs analyze

      - name: Check CodeQL Analysis Results
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs check-results

      - name: Debug CodeQL execution
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs debug

      - name: Wait for CodeQL results
        run: |
          node -e "setTimeout(()=>{}, 10000)"

      - name: Find and process CodeQL results
        run: |
          node .github/scripts/callgraph_enhanced/find-process-results.cjs

      - name: Generate HTML graph
        run: |
          node .github/scripts/callgraph_enhanced/generate-html-graph.cjs

      - name: Copy files to generated-docs and commit results
        run: |
          node .github/scripts/callgraph_enhanced/copy-commit-results.cjs

--- 呼び出し元
# 呼び出し元ワークフロー: call-callgraph_enhanced.yml
name: Call Call Graph Enhanced

on:
  schedule:
    # 毎日午前5時(JST) = UTC 20:00前日
    - cron: '0 20 * * *'
  workflow_dispatch:

jobs:
  call-callgraph-enhanced:
    # uses: cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
    uses: ./.github/workflows/callgraph_enhanced.yml # ローカルでのテスト用
```

# レビュー結果OKと判断する
- レビュー結果を人力でレビューした形になった

# test
- #4 同様にローカル WSL + act でtestする
- エラー。userのtest設計ミス。
  - scriptの挙動 : src/ がある前提
  - 今回の共通ワークフローのリポジトリ : src/ がない
  - 今回testで実現したいこと
    - 仮のソースでよいので、関数コールグラフを生成させる
  - 対策
    - src/ にダミーを配置する
- test green
  - ただしcommit pushはしてないので、html内容が0件NG、といったケースの検知はできない
  - もしそうなったら別issueとしよう

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする
- もしhtml内容が0件NG、などになったら、別issueとするつもり

{% endraw %}
```

### src/app.rs
```rs
{% raw %}
use crate::models::Ym2151Log;

/// Display mode for time values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeDisplayMode {
    /// Display cumulative time (delta from previous event)
    Cumulative,
    /// Display absolute timestamp
    Timestamp,
}

/// Application state
pub struct App {
    /// The loaded YM2151 log data
    pub log: Ym2151Log,
    /// Current file path (if any)
    pub file_path: Option<String>,
    /// Current scroll position
    pub scroll_offset: usize,
    /// Time display mode
    pub time_mode: TimeDisplayMode,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Selected event index
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            log: Ym2151Log { events: vec![] },
            file_path: None,
            scroll_offset: 0,
            time_mode: TimeDisplayMode::Cumulative,
            should_quit: false,
            selected_index: 0,
        }
    }

    /// Load a JSON file
    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        self.log = serde_json::from_str(&content)?;
        self.file_path = Some(path.to_string());
        self.selected_index = 0;
        self.scroll_offset = 0;
        Ok(())
    }

    /// Save the current log to file
    pub fn save_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.file_path {
            let content = serde_json::to_string_pretty(&self.log)?;
            std::fs::write(path, content)?;
            Ok(())
        } else {
            Err("No file path set".into())
        }
    }

    /// Toggle time display mode
    pub fn toggle_time_mode(&mut self) {
        self.time_mode = match self.time_mode {
            TimeDisplayMode::Cumulative => TimeDisplayMode::Timestamp,
            TimeDisplayMode::Timestamp => TimeDisplayMode::Cumulative,
        };
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        // Allow cursor to move to one position beyond the last event (for future insertion)
        if self.selected_index < self.log.events.len() {
            self.selected_index += 1;
        }
    }

    /// Update scroll offset to keep selected item visible
    pub fn update_scroll(&mut self, visible_height: usize) {
        if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index.saturating_sub(visible_height - 1);
        }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }

    /// Get cumulative time for an event (delta from previous)
    pub fn get_cumulative_time(&self, index: usize) -> f64 {
        if index == 0 {
            self.log.events[0].time
        } else if index < self.log.events.len() {
            self.log.events[index].time - self.log.events[index - 1].time
        } else {
            0.0
        }
    }

    /// Get formatted time string for an event
    pub fn get_time_string(&self, index: usize) -> String {
        if index >= self.log.events.len() {
            return String::from("0.000000");
        }

        let time = match self.time_mode {
            TimeDisplayMode::Timestamp => self.log.events[index].time,
            TimeDisplayMode::Cumulative => self.get_cumulative_time(index),
        };

        format!("{:.6}", time)
    }

    /// Format event for display
    pub fn format_event(&self, index: usize) -> String {
        if index >= self.log.events.len() {
            return String::new();
        }

        let event = &self.log.events[index];
        let time_str = self.get_time_string(index);

        if event.is_key_on() {
            format!("{}  KeyON  {}", time_str, event.data)
        } else {
            format!("{}  {}  {}", time_str, event.addr, event.data)
        }
    }

    /// Preview current event by playing events from start up to selected position
    #[cfg(windows)]
    pub fn preview_current_event(&self) {
        if self.log.events.is_empty() {
            return;
        }

        // Create a log containing events from start to current selection (inclusive)
        let end_index = self
            .selected_index
            .saturating_add(1)
            .min(self.log.events.len());
        let preview_events = self.log.events[0..end_index].to_vec();
        let preview_log = crate::models::Ym2151Log {
            events: preview_events,
        };

        // Convert to JSON and send to server
        if let Ok(json_string) = serde_json::to_string(&preview_log) {
            if let Err(e) = ym2151_log_play_server::client::send_json(&json_string) {
                eprintln!("Preview playback error: {}", e);
            }
        }
    }

    #[cfg(not(windows))]
    pub fn preview_current_event(&self) {
        // No-op on non-Windows platforms
    }

    /// Set wait time (cumulative time) for the selected event in milliseconds
    /// Only works in Cumulative display mode
    ///
    /// # Arguments
    /// * `milliseconds` - The wait time in milliseconds (typically 0-9).
    ///   Values are used as-is without validation. Common usage:
    ///   0-9ms (mapped from keys 0-9).
    pub fn set_wait_time_ms(&mut self, milliseconds: u32) {
        // Only allow modification in Cumulative mode
        if self.time_mode != TimeDisplayMode::Cumulative {
            return;
        }

        // Check if we have events and a valid selection
        if self.log.events.is_empty() || self.selected_index >= self.log.events.len() {
            return;
        }

        // Convert milliseconds to seconds
        let new_wait_time = (milliseconds as f64) / 1000.0;

        // Calculate the new absolute timestamp for the selected event
        let new_timestamp = if self.selected_index == 0 {
            // First event: set absolute time
            new_wait_time
        } else {
            // Other events: add wait time to previous event's timestamp
            self.log.events[self.selected_index - 1].time + new_wait_time
        };

        // Calculate the time delta (how much we're changing)
        let old_timestamp = self.log.events[self.selected_index].time;
        let time_delta = new_timestamp - old_timestamp;

        // Update the selected event's timestamp
        self.log.events[self.selected_index].time = new_timestamp;

        // Adjust all subsequent events' timestamps by the same delta
        for i in (self.selected_index + 1)..self.log.events.len() {
            self.log.events[i].time += time_delta;
        }
    }

    /// Delete the currently selected event
    pub fn delete_selected_event(&mut self) {
        // Check if we have events and a valid selection
        if self.log.events.is_empty() || self.selected_index >= self.log.events.len() {
            return;
        }

        // Remove the selected event
        self.log.events.remove(self.selected_index);

        // Adjust selected_index if it's now out of bounds
        if !self.log.events.is_empty() && self.selected_index >= self.log.events.len() {
            self.selected_index = self.log.events.len() - 1;
        }

        // Adjust scroll_offset if necessary
        if self.scroll_offset > self.selected_index {
            self.scroll_offset = self.selected_index;
        }
    }

    /// Insert a new event before the currently selected position
    pub fn insert_event_before_selected(&mut self) {
        // Calculate the timestamp for the new event
        let new_time = if self.selected_index == 0 {
            // Inserting before the first event: use time 0.0
            0.0
        } else if self.selected_index >= self.log.events.len() {
            // Inserting after all events: use last event's time
            self.log.events.last().map(|e| e.time).unwrap_or(0.0)
        } else {
            // Inserting between events: use previous event's time
            self.log.events[self.selected_index - 1].time
        };

        // Create a new default event
        let new_event = crate::models::Ym2151Event {
            time: new_time,
            addr: "00".to_string(),
            data: "00".to_string(),
        };

        // Insert the new event at the selected position
        // insert() can handle index == len(), which appends to the end
        self.log.events.insert(self.selected_index, new_event);

        // Keep the cursor on the newly inserted event (don't move selected_index)
        // Adjust scroll_offset if necessary to keep the new event visible
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ym2151Event;

    #[test]
    fn test_set_wait_time_ms() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Cumulative;

        // Create test events
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "60".to_string(),
                data: "14".to_string(),
            },
        ];

        // Select event 1 and set wait time to 5ms
        app.selected_index = 1;
        app.set_wait_time_ms(5);

        // Verify event 1 now has timestamp 0.005 (0.0 + 0.005)
        assert!((app.log.events[1].time - 0.005).abs() < 0.0001);

        // Verify event 2 was also adjusted (should be 0.015, was 0.02, delta = -0.005)
        assert!((app.log.events[2].time - 0.015).abs() < 0.0001);
    }

    #[test]
    fn test_set_wait_time_ms_timestamp_mode() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Timestamp;

        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        app.selected_index = 1;
        let original_time = app.log.events[1].time;

        // Should not modify in Timestamp mode
        app.set_wait_time_ms(5);

        assert_eq!(app.log.events[1].time, original_time);
    }

    #[test]
    fn test_set_wait_time_ms_first_event() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Cumulative;

        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Select first event and set wait time to 3ms
        app.selected_index = 0;
        app.set_wait_time_ms(3);

        // First event should be at 0.003
        assert!((app.log.events[0].time - 0.003).abs() < 0.0001);

        // Second event should also be adjusted (was 0.01, delta = +0.003)
        assert!((app.log.events[1].time - 0.013).abs() < 0.0001);
    }

    #[test]
    fn test_set_wait_time_ms_zero() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Cumulative;

        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "60".to_string(),
                data: "14".to_string(),
            },
        ];

        // Select event 1 and set wait time to 0ms
        app.selected_index = 1;
        app.set_wait_time_ms(0);

        // Verify event 1 now has timestamp 0.0 (same as previous event)
        assert!((app.log.events[1].time - 0.0).abs() < 0.0001);

        // Verify event 2 was also adjusted (should be 0.01, was 0.02, delta = -0.01)
        assert!((app.log.events[2].time - 0.01).abs() < 0.0001);
    }

    #[test]
    fn test_delete_selected_event() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "60".to_string(),
                data: "14".to_string(),
            },
        ];

        // Select middle event and delete it
        app.selected_index = 1;
        app.delete_selected_event();

        // Verify event count decreased
        assert_eq!(app.log.events.len(), 2);

        // Verify the correct event was deleted (remaining events should be index 0 and 2)
        assert_eq!(app.log.events[0].addr, "20");
        assert_eq!(app.log.events[1].addr, "60");

        // Verify selected_index is still valid
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_delete_last_event() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Select last event and delete it
        app.selected_index = 1;
        app.delete_selected_event();

        // Verify event count decreased
        assert_eq!(app.log.events.len(), 1);

        // Verify selected_index was adjusted to last valid index
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_delete_single_event() {
        let mut app = App::new();
        app.log.events = vec![Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        }];

        // Select the only event and delete it
        app.selected_index = 0;
        app.delete_selected_event();

        // Verify all events are deleted
        assert_eq!(app.log.events.len(), 0);

        // selected_index should remain 0 (though there are no events)
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_delete_empty_list() {
        let mut app = App::new();
        app.log.events = vec![];

        // Try to delete from empty list (should not panic)
        app.selected_index = 0;
        app.delete_selected_event();

        // Verify still empty
        assert_eq!(app.log.events.len(), 0);
    }

    #[test]
    fn test_move_down_to_empty_line() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Start at first event
        app.selected_index = 0;

        // Move down to second event
        app.move_down();
        assert_eq!(app.selected_index, 1);

        // Move down to empty line (one beyond last event)
        app.move_down();
        assert_eq!(app.selected_index, 2);
        assert_eq!(app.selected_index, app.log.events.len());

        // Try to move down again (should stay at empty line)
        app.move_down();
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn test_move_down_empty_log() {
        let mut app = App::new();
        app.log.events = vec![];

        // Start at index 0 (empty)
        app.selected_index = 0;

        // Try to move down (should stay at 0)
        app.move_down();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_insert_event_before_selected_at_start() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.01,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Insert before first event
        app.selected_index = 0;
        app.insert_event_before_selected();

        // Verify event count increased
        assert_eq!(app.log.events.len(), 3);

        // Verify new event inserted at position 0
        assert_eq!(app.log.events[0].addr, "00");
        assert_eq!(app.log.events[0].data, "00");
        assert!((app.log.events[0].time - 0.0).abs() < 0.0001);

        // Verify original events shifted
        assert_eq!(app.log.events[1].addr, "20");
        assert_eq!(app.log.events[2].addr, "40");

        // Verify selected_index stayed on the new event
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_insert_event_before_selected_in_middle() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "60".to_string(),
                data: "14".to_string(),
            },
        ];

        // Insert before middle event (index 1)
        app.selected_index = 1;
        app.insert_event_before_selected();

        // Verify event count increased
        assert_eq!(app.log.events.len(), 4);

        // Verify new event inserted at position 1 with time from previous event
        assert_eq!(app.log.events[1].addr, "00");
        assert_eq!(app.log.events[1].data, "00");
        assert!((app.log.events[1].time - 0.0).abs() < 0.0001);

        // Verify original events
        assert_eq!(app.log.events[0].addr, "20");
        assert_eq!(app.log.events[2].addr, "40");
        assert_eq!(app.log.events[3].addr, "60");

        // Verify selected_index stayed on the new event
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_insert_event_before_selected_at_end() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Move cursor to empty line after last event
        app.selected_index = 2;
        app.insert_event_before_selected();

        // Verify event count increased
        assert_eq!(app.log.events.len(), 3);

        // Verify new event inserted at position 2 with time from last event
        assert_eq!(app.log.events[2].addr, "00");
        assert_eq!(app.log.events[2].data, "00");
        assert!((app.log.events[2].time - 0.01).abs() < 0.0001);

        // Verify original events unchanged
        assert_eq!(app.log.events[0].addr, "20");
        assert_eq!(app.log.events[1].addr, "40");

        // Verify selected_index stayed at 2 (now pointing to the new event)
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn test_insert_event_before_selected_empty_list() {
        let mut app = App::new();
        app.log.events = vec![];

        // Insert into empty list
        app.selected_index = 0;
        app.insert_event_before_selected();

        // Verify event count increased
        assert_eq!(app.log.events.len(), 1);

        // Verify new event created with time 0.0
        assert_eq!(app.log.events[0].addr, "00");
        assert_eq!(app.log.events[0].data, "00");
        assert!((app.log.events[0].time - 0.0).abs() < 0.0001);

        // Verify selected_index is still 0
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_insert_event_scroll_adjustment() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Set scroll_offset ahead of selected_index
        app.selected_index = 0;
        app.scroll_offset = 1;

        app.insert_event_before_selected();

        // Verify scroll_offset was adjusted to keep new event visible
        assert_eq!(app.scroll_offset, 0);
    }
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
d1d8865 他project同様のCI設定
8e7406a google検索にindexさせる用
02ba847 Merge pull request #14 from cat2151/copilot/insert-event-before-line
ee43789 Simplify insert logic based on code review feedback
bb137d5 Update documentation for event insertion feature
02fee96 Implement event insertion feature with SLASH and ENTER keys
10ba63c Initial plan
926a400 Merge pull request #13 from cat2151/copilot/enable-cursor-movement-to-empty-line
dba3ec7 Enable cursor movement to empty line after last event
abed846 Initial plan

### 変更されたファイル:
.github/workflows/call-daily-project-summary.yml
.github/workflows/call-issue-note.yml
.github/workflows/call-rust-windows-check.yml
.github/workflows/call-translate-readme.yml
Cargo.lock
Cargo.toml
IMPLEMENTATION_PLAN.md
README.ja.md
README.md
googled947dc864c270e07.html
src/app.rs
src/main.rs
src/ui.rs
test_data/minimal.json


---
Generated at: 2025-12-16 07:04:33 JST

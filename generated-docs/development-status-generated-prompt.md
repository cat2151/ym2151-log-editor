Last updated: 2025-12-17

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
- issue-notes/19.md
- issue-notes/20.md
- issue-notes/21.md
- src/app.rs
- src/event_editor.rs
- src/file_io.rs
- src/main.rs
- src/models.rs
- src/navigation.rs
- src/preview.rs
- src/time_display.rs
- src/ui.rs
- test_data/minimal.json
- test_data/sample.json

## 現在のオープンIssues
## [Issue #21](../issue-notes/21.md): リファクタリング。app.rs の test は src/tests/ に切り出し、app.rs をコンパクトにして、今後の機能追加時のハルシネーションのリスクを下げる
[issue-notes/21.md](https://github.com/cat2151/ym2151-log-editor/blob/main/issue-notes/21.md)

...
ラベル: 
--- issue-notes/21.md の内容 ---

```markdown
# issue リファクタリング。app.rs の test は src/tests/ に切り出し、app.rs をコンパクトにして、今後の機能追加時のハルシネーションのリスクを下げる #21
[issues #21](https://github.com/cat2151/ym2151-log-editor/issues/21)



```

## [Issue #20](../issue-notes/20.md): preview演奏機能は、カーソル位置までの演奏ではなく、JSON全体を演奏とする
[issue-notes/20.md](https://github.com/cat2151/ym2151-log-editor/blob/main/issue-notes/20.md)

...
ラベル: 
--- issue-notes/20.md の内容 ---

```markdown
# issue preview演奏機能は、カーソル位置までの演奏ではなく、JSON全体を演奏とする #20
[issues #20](https://github.com/cat2151/ym2151-log-editor/issues/20)



```

## [Issue #19](../issue-notes/19.md): addr 0x08 KEYON表示について、dataのbit3,4,5,6が0であれば、KEYOFF表示とする
[issue-notes/19.md](https://github.com/cat2151/ym2151-log-editor/blob/main/issue-notes/19.md)

...
ラベル: 
--- issue-notes/19.md の内容 ---

```markdown
# issue addr 0x08 KEYON表示について、dataのbit3,4,5,6が0であれば、KEYOFF表示とする #19
[issues #19](https://github.com/cat2151/ym2151-log-editor/issues/19)



```

## [Issue #2](../issue-notes/2.md): ドッグフーディングする

ラベル: 
--- issue-notes/2.md の内容 ---

```markdown

```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/19.md
```md
{% raw %}
# issue project-summary の development-status 生成時、issue-notes/ 配下のmdファイルの内容を参照させる #19
[issues #19](https://github.com/cat2151/github-actions/issues/19)

# 何が困るの？
- issue解決に向けての次の一手の内容が実態に即していないことが多い。

# 対策案
- issue-notes/ 配下のmdファイルの内容を参照させる

# 備考
- さらにmd内に書かれているfileも、project内をcjsに検索させて添付させると、よりGeminiの生成品質が向上する可能性がある。
    - [issues #20](https://github.com/cat2151/github-actions/issues/20)
- さらにproject overviewでGeminiがまとめたmdも、Geminiに与えると、よりGeminiの生成品質が向上する可能性がある。
    - [issues #21](https://github.com/cat2151/github-actions/issues/21)
- さらに、Geminiに与えたpromptをfileにしてcommit pushしておくと、デバッグに役立つ可能性がある。
    - [issues #22](https://github.com/cat2151/github-actions/issues/22)

# close条件
- issues #22 がcloseされること。
- commitされたpromptを確認し、issue-notes/ 配下のmdファイルがpromptに添付されていること、が確認できること。

# 状況
- 課題、実装したがtestができていない
- 対策、issues #22 が実装されれば、testができる
- 対策、issues #22 のcloseを待つ

# 状況
- issues #22 がcloseされた
- testできるようになった
- commitされたpromptを確認した。issue-notes/ 配下のmdファイルがpromptに添付されていること、が確認できた

# closeする

{% endraw %}
```

### issue-notes/19.md
```md
{% raw %}
# issue addr 0x08 KEYON表示について、dataのbit3,4,5,6が0であれば、KEYOFF表示とする #19
[issues #19](https://github.com/cat2151/ym2151-log-editor/issues/19)



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

### .github/actions-tmp/issue-notes/20.md
```md
{% raw %}
# issue project-summary の development-status 生成時、issue-notes/ 配下のmdにファイル名が書いてあれば、そのファイル内容もpromptに添付、を試す #20
[issues #20](https://github.com/cat2151/github-actions/issues/20)

# 何が困るの？
- Geminiに次の一手を生成させるとき、cjsの内容も添付したほうが、生成品質が改善できる可能性がある。

# 案
## outputのimage
- promptが言及するfilename、について、そのfileの内容もすべてpromptに含める。
    - 軸は、projectのfilename一覧である。
        - 一覧それぞれのfilenameについて、promptで言及されているものをfile内容埋め込み、とする。
- 方向性
    - シンプルで明確なルール、曖昧さのないルールで、メンテを楽にすることを優先する
    - 余分なファイルが出てしまうが割り切ってOKとし、欠落リスクを減らせることを優先する
- 備考
    - 曖昧でメンテが必要な「documentからのfilename抽出」をやめ、
        - かわりに、逆に、「今のprojectにあるfileすべてのうち、promptで言及されているもの」を軸とする
## 実現方法の案
- project全体について、filenameと、filepath配列（複数ありうる）、のmapを取得する。そういう関数Aをまず実装する。
    - filepathは、agentが扱えるよう、github上のworkの絶対pathではなく、projectRootからの相対パス表記とする。
- そして、そのfilenameにmatchするfilepath配列について、filepathとファイル内容を記したmarkdown文字列を返却、という関数Bを実装する。
- さらに、Geminiにわたすpromptについて、前述の関数Aのfilenameそれぞれについて、prompt内を検索し、filenameが存在する場合は、そのfilenameについて、関数Bを用いてmarkdown文字列を取得する。そうして得られたmarkdown文字列群を返却する、という関数Cを実装する。
- さらに、promptの末尾に書いてあるプレースホルダー「`${file_contents}`」を、関数Cの結果で置き換える、という関数Dを実装する。
- 実際には、Geminiにわたすpromptのプレースホルダー展開は、2回にわたる必要がある。1回目でissues-note内容をpromptに埋め込む。2回目でそのpromptに対して関数Dを適用する。
## 備忘
- 上記は、agentにplanさせてレビューし、context不足と感じたら上記をメンテ、というサイクルで書いた。

# どうする？
- 上記をagentに投げる。documentやtestについてのplanもしてくるかもしれないがそこは時間の都合で省略して実施させるつもり。
- 投げた、実装させた、レビューして人力リファクタリングした
- testする

# 結果
- バグ
    - この20.mdにあるプレースホルダーが置換されてしまっている
    - issue-notesで言及されていないfileまで添付されてしまっている
- 分析
    - この20.mdにあるプレースホルダーが置換されてしまっている
        - 原因
            - 20.mdにあるプレースホルダーまで置換対象としてしまっていたため。
            - prompt全体のプレースホルダーを置換対象としてしまっていたため。
            - issue-notesを埋め込んだあとでの、プレースホルダー処理だったので、
                - 20.md が置換対象となってしまったため。
        - 対策案
            - プレースホルダーはすべて、「行頭と行末で囲まれている」ときだけ置換対象とする。
                - つまり文中やcode中のプレースホルダーは置換対象外とする。
            - さらに、2つ以上プレースホルダーが出たら想定外なので早期エラー終了させ、検知させる。
    - issue-notesで言及されていないfileまで添付されてしまっている
        - 原因
            - promptに、既にprojectの全file listが書き込まれたあとなので、
                - issue-noteで言及されていなくても、
                - promptの全file listを対象に検索してしまっている
        - 対策案の候補
            - プレースホルダー置換の順番を変更し、全file listは最後に置換する
            - file添付の対象を変更し、promptでなく、issue-notesとする
                - これが範囲が絞られているので安全である、と考える
        - 備忘
            - 全fileの対象は、リモートリポジトリ側のfileなので、secretsの心配はないし、実際に検索して確認済み

# どうする？
- agent半分、人力が半分（agentがハルシネーションでソース破壊したので、関数切り分けしたり、リファクタリングしたり）。
- で実装した。
- testする

# 結果
- test green

# closeとする

{% endraw %}
```

### issue-notes/20.md
```md
{% raw %}
# issue preview演奏機能は、カーソル位置までの演奏ではなく、JSON全体を演奏とする #20
[issues #20](https://github.com/cat2151/ym2151-log-editor/issues/20)



{% endraw %}
```

### .github/actions-tmp/issue-notes/21.md
```md
{% raw %}
# issue project-summary の development-status 生成時、project-overviewが生成済みのproject-overview.mdもpromptに添付、を試す #21
[issues #21](https://github.com/cat2151/github-actions/issues/21)

# 何が困るの？
- project-overview.mdがpromptに添付されていたほうが、Geminiの生成品質が改善できる可能性がある。
    - メリットは、ファイル一覧、関数一覧、をGeminiにわたせること

# 検討事項
- 課題、その一覧に付記されている「ファイルや関数の要約」は、Geminiが「ファイル名や関数名を元に生成しただけ」で、「ファイル内容や関数内容を参照せずに生成した」可能性が高い
    - 対策、project-overview.mdに依存しない。
        - 方法、新規関数をagentに実装させる
            - 新規関数で、ファイル一覧と関数一覧を生成する
        - 根拠、そのほうが、シンプルに目的を達成できる可能性が高そう。
        - 根拠、project-overview.mdだと、不具合として.github 配下のymlがlistに含まれておらず、ymlに関するissue、に関する生成、をするとき不具合の可能性がありそう。そういった、別機能の不具合に影響されがち。
- 課題、早期に実施したほうが毎日好影響が出る可能性がある
    - 対策、上記検討事項の対処は後回しにして、先に実装してみる
    - agentに投げる
- 課題、ProjectSummaryCoordinator をみたところ、並列処理されている
    - なので、project-overview.mdを参照したいときに、まだ生成されていない、という可能性が高い
    - 対策、前述の、新規関数で、ファイル一覧と関数一覧を生成させる

# agentに投げるための整理
- 編集対象ファイル
    - prompt
        - .github_automation/project_summary/prompts/development-status-prompt.md
        - 編集内容
            - projectのファイル一覧を埋め込む用の、プレースホルダーを追加する
    - source
        - .github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
        - 編集内容
            - projectのファイル一覧を生成する関数、を実装し、
            - それを前述のプレースホルダーに埋め込む

# agentに投げて実装させた

# test結果
- 以下が不要
    - .git/
    - node_modules/

# どうする？
- agentに上記を変更させた
- testする

# 結果
- test greenとなった

# まとめ
- issueのtitleからは仕様変更した。
    - projectのfile一覧をpromptに含める、とした。
    - そのほうがpromptとして、よい生成結果が期待できる、と判断した。
- test greenとなった

# closeとする

{% endraw %}
```

### issue-notes/21.md
```md
{% raw %}
# issue リファクタリング。app.rs の test は src/tests/ に切り出し、app.rs をコンパクトにして、今後の機能追加時のハルシネーションのリスクを下げる #21
[issues #21](https://github.com/cat2151/ym2151-log-editor/issues/21)



{% endraw %}
```

### .github/actions-tmp/issue-notes/9.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズが0件なので、原因を可視化する #9
[issues #9](https://github.com/cat2151/github-actions/issues/9)

# agentに修正させたり、人力で修正したりした
- agentがハルシネーションし、いろいろ根の深いバグにつながる、エラー隠蔽などを仕込んでいたため、検知が遅れた
- 詳しくはcommit logを参照のこと
- WSL + actの環境を少し変更、act起動時のコマンドライン引数を変更し、generated-docsをmountする（ほかはデフォルト挙動であるcpだけにする）ことで、デバッグ情報をコンテナ外に出力できるようにし、デバッグを効率化した

# test green

# closeとする

{% endraw %}
```

### src/app.rs
```rs
{% raw %}
use crate::models::Ym2151Log;
use crate::navigation::NavigationState;
use crate::time_display::TimeDisplayMode;

/// Application state
pub struct App {
    /// The loaded YM2151 log data
    pub log: Ym2151Log,
    /// Current file path (if any)
    pub file_path: Option<String>,
    /// Navigation state (scroll and selection)
    pub navigation: NavigationState,
    /// Time display mode
    pub time_mode: TimeDisplayMode,
    /// Whether the app should quit
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            log: Ym2151Log { events: vec![] },
            file_path: None,
            navigation: NavigationState::new(),
            time_mode: TimeDisplayMode::Cumulative,
            should_quit: false,
        }
    }

    /// Load a JSON file
    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.log = crate::file_io::load_file(path)?;
        self.file_path = Some(path.to_string());
        self.navigation.reset();
        Ok(())
    }

    /// Save the current log to file
    pub fn save_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.file_path {
            crate::file_io::save_file(path, &self.log)?;
            Ok(())
        } else {
            Err("No file path set".into())
        }
    }

    /// Toggle time display mode
    pub fn toggle_time_mode(&mut self) {
        self.time_mode.toggle();
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        self.navigation.move_up();
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        self.navigation.move_down(self.log.events.len());
    }

    /// Update scroll offset to keep selected item visible
    pub fn update_scroll(&mut self, visible_height: usize) {
        self.navigation.update_scroll(visible_height);
    }

    /// Format event for display
    pub fn format_event(&self, index: usize) -> String {
        crate::time_display::format_event(&self.log, index, self.time_mode)
    }

    /// Preview current event by playing events from start to selected position
    pub fn preview_current_event(&self) {
        crate::preview::preview_current_event(&self.log, self.navigation.selected_index);
    }

    /// Set wait time (cumulative time) for the selected event in milliseconds
    /// Only works in Cumulative display mode
    ///
    /// # Arguments
    /// * `milliseconds` - The wait time in milliseconds (typically 0-9).
    ///   Values are used as-is without validation. Common usage:
    ///   0-9ms (mapped from keys 0-9).
    pub fn set_wait_time_ms(&mut self, milliseconds: u32) {
        crate::event_editor::set_wait_time_ms(
            &mut self.log,
            self.navigation.selected_index,
            milliseconds,
            self.time_mode,
        );
    }

    /// Delete the currently selected event
    pub fn delete_selected_event(&mut self) {
        crate::event_editor::delete_event(&mut self.log, self.navigation.selected_index);
        self.navigation.adjust_after_delete(self.log.events.len());
    }

    /// Insert a new event before the currently selected position
    pub fn insert_event_before_selected(&mut self) {
        crate::event_editor::insert_event_before(&mut self.log, self.navigation.selected_index);
        self.navigation.adjust_after_insert();
    }

    // Accessor methods for backward compatibility with UI code
    pub fn selected_index(&self) -> usize {
        self.navigation.selected_index
    }

    pub fn scroll_offset(&self) -> usize {
        self.navigation.scroll_offset
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
        app.navigation.selected_index = 1;
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

        app.navigation.selected_index = 1;
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
        app.navigation.selected_index = 0;
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
        app.navigation.selected_index = 1;
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
        app.navigation.selected_index = 1;
        app.delete_selected_event();

        // Verify event count decreased
        assert_eq!(app.log.events.len(), 2);

        // Verify the correct event was deleted (remaining events should be index 0 and 2)
        assert_eq!(app.log.events[0].addr, "20");
        assert_eq!(app.log.events[1].addr, "60");

        // Verify selected_index is still valid
        assert_eq!(app.navigation.selected_index, 1);
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
        app.navigation.selected_index = 1;
        app.delete_selected_event();

        // Verify event count decreased
        assert_eq!(app.log.events.len(), 1);

        // Verify selected_index was adjusted to last valid index
        assert_eq!(app.navigation.selected_index, 0);
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
        app.navigation.selected_index = 0;
        app.delete_selected_event();

        // Verify all events are deleted
        assert_eq!(app.log.events.len(), 0);

        // selected_index should remain 0 (though there are no events)
        assert_eq!(app.navigation.selected_index, 0);
    }

    #[test]
    fn test_delete_empty_list() {
        let mut app = App::new();
        app.log.events = vec![];

        // Try to delete from empty list (should not panic)
        app.navigation.selected_index = 0;
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
        app.navigation.selected_index = 0;

        // Move down to second event
        app.move_down();
        assert_eq!(app.navigation.selected_index, 1);

        // Move down to empty line (one beyond last event)
        app.move_down();
        assert_eq!(app.navigation.selected_index, 2);
        assert_eq!(app.navigation.selected_index, app.log.events.len());

        // Try to move down again (should stay at empty line)
        app.move_down();
        assert_eq!(app.navigation.selected_index, 2);
    }

    #[test]
    fn test_move_down_empty_log() {
        let mut app = App::new();
        app.log.events = vec![];

        // Start at index 0 (empty)
        app.navigation.selected_index = 0;

        // Try to move down (should stay at 0)
        app.move_down();
        assert_eq!(app.navigation.selected_index, 0);
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
        app.navigation.selected_index = 0;
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
        assert_eq!(app.navigation.selected_index, 0);
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
        app.navigation.selected_index = 1;
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
        assert_eq!(app.navigation.selected_index, 1);
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
        app.navigation.selected_index = 2;
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
        assert_eq!(app.navigation.selected_index, 2);
    }

    #[test]
    fn test_insert_event_before_selected_empty_list() {
        let mut app = App::new();
        app.log.events = vec![];

        // Insert into empty list
        app.navigation.selected_index = 0;
        app.insert_event_before_selected();

        // Verify event count increased
        assert_eq!(app.log.events.len(), 1);

        // Verify new event created with time 0.0
        assert_eq!(app.log.events[0].addr, "00");
        assert_eq!(app.log.events[0].data, "00");
        assert!((app.log.events[0].time - 0.0).abs() < 0.0001);

        // Verify selected_index is still 0
        assert_eq!(app.navigation.selected_index, 0);
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
        app.navigation.selected_index = 0;
        app.navigation.scroll_offset = 1;

        app.insert_event_before_selected();

        // Verify scroll_offset was adjusted to keep new event visible
        assert_eq!(app.navigation.scroll_offset, 0);
    }
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
20c60d2 Auto-translate README.ja.md to README.md [auto]
f068d1a Clarify project scope and non-goals in README
f9ecb83 Add issue note for #21 [auto]
f6b9943 Add issue note for #20 [auto]
260ea45 Add issue note for #19 [auto]
160460f Update business logic implementation instructions
8c0d888 Update linting instructions and checklist
9c66816 Update Windows compatibility check instructions
3000225 Merge pull request #18 from cat2151/copilot/fix-windows-gnu-cross-compilation
12aa0b5 Remove unused get_time_string method to fix Windows GNU cross-compilation

### 変更されたファイル:
.github/copilot-instructions.md
README.ja.md
README.md
issue-notes/19.md
issue-notes/20.md
issue-notes/21.md


---
Generated at: 2025-12-17 07:04:30 JST

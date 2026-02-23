Last updated: 2026-02-24

# Development Status

## 現在のIssues
- [Issue #28](../issue-notes/28.md)では、GitHub Actionsのユーザーコミットチェック共通ワークフロー化が完了しましたが、Agentの生成コード品質に課題が指摘されました。
- [Issue #2](../issue-notes/2.md)の関数コールグラフHTMLビジュアライズ生成の共通ワークフロー化は完了済みですが、トップレベルのIssueノートは空で、今後のドッグフーディングが期待されます。
- 直近のコミットではJekyll設定ファイルが追加され、自動生成ドキュメントの公開基盤が整備されつつあります。

## 次の一手候補
1.  [Issue #30](../issue-notes/30.md): Agentによるコード生成時のハルシネーション対策と品質向上ガイドラインの作成
    -   最初の小さな一歩: 既存の`.github/actions-tmp/issue-notes/8.md`と`.github/actions-tmp/issue-notes/28.md`を再分析し、Agentのハルシネーションが発生した具体的な箇所や原因、およびそれに対する手動での修正内容を特定する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: .github/actions-tmp/issue-notes/8.md, .github/actions-tmp/issue-notes/28.md

        実行内容:
        対象ファイルの内容を詳細に分析し、以下の観点からAgentのハルシネーションや低品質なコード生成が指摘された箇所を特定してください。
        1. どのタスク（例: ymlの分割、特定のスクリプト修正）で問題が発生したか。
        2. Agentがどのような誤った提案やコードを生成したか（具体的なコードスニペットがあれば引用）。
        3. それらの問題が、どのような要因（例: プロンプトの曖昧さ、複雑な依存関係、大規模なコードベース）によって引き起こされたと推測されるか。

        確認事項:
        分析にあたり、Agentが「失敗」「ハルシネーション」「大量破壊」「根本的なバグ」といった表現を使用している箇所を特に注意してレビューしてください。

        期待する出力:
        特定された問題点、具体的なハルシネーションの例、および推測される原因をmarkdown形式でリストアップしてください。将来的なAgent利用におけるプロンプト設計やタスク分解の改善に役立つ洞察を含めてください。
        ```

2.  [Issue #40](../issue-notes/40.md): Call Graph生成ワークフローのHTML出力内容の検証と改善
    -   最初の小さな一歩: `generated-docs/callgraph.html`が存在するかを確認し、もし存在すればその内容を直接確認して、Call Graphが正しく描画されているか、またそれが開発者にとって有用な情報を提供しているかを評価する。存在しない場合は、生成プロセスに問題がないか確認する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: .github/actions-tmp/generated-docs/callgraph.html, .github/actions-tmp/.github/workflows/callgraph.yml, .github/actions-tmp/.github_automation/callgraph/scripts/generate-html-graph.cjs

        実行内容:
        1. `.github/actions-tmp/generated-docs/callgraph.html`が存在するかを確認してください。
        2. もしファイルが存在する場合、その内容を読み込み、Call Graphが実際に描画されているか、また、視覚的な崩れやエラーがないかを評価してください。
        3. Call Graph生成ワークフロー（`callgraph.yml`）およびHTML生成スクリプト（`generate-html-graph.cjs`）を分析し、HTML出力が適切でない場合に考えられる原因（例: データ不足、スクリプトエラー、依存ライブラリの問題）を特定してください。

        確認事項:
        ファイル一覧に`generated-docs/callgraph.html`と`generated-docs/callgraph.js`があるので、これらの連携も考慮に入れてください。[Issue #8](../issue-notes/8.md)で「もし生成されたhtmlがNGの場合は、別issueとするつもり」とされている点を意識してください。

        期待する出力:
        `callgraph.html`の存在確認結果と、その内容の初期評価結果（例: 「グラフは正常に表示されている」「グラフが一部崩れている」など）、およびHTML出力の改善に向けた具体的な提案をmarkdown形式で出力してください。
        ```

3.  [Issue #42](../issue-notes/42.md): 開発状況レポートの生成精度とロバストネスの向上
    -   最初の小さな一歩: 現在の生成プロンプトと入力情報（この開発状況生成プロンプト自体と「開発状況情報」セクション）を詳細に分析し、特にIssue番号の不整合（[Issue #28](../issue-notes/28.md)のケース）が発生する原因と、それを避けるためのプロンプトの改善点を特定する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: （このプロンプト自体：開発状況生成プロンプト）, issue-notes/2.md, issue-notes/28.md, .github/actions-tmp/issue-notes/2.md, .github/actions-tmp/issue-notes/28.md

        実行内容:
        1. 現在の「開発状況生成プロンプト」の指示内容をレビューし、特に「現在のIssues」の要約と「次の一手候補」の生成において、ハルシネーションや情報の不整合を招きやすい箇所がないかを特定してください。
        2. `issue-notes/28.md`と`.github/actions-tmp/issue-notes/28.md`、および`issue-notes/2.md`と`.github/actions-tmp/issue-notes/2.md`の内容の不整合が、現在の出力プロセスでどのように処理されるかを分析してください。
        3. このような不整合を将来的に自動で解決し、より正確なレポートを生成するためのプロンプト改善案（例: 優先順位付けルール、明確なソース指定）を提案してください。

        確認事項:
        「生成しないもの」のセクションに記載されたハルシネーションの回避ルールと、出力フォーマットの要件を厳守しているかを確認してください。Issue番号のリンク形式が正確であることも確認してください。

        期待する出力:
        現在のプロンプトの課題点と、それを解決するための具体的な改善案をmarkdown形式で出力してください。特に、複数のIssueノートファイルが存在する場合の曖昧さを解消するための明確な指示や、情報源の優先順位付けに関する提案を含めてください。

---
Generated at: 2026-02-24 07:17:15 JST

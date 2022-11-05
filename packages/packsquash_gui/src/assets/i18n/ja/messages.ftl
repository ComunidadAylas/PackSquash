-app-name = PackSquash
language-text-directionality = ltr
switched-to-language = 表示言語が日本語に変更されました
open-project-discord = 💬 Discordサーバーに参加する
open-project-github-repo = 💻 GitHubのリポジトリを開く
open-project-kofi = Ko-fiでスポンサーになって！ ❤️
app-logo-alt = { -app-name } logo
go-back = 戻る
home-screen-title = Home
home-screen-welcome = ようこそ！
home-screen-landing-text = 今日はどんなパックを圧縮する？
home-screen-squash-action = Start
home-screen-about-action = このソフトウェアについて...
about-screen-title = このソフトウェアについて
about-screen-app-build-date = ビルド日時:
about-screen-app-build-profile = ビルドプロファイル:
about-screen-app-build-target = ビルドターゲット:
about-screen-app-user-agent = ユーザーエージェント:
about-screen-app-license-text =
    このプログラムはフリーソフトウェアです。あなたはこれを、フリーソフトウェア財団によって発行されたGNU 一般公衆利用許諾書(バージョン3か、 それ以降のバージョンのうちどれか)が定める条件の下で再頒布または改変することができます。
    
    このプログラムは有用であることを願って頒布されますが、"全くの無保証"です。商業可能性の保証や特定目的への適合性は、言外に示されたものも含め、全く存在しません。詳しくはGNU 一般公衆利用許諾書をご覧くださ い。
about-screen-thanks = クレジット...
crash-screen-title = 未処理のエラー
crash-screen-header = 申し訳ございません！ エラーが発生しました。
crash-screen-error-details-text =
    ユーザーインターフェースに予期せぬ問題が発生しました。
    
    このようなことは起こらないはずです。問題が続くようであれば、この問題を報告し { -app-name } を再起動してください。
    
    エラーの技術的な説明は以下のとおりです。
crash-screen-continue-action = 続ける
pack-selection-screen-title = パックの選択
pack-selection-screen-caption = さあ、はじめましょう！
pack-selection-screen-caption-text =
    { -app-name } は、どのパックを最適化すればよいかを知る必要があります。
    
    パックのフォルダーをここにドラッグ＆ドロップするか、下のボタンをクリックして選択してください。
pack-selector-dialog-button = パックを選択する
pack-selection-screen-bad-drag-item-count-error = ドラッグしたアイテムが多すぎる、または少なすぎます。パックのフォルダーを1つだけドラッグしてください。
pack-selection-screen-bad-pack-directory-error = パックのフォルダーと認識できません。フォルダーであること、pack.mcmeta ファイルが含まれていることを確認してください。
configuration-screen-title = 設定
configuration-screen-caption = どのようにパックを最適化しますか?
configuration-screen-default-options-action = デフォルトの設定を使用する
configuration-screen-default-options-action-description = 特定のニーズがない場合は、バニラパックの最初の設定としてお勧めします。PackSquash はほとんどの場合、"正常な動作"をします。
configuration-screen-custom-options-action = カスタマイズした設定を使用する
configuration-screen-custom-options-action-description = PackSquashの動作方法を変更する場合や、ModのサポートやZIPの保護などの機能を使用したい場合は、これを選択してください。
configuration-screen-invalid-custom-option-error = この設定に入力された値は無効です。許容される値の詳細については、その説明を参照してください。
configuration-screen-invalid-custom-options-error = 少なくとも1つの設定が無効な値になっています。続行する前にそれらを見直してください。
configuration-screen-custom-options-file-action = 設定ファイルを使用する
configuration-screen-custom-options-file-action-description = 手動で作成した、もしくは以前に実行した設定ファイルがある場合に適しています。
configuration-screen-custom-options-file-parse-error =
    設定ファイルの読み込み中にエラーが発生しました。不備はありませんか？
    
    技術的な説明: { $errorDescription }
configuration-screen-optimize-action = 最適化する
configuration-screen-copy-options-action = 設定ファイルをコピーする
configuration-screen-options-copied = クリップボードに設定ファイルをコピーしました
configuration-screen-try-to-copy-invalid-options-error = { configuration-screen-invalid-custom-options-error }
optimization-screen-title = 最適化
optimization-screen-caption = パックの最適化中、しばらくお待ちください...
optimization-screen-pack-type-placeholder = パックの種類を判別しています...
optimization-screen-pack-type-description =
    パックの種類: Minecraft{ $gameVersionRange } { $packType ->
       *[resource_pack] リソースパック
        [data_pack] データパック
    }
optimization-screen-processed-file-count = { $processedFileCount } / { $packFileCount } のファイルを処理しました
optimization-screen-warning-count =
    { $warningCount ->
        [0] 警告なし
       *[other] { $warningCount } 個の警告
    }
optimization-screen-asset-status-message =
    { $assetPath }: { $strategyIdentifier ->
        [ValidatedAndMinified] 検証・軽量化
        [ValidatedDebloatedAndMinified] 検証・圧縮・軽量化
        [ValidatedAndPrettified] 検証・整形
        [ValidatedDebloatedAndPrettified] 検証・圧縮・整形
       *[Optimized] 最適化
    }
optimization-success-screen-title = パックが最適化されました
optimization-success-screen-done-text = 完了！
optimization-success-screen-caption =
    { $warningCount ->
        [0] パックは正常に最適化されました
        [one] パックは最適化されましたが、注意すべき問題が確認されました
       *[other] パックは最適化されましたが、注意すべき問題が確認されました
    }。どうしますか？
optimization-success-screen-copy-warnings-action = 警告文をクリップボードにコピーする
optimization-success-screen-warnings-copied = 警告文をクリップボードにコピーしました
optimization-success-screen-open-generated-zip-action = 生成されたZIPの出力先を開く
optimization-success-screen-start-over-action = 最初からやり直す
optimization-success-screen-start-over-toast-text = { -app-name } を使っていただきありがとうございます！もし気に入っていただけたなら、ぜひコミュニティへの参加やプロジェクトへのサポートをご検討ください。
optimization-failure-screen-title = 最適化のエラー
optimization-failure-screen-header = パックに何か問題があります！
optimization-failure-screen-caption = 以下は、エラーの技術的な説明です。うまくいけば、解決するのに役立ちます。
optimization-failure-screen-start-over-action = { optimization-success-screen-start-over-action }
optimization-failure-screen-try-again-action = 再試行
optimization-failure-screen-open-online-help-action = オンラインヘルプを開く
update-dialog-title = 利用可能なアップデートがあります
update-dialog-caption = 今日はラッキーな日です。新しい { -app-name } バージョンがリリースされました！今すぐアップデートしますか？
update-dialog-update-version = バージョン: { $version }
update-dialog-update-publication-date = 公開日: { DATETIME($date, dateStyle: "medium", timeStyle: "short") }
update-dialog-update-version-notes = 更新情報:
update-dialog-update-accept-action = はい
update-dialog-update-reject-action = いいえ
update-dialog-update-in-progress-notification = アップデートのダウンロードとインストール中です、しばらくお待ちください...
update-dialog-update-error = エラーが発生したためアップデートをインストールできませんでした: { $errorDescription }

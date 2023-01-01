-app-name = PackSquash
language-text-directionality = ltr
switched-to-language = 語言已切換為繁體中文
open-project-discord = 💬 加入 Discord 伺服器
open-project-github-repo = 💻 開啟 GitHub 儲存庫
open-project-kofi = 在 Ko-fi 上贊助我！❤️
app-logo-alt = { -app-name } logo
go-back = 返回
home-screen-title = 首頁
home-screen-welcome = 歡迎！
home-screen-landing-text = 今天想要壓縮些什麼？
home-screen-squash-action = 開始
home-screen-about-action = 關於…
about-screen-title = 關於
about-screen-app-build-date = 組建日期：
about-screen-app-build-profile = 組建類型：
about-screen-app-build-target = 組建目標：
about-screen-app-user-agent = 使用者代理程式：
about-screen-app-license-text =
    本程式為自由軟體，您可以遵照自由軟體基金會（Free Software Foundation）出版的公共許可證條款（GNU Affero General Public License）第三版或更新、您所選的版本來修改與重新發佈本程式。
    
    發布本程式目的是為了希望它對您有用，但沒有提供任何保證；甚至沒有適合特定目的的隱含和保證。更詳細的細節請參閱 GNU Affero 通用公共許可。
about-screen-thanks = 感謝…
crash-screen-title = 未處理的錯誤
crash-screen-header = 抱歉，但有些東西出錯了！
crash-screen-error-details-text =
    使用者介面發生了未預期的問題。
    
    這不應該發生。如果你繼續遇到此問題，請回報本問題並重新啟動 { -app-name }。
    
    錯誤的技術描述如下：
crash-screen-continue-action = 繼續
pack-selection-screen-title = 選擇資料夾
pack-selection-screen-caption = 讓我們開始！
pack-selection-screen-caption-text =
    { -app-name } 需讀取欲最佳化資料夾位置。
    
    請拖曳資料夾到這裡，或是點擊底下的按鈕。
pack-selector-dialog-button = 選擇資料夾
pack-selection-screen-bad-drag-item-count-error = 拖曳太多或太少物品。請僅拖曳一個資料夾。
pack-selection-screen-bad-pack-directory-error = 這看起來不像是包含資源包或是資料包的資料夾。請檢查該資料夾是否包含「pack.mcmeta」檔案。
configuration-screen-title = 組態設定
configuration-screen-caption = 要如何最佳化？
configuration-screen-default-options-action = 使用預設選項
configuration-screen-default-options-action-description = 如果你沒有特定需求，這個選項是推薦的原版資源包或資料包的起始點。{ -app-name } 在大多數情況下將會「正常」最佳化。
configuration-screen-custom-options-action = 定義和使用自訂選項
configuration-screen-custom-options-action-description = 當你需要改變 { -app-name } 的運作方式時選擇這個，或是在你需要使用一些功能例如模組支援或壓縮檔保護。
configuration-screen-invalid-custom-option-error = 你所輸入的值在這個選項中是無效的。對於更多資訊，請查看它的描述來了解所接受的值
configuration-screen-invalid-custom-options-error = 最少有一個選項有無效的值。請在繼續之前重新檢查
configuration-screen-custom-options-file-action = 使用選項檔
configuration-screen-custom-options-file-action-description = 如果你已經有選項檔，無論是手動寫的還是從之前運行所產生的，這個選項將適合你。
configuration-screen-custom-options-file-parse-error =
    讀取選項檔時發生錯誤。這是有效檔案？
    
    技術描述：{ $errorDescription }
configuration-screen-optimize-action = 最佳化
configuration-screen-copy-options-action = 複製選項檔案到剪貼簿
configuration-screen-options-copied = 選項檔案已複製至剪貼簿
configuration-screen-try-to-copy-invalid-options-error = { configuration-screen-invalid-custom-options-error }
optimization-screen-title = 最佳化
optimization-screen-caption = 請稍等，正在最佳化中……
optimization-screen-pack-type-placeholder = 正在判斷類型……
optimization-screen-pack-type-description =
    類型：Minecraft { $gameVersionRange } { $packType ->
       *[resource_pack] 資源包
        [data_pack] 資料包
    }
optimization-screen-processed-file-count = 已處理 { $processedFileCount }/{ $packFileCount } 個檔案
optimization-screen-warning-count =
    { $warningCount ->
        [0] 無警告
       *[other] { $warningCount } 個警告
    }
optimization-screen-asset-status-message =
    { $assetPath }: { $strategyIdentifier ->
        [ValidatedAndMinified] 已驗證和極簡化
        [ValidatedDebloatedAndMinified] 已驗證、去膨脹化和極簡化
        [ValidatedAndPrettified] 已驗證和美化
        [ValidatedDebloatedAndPrettified] 已驗證、去膨脹化和美化
       *[Optimized] 最佳化
    }
optimization-success-screen-title = 完成最佳化
optimization-success-screen-done-text = 完成！
optimization-success-screen-caption =
    { $warningCount ->
        [0] 已成功完成最佳化
       *[other] 已完成最佳化，但有出現幾個問題需要你的注意。
    } 現在你想做什麼？
optimization-success-screen-copy-warnings-action = 複製警告至剪貼簿
optimization-success-screen-warnings-copied = 警告已複製至剪貼簿
optimization-success-screen-open-generated-zip-action = 在檔案總管中開啟產生的壓縮檔案
optimization-success-screen-start-over-action = 重新開始
optimization-success-screen-start-over-toast-text = 感謝你使用 { -app-name }！如果你喜歡，請考慮加入社群共同討論或通過贊助來支援專案。
optimization-failure-screen-title = 最佳化錯誤
optimization-failure-screen-header = 喔不，有些東西出錯了！
optimization-failure-screen-caption = 底下是錯誤的技術描述。希望它可以對你有所幫助：
optimization-failure-screen-start-over-action = { optimization-success-screen-start-over-action }
optimization-failure-screen-try-again-action = 再試一次
optimization-failure-screen-open-online-help-action = 打開線上幫助
update-dialog-title = 有可用的更新
update-dialog-caption = 今天是你的幸運日：新的 { -app-name } 版本出爐啦！你想要現在更新嗎？
update-dialog-update-version = 版本：{ $version }
update-dialog-update-publication-date = 發布日期：{ DATETIME($date, dateStyle: "medium", timeStyle: "short") }
update-dialog-update-version-notes = 版本說明：
update-dialog-update-accept-action = 是
update-dialog-update-reject-action = 否
update-dialog-update-in-progress-notification = 下載並安裝更新中，請稍後……
update-dialog-update-error = 由於發生錯誤，因此無法安裝更新：{ $errorDescription }

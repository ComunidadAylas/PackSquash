-app-name = PackSquash

language-text-directionality = ltr
switched-to-language = Language switched to English

open-project-discord = 💬 Join the Discord server
open-project-github-repo = 💻 Open the GitHub repository
open-project-kofi = Sponsor me on Ko-fi! ❤️

app-logo-alt = { -app-name } logo

go-back = Back

home-screen-title = Home
home-screen-welcome = Welcome!
home-screen-landing-text = What pack will you squash today?
home-screen-squash-action = Start
home-screen-about-action = About...

about-screen-title = About
about-screen-app-build-date = Build date:
about-screen-app-build-profile = Build profile:
about-screen-app-build-target = Build target:
about-screen-app-user-agent = User agent:
about-screen-app-license-text = This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
about-screen-thanks = Thanks to...

crash-screen-title = Unhandled error
crash-screen-header = Sorry, something went wrong!
crash-screen-error-details-text = The user interface has encountered an unexpected problem.

    This should not happen. Please report this issue and restart { -app-name } if you continue to have trouble.

    A technical description of the error is given below:
crash-screen-continue-action = Continue

pack-selection-screen-title = Pack selection
pack-selection-screen-caption = Let's get started!
pack-selection-screen-caption-text = { -app-name } needs to know what pack to optimize.

    Please select it by dragging and dropping its directory here, or by clicking the button below.
pack-selector-dialog-button = Select pack
pack-selection-screen-bad-drag-item-count-error = Too many or too few items were dragged. Please drag only one pack directory.
pack-selection-screen-bad-pack-directory-error = That does not look like a pack directory. Please check that it is a directory and that it contains a pack.mcmeta file.

configuration-screen-title = Configuration
configuration-screen-caption = How will the pack be optimized?
configuration-screen-default-options-action = Use default options
configuration-screen-default-options-action-description = The recommended starting point for vanilla packs if you don't have specific needs. { -app-name } will "just work" in most cases.
configuration-screen-custom-options-action = Define and use customized options
configuration-screen-custom-options-action-description = Choose this if you need to change how { -app-name } operates, or if you want to use features like mod support and ZIP protection.
configuration-screen-invalid-custom-option-error = The value you have entered for this option is invalid. For more information on their acceptable values, please see its description
configuration-screen-invalid-custom-options-error = At least one option has an invalid value. Please review them before continuing
configuration-screen-custom-options-file-action = Use options file
configuration-screen-custom-options-file-action-description = The suitable choice if you already have an options file, either manually written or generated in a previous run.
configuration-screen-custom-options-file-parse-error = An error occurred while reading the options file. Is it valid?

    Technical description: { $errorDescription }
configuration-screen-optimize-action = Optimize
configuration-screen-copy-options-action = Copy options file to clipboard
configuration-screen-options-copied = Options file copied to the clipboard
configuration-screen-try-to-copy-invalid-options-error = { configuration-screen-invalid-custom-options-error }

optimization-screen-title = Optimization
optimization-screen-caption = Hold on, optimizing pack...
optimization-screen-pack-type-placeholder = Determining pack type...
optimization-screen-pack-type-description = Pack type: Minecraft { $gameVersionRange } { $packType ->
        *[resource_pack] resource pack
         [data_pack] data pack
    }
optimization-screen-processed-file-count = { $processedFileCount } of { $packFileCount } { $packFileCount ->
         [one] file
        *[other] files
    } processed
optimization-screen-warning-count = { $warningCount ->
         [0] No warnings
         [one] One warning
        *[other] { $warningCount } warnings
    }
optimization-screen-asset-status-message = { $assetPath }: { $strategyIdentifier ->
         [ValidatedAndMinified] validated and minified
         [ValidatedDebloatedAndMinified] validated, debloated and minified
         [ValidatedAndPrettified] validated and prettified
         [ValidatedDebloatedAndPrettified] validated, debloated and prettified
        *[Optimized] optimized
    }

optimization-success-screen-title = Pack optimized
optimization-success-screen-done-text = Done!
optimization-success-screen-caption = { $warningCount ->
         [0] The pack has been successfully optimized
         [one] The pack has been optimized, but an issue that warrants your attention was identified
        *[other] The pack has been optimized, but issues that warrant your attention were identified
    }. What do you want to do now?
optimization-success-screen-copy-warnings-action = Copy warnings to clipboard
optimization-success-screen-warnings-copied = Warnings copied to the clipboard
optimization-success-screen-open-generated-zip-action = Open generated ZIP in file browser
optimization-success-screen-start-over-action = Start over
optimization-success-screen-start-over-toast-text = Thank you for using { -app-name }! If you liked it, please consider engaging with the community or supporting the project.

optimization-failure-screen-title = Optimization error
optimization-failure-screen-header = Oops, something is wrong with the pack!
optimization-failure-screen-caption = Below is a technical description of the error. Hopefully, it will help you to solve it:
optimization-failure-screen-start-over-action = { optimization-success-screen-start-over-action }
optimization-failure-screen-try-again-action = Try again
optimization-failure-screen-open-online-help-action = Open online help

update-dialog-title = Update available
update-dialog-caption = It's your lucky day: a new { -app-name } version is out! Do you want to update now?
update-dialog-update-version = Version: { $version }
update-dialog-update-publication-date = Publication date: { DATETIME($date, dateStyle: "medium", timeStyle: "short") }
update-dialog-update-version-notes = Version notes:
update-dialog-update-accept-action = Yes
update-dialog-update-reject-action = No
update-dialog-update-in-progress-notification = Downloading and installing update, please wait...
update-dialog-update-error = The update could not be installed due to an error: { $errorDescription }

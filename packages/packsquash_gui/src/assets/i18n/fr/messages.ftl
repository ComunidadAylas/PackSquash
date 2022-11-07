-app-name = PackSquash
language-text-directionality = ltr
switched-to-language = La langue a été mise en français
open-project-discord = 💬 Rejoindre le serveur Discord
open-project-github-repo = 💻 Ouvrir le dépôt GitHub
open-project-kofi = Sponsorisez-moi sur Ko-fi ! ❤️
app-logo-alt = Logo de { -app-name }
go-back = Précédent
home-screen-title = Accueil
home-screen-welcome = Bienvenue !
home-screen-landing-text = Quel paquet allez-vous optimiser aujourd'hui ?
home-screen-squash-action = Commencer
home-screen-about-action = À propos de...
about-screen-title = À propos de
about-screen-app-build-date = Date de compilation:
about-screen-app-build-profile = Profil de compilation :
about-screen-app-build-target = Cible de compilation :
about-screen-app-user-agent = Agent utilisateur :
about-screen-app-license-text =
    Ce programme est un logiciel libre : vous pouvez le redistribuer et/ou le modifier tout en respectant les termes de la "GNU General Public License" publiée par la Free Software Foundation; soit la version 3 de Licence,  soit (selon votre préférence) toute version ultérieure.
    
    Ce programme est distribué dans l'espoir qu'il sera utile, cependant SANS AUCUNE GARANTIE; sans même une garantie implicite de QUALITÉ MARCHANDE ou D’ADAPTATION À UN USAGE PARTICULIER. Reportez-vous à la Licence Publique Générale GNU Affero pour plus de détails.
about-screen-thanks = Merci à...
crash-screen-title = Erreur inattendue
crash-screen-header = Désolé, quelque chose s'est mal passé !
crash-screen-error-details-text =
    L'interface utilisateur a rencontré un problème inattendu.
    
    Cela ne devrait pas se produire. Veuillez signaler ce problème et redémarrer { -app-name } si vous continuez à avoir des problèmes.
    
    Une description technique de l'erreur est donnée ci-dessous :
crash-screen-continue-action = Continuer
pack-selection-screen-title = Sélection du pack
pack-selection-screen-caption = Commençons !
pack-selection-screen-caption-text =
    { -app-name } a besoin de savoir quel pack à optimiser.
    
    Choisissez-le en faisant glisser son répertoire ici, ou en cliquant sur le bouton ci-dessous.
pack-selector-dialog-button = Sélectionnez le pack
pack-selection-screen-bad-drag-item-count-error = Vous avez déplacé trop ou trop peu d'éléments. Faites glisser qu'un seul répertoire de pack.
pack-selection-screen-bad-pack-directory-error = Cela ne ressemble pas à un répertoire de pack. Assurez-vous qu'il s'agit d'un répertoire et qu'il contient un fichier pack.mcmeta.
configuration-screen-title = Configuration
configuration-screen-caption = Comment le pack sera-t-il optimisé ?
configuration-screen-default-options-action = Utiliser les options par défaut
configuration-screen-default-options-action-description = Le point de départ recommandé pour les packs "vanilla" si vous n'avez pas de besoins spécifiques. PackSquash fonctionnera "correctement" dans la plupart des cas.
configuration-screen-custom-options-action = Définir et utiliser les options personnalisées
configuration-screen-custom-options-action-description = Choisissez cette option si vous avez besoin de modifier le mode de fonctionnement de PackSquash, ou si vous souhaitez utiliser des fonctionnalités telles que la prise en charge des mods et la protection ZIP.
configuration-screen-invalid-custom-option-error = La valeur que vous avez saisie pour cette option n'est pas valide. Pour plus d'informations sur leurs valeurs acceptables, veuillez consulter sa description
configuration-screen-invalid-custom-options-error = Au moins une option a une valeur invalide. Veuillez la vérifier avant de continuer
configuration-screen-custom-options-file-action = Utiliser le fichier d'options
configuration-screen-custom-options-file-action-description = Le choix approprié si vous avez déjà un fichier d'options, écrit manuellement ou généré lors d'une exécution précédente.
configuration-screen-custom-options-file-parse-error =
    Une erreur s'est produite lors de la lecture du fichier d'options. Est-il valide ?
    
    Description technique : { $errorDescription }
configuration-screen-optimize-action = Optimiser
configuration-screen-copy-options-action = Copier le fichier d'options dans le presse-papier
configuration-screen-options-copied = Fichier d'options copié dans le presse-papier
configuration-screen-try-to-copy-invalid-options-error = { configuration-screen-invalid-custom-options-error }
optimization-screen-title = Optimisation
optimization-screen-caption = Optimisation du pack... Veuillez patienter
optimization-screen-pack-type-placeholder = Déterminer le type de pack...
optimization-screen-pack-type-description =
    Type de pack : Minecraft { $gameVersionRange } { $packType ->
       *[resource_pack] pack de ressources
        [data_pack] pack de données
    }
optimization-screen-processed-file-count =
    { $processedFileCount } de { $packFileCount } { $packFileCount ->
        [one] fichier
       *[other] fichiers
    } traités
optimization-screen-warning-count =
    { $warningCount ->
        [0] Aucun avertissement
        [one] Un avertissement
       *[other] { $warningCount } avertissements
    }
optimization-screen-asset-status-message =
    { $assetPath }: { $strategyIdentifier ->
        [ValidatedAndMinified] validé et minimisé
        [ValidatedDebloatedAndMinified] validé, purgé et minimisé
        [ValidatedAndPrettified] validé et embelli
        [ValidatedDebloatedAndPrettified] validé, purgé et embelli
       *[Optimized] optimisé
    }
optimization-success-screen-title = Pack optimisé
optimization-success-screen-done-text = C'est fait !
optimization-success-screen-caption =
    { $warningCount ->
        [0] Le pack a été optimisé avec succès
        [one] Le pack a été optimisé, mais un problème a été identifié et il est conseillé de le résoudre
       *[other] Le pack a été optimisé, mais des problèmes ont été identifiés et il est conseillé de les résoudre
    }. Que voulez-vous faire maintenant ?
optimization-success-screen-copy-warnings-action = Copier les avertissements dans le presse-papiers
optimization-success-screen-warnings-copied = Avertissements copiés dans le presse-papiers
optimization-success-screen-open-generated-zip-action = Ouvrir le ZIP généré dans le navigateur de fichiers
optimization-success-screen-start-over-action = Recommencer
optimization-success-screen-start-over-toast-text = Merci d'avoir utilisé { -app-name } ! Si vous l'avez aimé, vous pouvez envisager de participer à sa communauté ou de soutenir le projet.
optimization-failure-screen-title = Erreur d'optimisation
optimization-failure-screen-header = Oups, quelque chose ne va pas dans le pack !
optimization-failure-screen-caption = Ci-dessous une description technique de l'erreur s'affiche. J'espère que cela vous aidera à la résoudre :
optimization-failure-screen-start-over-action = { optimization-success-screen-start-over-action }
optimization-failure-screen-try-again-action = Essayez à nouveau
optimization-failure-screen-open-online-help-action = Consulter l'aide en ligne
update-dialog-title = Mise à jour disponible
update-dialog-caption = C'est votre jour de chance, une nouvelle version de { -app-name } est sortie ! Voulez-vous mettre à jour maintenant ?
update-dialog-update-version = Version : { $version }
update-dialog-update-publication-date = Date de publication : { DATETIME($date, dateStyle : "medium", timeStyle : "short") }
update-dialog-update-version-notes = Notes de version :
update-dialog-update-accept-action = Oui
update-dialog-update-reject-action = Non
update-dialog-update-in-progress-notification = Téléchargement et installation de la mise à jour, merci de patienter...
update-dialog-update-error = La mise à jour n'a pas pu être installée en raison d'une erreur: { $errorDescription }

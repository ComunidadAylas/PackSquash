-app-name = PackSquash

language-text-directionality = ltr
switched-to-language = Idioma cambiado a galego

open-project-discord = 💬 Unirse ao servidor de Discord
open-project-github-repo = 💻 Ir ao repositorio de GitHub
open-project-kofi = Apóiame en Ko-fi! ❤️

app-logo-alt = Logotipo de { -app-name }

go-back = Volver

home-screen-title = Inicio
home-screen-welcome = Benvido!
home-screen-landing-text = Que paquete optimizarás hoxe?
home-screen-squash-action = Comezar
home-screen-about-action = Acerca de...

about-screen-title = Acerca de
about-screen-app-build-date = Data de compilación:
about-screen-app-build-profile = Perfil de compilación:
about-screen-app-build-target = Plataforma de compilación:
about-screen-app-user-agent = Axente de usuario:
about-screen-app-license-text = Este programa é software libre: podes redistribuilo e/ou modificalo baixo os termos da GNU Affero General Public License publicada pola Free Software Foundation, xa sexa a versión 3 da Licencia ou, a túa escolla, calquera versión posterior.

    Este programa distribúese coa esperanza de que sexa útil, mais SEN NINGUNHA GARANTÍA; nin siquera a garantía implícita de COMERCIALIZACIÓN ou ADECUACIÓN A UN PROPÓSITO PARTICULAR. Consulta a GNU Affero General Public License para máis detalles.
about-screen-thanks = Grazas a...

crash-screen-title = Erro inesperado
crash-screen-header = Perdoa, algo foi mal!
crash-screen-error-details-text = A interface de usuario encontrou un problema inesperado.

    Esto non debería pasar. Reporta esta incidencia e reinicia { -app-name } se segues tendo problemas.

    Deseguido móstrase unha descripción técnica do erro:
crash-screen-continue-action = Continuar

pack-selection-screen-title = Escolla de paquete
pack-selection-screen-caption = ¡Comecemos!
pack-selection-screen-caption-text = { -app-name } precisa saber qué paquete optimizar.

    Escólleo arrastrando o seu directorio aquí, ou facendo clic no botón de abaixo.
pack-selector-dialog-button = Escoller paquete
pack-selection-screen-bad-drag-item-count-error = Arrastraches elementos de máis ou de menos. Arrastra un só directorio de paquete.
pack-selection-screen-bad-pack-directory-error = Iso non semella o directorio dun paquete. Comproba que é un directorio e contén un ficheiro pack.mcmeta.

configuration-screen-title = Configuración
configuration-screen-caption = Como optimizar o paquete?
configuration-screen-default-options-action = Usar opcións predeterminadas
configuration-screen-default-options-action-description = O punto de partida recomendado para paquetes vanilla se non tes necesidades específicas. { -app-name } funcionará ben na maioría de casos.
configuration-screen-custom-options-action = Definir e usar opcións personalizadas
configuration-screen-custom-options-action-description = Escolle isto se precisas cambiar como funciona { -app-name }, ou se queres usar características coma o soporte de mods e protección ZIP.
configuration-screen-invalid-custom-option-error = O valor introducido para esta opción é inválido. Para máis información acerca dos valores aceptables, revisa a súa descripción
configuration-screen-invalid-custom-options-error = Polo menos unha opción ten un valor inválido. Por favor, revísaas antes de continuar
configuration-screen-custom-options-file-action = Usar ficheiro de opcións
configuration-screen-custom-options-file-action-description = A escolla apropiada se xa tes un ficheiro de opcións, xa sexa escrito manualmente ou xerado nunha execución anterior.
configuration-screen-custom-options-file-parse-error = Ocorreu un erro ao ler o ficheiro de opcións. É válido?

    Descripción técnica: { $errorDescription }
configuration-screen-optimize-action = Optimizar
configuration-screen-copy-options-action = Copiar ficheiro de opcións no portapapeis
configuration-screen-options-copied = Ficheiro de opcións copiado no portapapeis
configuration-screen-try-to-copy-invalid-options-error = { configuration-screen-invalid-custom-options-error }

optimization-screen-title = Optimización
optimization-screen-caption = Optimizando paquete... Agarda
optimization-screen-pack-type-placeholder = Precisando tipo de paquete...
optimization-screen-pack-type-description = Tipo de paquete: { $packType ->
        *[resource_pack] paquete de recursos
         [data_pack] paquete de datos
    } para Minecraft { $gameVersionRange }
optimization-screen-processed-file-count = { $processedFileCount } de { $packFileCount } { $packFileCount ->
         [one] ficheiro procesado
        *[other] ficheiros procesados
    }
optimization-screen-warning-count = { $warningCount ->
         [0] Sen avisos
         [one] Un aviso
        *[other] { $warningCount } avisos
    }
optimization-screen-asset-status-message = { $assetPath }: { $strategyIdentifier ->
         [ValidatedAndMinified] validado e minimizado
         [ValidatedDebloatedAndMinified] validado, purgado e embelecido
         [ValidatedAndPrettified] validado e embelecido
         [ValidatedDebloatedAndPrettified] validado, purgado e embelecido
        *[Optimized] optimizado
    }

optimization-success-screen-title = Paquete optimizado
optimization-success-screen-done-text = Feito!
optimization-success-screen-caption = { $warningCount ->
         [0] O paquete foi optimizado con éxito
         [one] O paquete foi optimizado, mais identificouse un asunto que se aconsella atender
        *[other] O paquete foi optimizado, mais identificáronse asuntos que se aconsella atender
    }. Que queres facer agora?
optimization-success-screen-copy-warnings-action = Copiar avisos no portapapeis
optimization-success-screen-warnings-copied = Avisos copiados no portapapeis
optimization-success-screen-open-generated-zip-action = Abrir ZIP xerado no explorador de ficheiros
optimization-success-screen-start-over-action = Volver a comezar
optimization-success-screen-start-over-toast-text = ¡Grazas por empregar { -app-name }! Se che gustou, podes considerar participar na súa comunidade ou apoiar o proxecto.

optimization-failure-screen-title = Erro de optimización
optimization-failure-screen-header = Ups, algo está mal co paquete!
optimization-failure-screen-caption = Deseguido móstrase unha descripción técnica do erro. Con sorte, axudarache a corrixilo:
optimization-failure-screen-start-over-action = { optimization-success-screen-start-over-action }
optimization-failure-screen-try-again-action = Intentalo de novo
optimization-failure-screen-open-online-help-action = Consultar axuda en línea

update-dialog-title = Actualización dispoñible
update-dialog-caption = É o teu día de sorte: saliu unha nova versión de { -app-name }! Queres actualizar agora?
update-dialog-update-version = Versión: { $version }
update-dialog-update-publication-date = Data de publicación: { DATETIME($date, dateStyle: "medium", timeStyle: "short") }
update-dialog-update-version-notes = Notas da versión:
update-dialog-update-accept-action = Si
update-dialog-update-reject-action = Non
update-dialog-update-in-progress-notification = Descargando e instalando actualización, agarda...
update-dialog-update-error = A actualización non puido ser instalada por un erro: { $errorDescription }

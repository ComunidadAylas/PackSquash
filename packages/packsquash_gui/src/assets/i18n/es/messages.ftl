-app-name = PackSquash
language-text-directionality = ltr
switched-to-language = Idioma cambiado a español
open-project-discord = 💬 Unirse al servidor de Discord
open-project-github-repo = 💻 Ir al repositorio de GitHub
open-project-kofi = ¡Apóyame en Ko-fi! ❤️
app-logo-alt = Logotipo de { -app-name }
go-back = Volver
home-screen-title = Inicio
home-screen-welcome = ¡Bienvenido!
home-screen-landing-text = ¿Qué paquete optimizarás hoy?
home-screen-squash-action = Empezar
home-screen-about-action = Acerca de...
about-screen-title = Acerca de
about-screen-app-build-date = Fecha de compilación:
about-screen-app-build-profile = Perfil de compilación:
about-screen-app-build-target = Plataforma de compilación:
about-screen-app-user-agent = Agente de usuario:
about-screen-app-license-text =
    Este programa es software libre: puedes redistribuirlo y/o modificarlo bajo los términos de la GNU Affero General Public License publicada por la Free Software Foundation, ya sea la versión 3 de la Licencia o, a tu elección, cualquier versión posterior.
    
    Este programa se distribuye con la esperanza de que sea útil, pero SIN NINGUNA GARANTÍA; ni siquiera la garantía implícita de COMERCIALIZACIÓN o ADECUACIÓN A UN PROPÓSITO PARTICULAR. Consulta la GNU Affero General Public License para más detalles.
about-screen-thanks = Gracias a...
crash-screen-title = Error inesperado
crash-screen-header = Disculpa, ¡algo fue mal!
crash-screen-error-details-text =
    La interfaz de usuario ha encontrado un problema inesperado.
    
    Esto no debería de ocurrir. Reporta esta incidencia y reinicia { -app-name } si sigues teniendo problemas.
    
    A continuación figura una descripción técnica del error:
crash-screen-continue-action = Continuar
pack-selection-screen-title = Selección de paquete
pack-selection-screen-caption = ¡Comencemos!
pack-selection-screen-caption-text =
    { -app-name } necesita saber qué paquete optimizar.
    
    Escógelo arrastrando su directorio hasta aquí, o haciendo clic en el botón de abajo.
pack-selector-dialog-button = Seleccionar paquete
pack-selection-screen-bad-drag-item-count-error = Has arrastrado demasiados o demasiado pocos elementos. Arrastra un único directorio de paquete.
pack-selection-screen-bad-pack-directory-error = Eso no parece el directorio de un paquete. Asegúrate de que es un directorio y contiene un fichero pack.mcmeta.
configuration-screen-title = Configuración
configuration-screen-caption = ¿Cómo optimizar el paquete?
configuration-screen-default-options-action = Usar opciones predeterminadas
configuration-screen-default-options-action-description = El punto de partida recomendado para paquetes vanilla si no tienes necesidades específicas. { -app-name } funcionará bien en la mayoría de casos.
configuration-screen-custom-options-action = Definir y usar opciones personalizadas
configuration-screen-custom-options-action-description = Escoge esto si necesitas cambiar cómo funciona { -app-name }, o si quieres usar características como el soporte de mods y la protección ZIP.
configuration-screen-invalid-custom-option-error = El valor introducido para esta opción es inválido. Para más información sobre sus valores aceptables, revisa su descripción
configuration-screen-invalid-custom-options-error = Al menos una opción tiene un valor inválido. Por favor, revísalas antes de continuar
configuration-screen-custom-options-file-action = Usar fichero de opciones
configuration-screen-custom-options-file-action-description = La elección apropiada si ya tienes un fichero de opciones, ya sea escrito manualmente o generado en una ejecución anterior.
configuration-screen-custom-options-file-parse-error =
    Ha ocurrido un error al leer el fichero de opciones. ¿Es válido?
    
    Descripción técnica: { $errorDescription }
configuration-screen-optimize-action = Optimizar
configuration-screen-copy-options-action = Copiar fichero de opciones al portapapeles
configuration-screen-options-copied = Fichero de opciones copiado al portapapeles
configuration-screen-try-to-copy-invalid-options-error = { configuration-screen-invalid-custom-options-error }
optimization-screen-title = Optimización
optimization-screen-caption = Optimizando paquete... Espera
optimization-screen-pack-type-placeholder = Determinando tipo de paquete...
optimization-screen-pack-type-description =
    Tipo de paquete: { $packType ->
       *[resource_pack] paquete de recursos
        [data_pack] paquete de datos
    } para Minecraft { $gameVersionRange }
optimization-screen-processed-file-count =
    { $processedFileCount } de { $packFileCount } { $packFileCount ->
        [one] fichero procesado
       *[other] ficheros procesados
    }
optimization-screen-warning-count =
    { $warningCount ->
        [0] Sin advertencias
        [one] Una advertencia
       *[other] { $warningCount } advertencias
    }
optimization-screen-asset-status-message =
    { $assetPath }: { $strategyIdentifier ->
        [ValidatedAndMinified] validado y minimizado
        [ValidatedDebloatedAndMinified] validado, purgado y minimizado
        [ValidatedAndPrettified] validado y embellecido
        [ValidatedDebloatedAndPrettified] validado, purgado y embellecido
       *[Optimized] optimizado
    }
optimization-success-screen-title = Paquete optimizado
optimization-success-screen-done-text = ¡Hecho!
optimization-success-screen-caption =
    { $warningCount ->
        [0] El paquete ha sido optimizado con éxito
        [one] El paquete ha sido optimizado, pero se ha identificado un asunto que se aconseja atender
       *[other] El paquete ha sido optimizado, pero se han identificado asuntos que se aconseja atender
    }. ¿Qué quieres hacer ahora?
optimization-success-screen-copy-warnings-action = Copiar advertencias al portapapeles
optimization-success-screen-warnings-copied = Advertencias copiadas al portapapeles
optimization-success-screen-open-generated-zip-action = Abrir ZIP generado en el explorador de ficheros
optimization-success-screen-start-over-action = Volver a empezar
optimization-success-screen-start-over-toast-text = ¡Gracias por usar { -app-name }! Si te ha gustado, puedes considerar participar en su comunidad o apoyar el proyecto.
optimization-failure-screen-title = Error de optimización
optimization-failure-screen-header = Ups, ¡algo está mal con el paquete!
optimization-failure-screen-caption = A continuación se muestra una descripción técnica del error. Con suerte, te ayudará a solucionarlo:
optimization-failure-screen-start-over-action = { optimization-success-screen-start-over-action }
optimization-failure-screen-try-again-action = Intentar de nuevo
optimization-failure-screen-open-online-help-action = Consultar ayuda en línea
update-dialog-title = Actualización disponible
update-dialog-caption = Es tu día de suerte: ¡ha salido una nueva versión de { -app-name }! ¿Quieres actualizar ahora?
update-dialog-update-version = Versión: { $version }
update-dialog-update-publication-date = Fecha de publicación: { DATETIME($date, dateStyle: "medium", timeStyle: "short") }
update-dialog-update-version-notes = Notas de la versión:
update-dialog-update-accept-action = Sí
update-dialog-update-reject-action = No
update-dialog-update-in-progress-notification = Descargando e instalando actualización, espera...
update-dialog-update-error = La actualización no pudo ser instalada por un error: { $errorDescription }

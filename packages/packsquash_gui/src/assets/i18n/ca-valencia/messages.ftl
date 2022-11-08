-app-name = PackSquash
language-text-directionality = ltr
switched-to-language = Llenguatge canviat a valencià
open-project-discord = Unir-se al servidor de Discord
open-project-github-repo = Anar al repositori de GitHub
open-project-kofi = Dona'm suport en Ko-fi! ❤️
app-logo-alt = Logotip de { -app-name }
go-back = Tornar
home-screen-title = Inici
home-screen-welcome = Benvingut!
home-screen-landing-text = Quin paquet optimitzaràs hui?
home-screen-squash-action = Començar
home-screen-about-action = Sobre...
about-screen-title = Sobre
about-screen-app-build-date = Data de compilació:
about-screen-app-build-profile = Perfil de compilació:
about-screen-app-build-target = Plataforma de compilació:
about-screen-app-user-agent = Agent d'usuari:
about-screen-app-license-text =
    Aquest programa és software lliure: pots redistribuir-ho i/o modificar-ho sota els termes de la GNU Affero General Public License publicada per la Free Software Foundation, ja siga la versió 3 de la Llicència o, a la teua elecció, qualsevol versió posterior.
    
    Aquest programa es distribueix amb l'esperança que siga útil, però SENSE CAP GARANTIA; ni tan sols la garantia implícita de COMERCIALITZACIÓ o ADEQUACIÓ A UN PROPÒSIT PARTICULAR. Consulta la GNU Affero General Public License per a més detalls.
about-screen-thanks = Gràcies a...
crash-screen-title = Error inesperat
crash-screen-header = Disculpa, alguna cosa va eixir malament!
crash-screen-error-details-text =
    La interfície d'usuari ha trobat un problema inesperat.
    
    Això no hauria d'ocórrer. Reporta aquesta incidència i reinicia { -app-name } si continues tenint problemes.
    
    A continuació figura una descripció tècnica de l'error:
crash-screen-continue-action = Continuar
pack-selection-screen-title = Selecció de paquet
pack-selection-screen-caption = Comencem!
pack-selection-screen-caption-text =
    { -app-name } necessita saber quin paquet optimitzar.
    
    Tria-ho arrossegant el seu directori fins ací, o fent clic en el botó de baix.
pack-selector-dialog-button = Seleccionar paquet
pack-selection-screen-bad-drag-item-count-error = Has arrossegat massa o massa pocs elements. Arrossega un únic directori de paquet.
pack-selection-screen-bad-pack-directory-error = Això no sembla el directori d'un paquet. Assegura't que és un directori i conté un fitxer pack.mcmeta.
configuration-screen-title = Configuració
configuration-screen-caption = Com vols optimitzar el paquet?
configuration-screen-default-options-action = Usar opcions predeterminades
configuration-screen-default-options-action-description = El punt de partida recomanat per a paquets vanilla si no tens necessitats específiques. { -app-name } funcionarà bé en la majoria de casos.
configuration-screen-custom-options-action = Definir i usar opcions personalitzades
configuration-screen-custom-options-action-description = Tria això si necessites canviar com funciona { -app-name }, o si vols usar característiques com el suport de mods i la protecció ZIP.
configuration-screen-invalid-custom-option-error = El valor introduït per a aquesta opció és invàlid. Per a més informació sobre els seus valors acceptables, revisa la seua descripció
configuration-screen-invalid-custom-options-error = Almenys una opció té un valor invàlid. Per favor, revisa-les abans de continuar
configuration-screen-custom-options-file-action = Usar fitxer d'opcions
configuration-screen-custom-options-file-action-description = L'elecció apropiada si ja tens un fitxer d'opcions, ja siga escrit manualment o generat en una execució anterior.
configuration-screen-custom-options-file-parse-error =
    Ha ocorregut un error en llegir el fitxer d'opcions. És vàlid?
    
    Descripció tècnica: { $errorDescription }
configuration-screen-optimize-action = Optimitzar
configuration-screen-copy-options-action = Copiar fitxer d'opcions al portapapers
configuration-screen-options-copied = Fitxer d'opcions copiat al portapapers
configuration-screen-try-to-copy-invalid-options-error = { configuration-screen-invalid-custom-options-error }
optimization-screen-title = Optimització
optimization-screen-caption = Optimitzant paquet... Espera
optimization-screen-pack-type-placeholder = Determinant tipus de paquet...
optimization-screen-pack-type-description =
    Tipus de paquet: { $packType ->
       *[resource_pack] paquet de recursos
         [data_pack] paquet de dades
    } per a Minecraft { $gameVersionRange }
optimization-screen-processed-file-count =
    { $processedFileCount } de { $packFileCount } { $packFileCount ->
        [one] fitxer processat
       *[other] fitxers processats
    }
optimization-screen-warning-count =
    { $warningCount ->
        [0] Sense advertiments
        [one] Un advertiment
       *[other] { $warningCount } advertiments
    }
optimization-screen-asset-status-message =
    { $assetPath }: { $strategyIdentifier ->
        [ValidatedAndMinified] validat i minimitzat
        [ValidatedDebloatedAndMinified] validat, purgat i minimitzat
        [ValidatedAndPrettified] validat i embellit
        [ValidatedDebloatedAndPrettified] validat, purgat i embellit
       *[Optimized] optimitzat
    }
optimization-success-screen-title = Paquet optimitzat
optimization-success-screen-done-text = Fet!
optimization-success-screen-caption =
    { $warningCount ->
        [0] El paquet ha sigut optimitzat amb èxit
        [one] El paquet ha sigut optimitzat, però s'ha identificat un assumpte que s'aconsella atendre
       *[other] El paquet ha sigut optimitzat, però s'han identificat assumptes que s'aconsella atendre
    }. Què vols fer ara?
optimization-success-screen-copy-warnings-action = Copiar advertiments al portapapers
optimization-success-screen-warnings-copied = Advertiments copiats al portapapers
optimization-success-screen-open-generated-zip-action = Obrir ZIP generat en l'explorador de fitxers
optimization-success-screen-start-over-action = Tornar a començar
optimization-success-screen-start-over-toast-text = Gràcies per usar { -app-name }! Si t'ha agradat, pots considerar participar en la seua comunitat o donar suport al projecte.
optimization-failure-screen-title = Error d'optimització
optimization-failure-screen-header = Ups, alguna cosa està malament amb el paquet!
optimization-failure-screen-caption = A continuació es mostra una descripció tècnica de l'error. Amb sort, t'ajudarà a solucionar-ho:
optimization-failure-screen-start-over-action = { optimization-success-screen-start-over-action }
optimization-failure-screen-try-again-action = Intentar de nou
optimization-failure-screen-open-online-help-action = Consultar ajuda en línia
update-dialog-title = Actualització disponible
update-dialog-caption = És el teu dia de sort: ha eixit una nova versió de { -app-name }! Vols actualitzar ara?
update-dialog-update-version = Versió: { $version }
update-dialog-update-publication-date = Data de publicació: { DATETIME($date, dateStyle: "medium", timeStyle: "short") }
update-dialog-update-version-notes = Notes de la versió:
update-dialog-update-accept-action = Sí
update-dialog-update-reject-action = No
update-dialog-update-in-progress-notification = Descarregant e instal·lant actualització, espera...
update-dialog-update-error = L'actualització no va poder ser instal·lada per un error: { $errorDescription }

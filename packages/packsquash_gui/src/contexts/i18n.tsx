import {
  Accessor,
  createContext,
  createEffect,
  createSignal,
  Setter,
  useContext
} from "solid-js";
import { ParentProps } from "solid-js/types/render/component";
import { negotiateLanguages } from "@fluent/langneg";
import { FluentBundle, FluentResource } from "@fluent/bundle";
import { Localization } from "@fluent/dom";
import { setSelectedLocale as persistSelectedLocale } from "../util/localeSettings";

// The locales are ordered by the estimated total number of speakers of their language
// according to https://en.wikipedia.org/wiki/List_of_languages_by_total_number_of_speakers
//
// The language tags here should be valid according to BCP47 and used by user agents, to make
// automatic language negotiation possible.
//
// For example, this second requirement rules out the more precise cmn-tw tag for Mandarin
// Chinese as spoken in Taiwan, because browsers universally identify it with the older zh-tw
// tag.
//
// Useful links:
// https://r12a.github.io/app-subtags/
// https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language
// https://stackoverflow.com/questions/5580876/navigator-language-list-of-all-languages
const LOCALES = ["en", "zh-tw", "es", "fr", "ja", "ca-valencia", "gl"] as const;

export type Locale = typeof LOCALES[number];

export const AvailableLocales: Record<Locale, LocaleInfo> = {
  en: {
    friendlyName: "English"
  },
  "zh-tw": {
    friendlyName: "繁體中文",
  },
  es: {
    friendlyName: "Español"
  },
  fr: {
    friendlyName: "Français"
  },
  ja: {
    friendlyName: "日本語"
  },
  "ca-valencia": {
    friendlyName: "Valencià"
  },
  gl: {
    friendlyName: "Galego"
  }
};

export interface I18nOptions {
  selectedLocale?: Locale;
}

export interface LocaleInfo {
  friendlyName: string;
}

export type LocalizationProvider = (
  id: string,
  args?: Record<string, unknown>,
  fallbackMessage?: string
) => Accessor<string>;

type I18nContextValue = [
  LocalizationProvider,
  {
    selectedLocale: Accessor<Locale | undefined>;
    setSelectedLocale: Setter<Locale | undefined>;
  }
];

const I18nContext = createContext<I18nContextValue>();

export function I18nProvider(props: ParentProps<I18nOptions>) {
  const [selectedLocale, setSelectedLocale] = createSignal(
    props.selectedLocale
  );
  const [l10n, setL10n] = createSignal<Localization>();

  async function* generateBundles(resourceIds: string[]) {
    console.debug("Generating localization bundles");

    const preferredLocale = selectedLocale();
    const negotiatedLocales = negotiateLanguages(
      [...(preferredLocale ? [preferredLocale] : []), ...navigator.languages],
      LOCALES,
      { defaultLocale: "en" }
    );

    console.debug(`Negotiated locales: ${negotiatedLocales}`);

    let isFirstNegotiatedLocale = true;
    for (const locale of negotiatedLocales) {
      const bundle = new FluentBundle(locale);

      let resourceId;
      try {
        for (resourceId of resourceIds) {
          const resourcePath = new URL(
            `../assets/i18n/${locale}/${resourceId}`,
            import.meta.url
          );
          const resource = new FluentResource(
            await (await fetch(resourcePath)).text()
          );

          let bundleErrors;
          if ((bundleErrors = bundle.addResource(resource)).length > 0) {
            throw bundleErrors[0];
          }
        }

        // The first negotiated locale is the one we will try to display. The rest are fallbacks.
        // We always have at least one negotiated locale because we default to English
        if (isFirstNegotiatedLocale) {
          document.documentElement.lang = locale;
          isFirstNegotiatedLocale = false;
        }

        yield bundle;
      } catch (error) {
        console.error(
          `Could not load resource ${resourceId} for locale bundle ${locale}: ${error}`
        );
      }
    }
  }

  createEffect(async () => {
    // Generating bundles reads the selected locale, so this effect will trigger when it changes
    const l10n = new Localization(["messages.ftl"], generateBundles);
    setL10n(l10n);

    // Interesting read about directionality: https://www.w3.org/International/questions/qa-html-dir
    document.documentElement.dir = await l10n.formatValue(
      "language-text-directionality"
    );

    const newSelectedLocale = selectedLocale();
    if (newSelectedLocale) {
      await persistSelectedLocale(newSelectedLocale);
    }
  });

  return (
    <I18nContext.Provider
      value={[
        (id, args, fallbackMessage) => {
          const fallbackString =
            fallbackMessage === undefined ? id : fallbackMessage;

          const [translatedMessage, setTranslatedMessage] =
            createSignal(fallbackString);

          createEffect(async () => {
            let message;
            try {
              message = await l10n()?.formatValue(id, args);
            } catch (error) {
              console.error("Translation error:", error);
            } finally {
              setTranslatedMessage(
                message === undefined ? fallbackString : message
              );
            }
          });

          return translatedMessage;
        },
        {
          selectedLocale: selectedLocale,
          setSelectedLocale: setSelectedLocale
        }
      ]}
    >
      {props.children}
    </I18nContext.Provider>
  );
}

export function useI18n() {
  return useContext(I18nContext) as I18nContextValue;
}

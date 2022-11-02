import { Locale } from "../contexts/i18n";
import settingsStore from "./settingsStore";

export async function getSelectedLocale() {
  return (await settingsStore.get<Locale>("selectedLocale")) || undefined;
}

export async function setSelectedLocale(locale: Locale) {
  return settingsStore.set("selectedLocale", locale);
}

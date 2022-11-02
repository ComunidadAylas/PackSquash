import packSquashLogoUrl from "../assets/packsquash_logo.webp";
import { useI18n } from "../contexts/i18n";

export default () => {
  const [l10n] = useI18n();

  const packSquashLogoAlt = l10n("app-logo-alt");

  return (
    <img
      src={packSquashLogoUrl}
      alt={packSquashLogoAlt()}
      class="mx-auto h-full max-h-96 min-h-0 object-contain drop-shadow-md"
      draggable={false}
    />
  );
};

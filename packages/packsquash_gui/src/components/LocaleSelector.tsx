import { For, getOwner, runWithOwner } from "solid-js";
import { Translate } from "phosphor-solid";
import { availableLocales, Locale, useI18n } from "../contexts/i18n";
import { toast } from "solid-toast";
import { tippy as solid_tippy } from "solid-tippy";

// Hack to make the TypeScript compiler happy with the use:tippy directive
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const tippy = solid_tippy;

export default () => {
  const [l10n, , { setSelectedLocale }] = useI18n();

  const componentOwner = getOwner();
  if (!componentOwner) {
    throw new Error("No component owner for LocaleSelector");
  }

  return (
    <div>
      <button
        class="h-12 w-12 rounded bg-packblue-100 drop-shadow-md hover:scale-110"
        title=""
        use:tippy={{
          hidden: true,
          props: {
            content: (
              <div class="grid grid-cols-2 place-items-center gap-2">
                <For each={Object.entries(availableLocales)}>
                  {(locale) => (
                    <button
                      class="rounded p-1 hover:cursor-pointer hover:bg-zinc-700"
                      lang={locale[0]}
                      onClick={() =>
                        runWithOwner(componentOwner, () => {
                          setSelectedLocale(locale[0] as Locale);

                          toast.dismiss("switched-to-language");

                          toast(l10n("switched-to-language"), {
                            icon: <Translate />,
                            unmountDelay: 0, // Prevents this toast message from being visibly translated after dismissal
                            id: "switched-to-language"
                          });
                        })
                      }
                    >
                      {locale[1].friendlyName} ({locale[0]})
                    </button>
                  )}
                </For>
              </div>
            ) as HTMLElement,
            placement: "bottom",
            animation: "scale-subtle",
            trigger: "click focus",
            interactive: true
          }
        }}
      >
        <Translate class="w-full p-2 text-zinc-50" size="auto" />
      </button>
    </div>
  );
};

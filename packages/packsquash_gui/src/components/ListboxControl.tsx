import {
  HeadlessDisclosureChild,
  Listbox,
  ListboxButton,
  ListboxOption,
  ListboxOptions
} from "solid-headless";
import { createEffect, createSignal, For, Show } from "solid-js";
import { CaretDown, CaretUp } from "phosphor-solid";

interface ListboxControlBaseProps {
  name: string;
  values: string[];
  required: boolean;
  onInvalidChange?: (nativeControl: HTMLInputElement) => void;
  placeholderText: string | (() => string);
}

interface SingleItemListboxControlProps extends ListboxControlBaseProps {
  multiple: false;
  defaultValue?: string;
  onChange?: (newValue: string | undefined) => void;
}

interface MultipleItemListboxControlProps extends ListboxControlBaseProps {
  multiple: true;
  defaultValue?: string[];
  onChange?: (newValue: string[] | undefined) => void;
}

let zIndex = 4096; // Some "on top of everything" stuff uses a Z-index of 9999
function wrappingDecrementZIndex() {
  const dec = zIndex - 1;
  return (zIndex = dec < 0 ? 4095 : dec);
}

export default (
  props: SingleItemListboxControlProps | MultipleItemListboxControlProps
) => {
  const multiple = isMultiple(props);
  const [selectedValue, setSelectedValue] = createSignal(props.defaultValue);

  const nativeControl = (
    <input
      type="text"
      class="hidden"
      name={props.name}
      required={props.required}
    />
  ) as HTMLInputElement;

  let firstValueChange = true;
  createEffect(() => {
    const newValue = selectedValue();
    nativeControl.value =
      typeof newValue == "string" ? newValue : newValue?.join(",") ?? "";

    if (!firstValueChange && !nativeControl.validity.valid) {
      props.onInvalidChange?.(nativeControl);
    }

    firstValueChange = false;
  });

  const selectedValueIsEmpty = () => {
    const value = selectedValue();
    return !value || value.length == 0;
  };

  const selectedValuesText = () => {
    const value = selectedValue();
    if (typeof value == "string") {
      return value;
    } else if (Array.isArray(value) && value.length > 0) {
      return value.join(", ");
    } else {
      return typeof props.placeholderText == "function"
        ? props.placeholderText()
        : props.placeholderText;
    }
  };

  return (
    <Listbox
      as="div"
      value={selectedValue() as never}
      {...(multiple ? { multiple: true } : {})}
      onSelectChange={(optionValue: string | string[] | undefined) => {
        // We don't use an effect to prevent setting the default value on render
        props.onChange?.(optionValue as never);
        setSelectedValue(optionValue);
      }}
      defaultOpen={false}
      toggleable={multiple}
      class="relative bg-form-control-background drop-shadow-md"
      // We need to set the Z-index, or otherwise further elements in the DOM will
      // be drawn in front of our dropdown, which looks bad. Moreover, we need to
      // monotonically decrease the Z-index so that further listboxes do not occlude
      // this listbox
      style={{ "z-index": wrappingDecrementZIndex() }}
    >
      <ListboxButton
        type="button"
        class="flex w-full items-center justify-between gap-2 break-all p-1 text-center"
      >
        {({ isOpen }) => (
          <>
            <span
              classList={{
                "text-form-control-placeholder-color": selectedValueIsEmpty()
              }}
            >
              {selectedValuesText}
            </span>
            <Show
              when={isOpen()}
              keyed={false}
              fallback={<CaretDown size="1em" class="min-w-[1em]" />}
            >
              <CaretUp size="1em" class="min-w-[1em]" />
            </Show>
          </>
        )}
      </ListboxButton>

      <HeadlessDisclosureChild>
        {({ isOpen }) => (
          <Show when={isOpen()} keyed={false}>
            {/* Setting the background again is necessary to work around WebKit bugs */}
            <ListboxOptions class="absolute right-0 w-full min-w-max break-all border-t-2 border-t-zinc-300 bg-form-control-background drop-shadow-md">
              <For each={props.values}>
                {(value) => (
                  <ListboxOption value={value}>
                    {({ isSelected }) => (
                      <div
                        class="p-2 transition hover:bg-zinc-300"
                        classList={{ "font-bold": isSelected() }}
                      >
                        {value}
                      </div>
                    )}
                  </ListboxOption>
                )}
              </For>
            </ListboxOptions>
          </Show>
        )}
      </HeadlessDisclosureChild>

      {nativeControl}
    </Listbox>
  );
};

function isMultiple(
  props: SingleItemListboxControlProps | MultipleItemListboxControlProps
): props is MultipleItemListboxControlProps {
  return props.multiple;
}

import {
  createEffect,
  createSignal,
  For,
  getOwner,
  JSXElement,
  Owner,
  runWithOwner,
  Show
} from "solid-js";
import { JSONSchema7, JSONSchema7TypeName } from "json-schema";
import optionsSchemaJson from "../data/optionsSchema.json";
import { LocalizationProvider, useI18n } from "../contexts/i18n";
import { save } from "@tauri-apps/api/dialog";
import ConfigurationControlContainer from "../components/OptionControlContainer";
import { selectedPackPath } from "../views/PackSelection";
import { invoke } from "@tauri-apps/api";
import {
  HeadlessDisclosureChild,
  Listbox,
  ListboxButton,
  ListboxOption,
  ListboxOptions
} from "solid-headless";
import { CaretDown, CaretUp } from "phosphor-solid";
import { toast } from "solid-toast";
import ListboxControl from "../components/ListboxControl";

const PACK_DIRECTORY_OPTION = "pack_directory";
export { PACK_DIRECTORY_OPTION };

export function getOutputZipPath(
  packSquashOptions: Record<string, unknown>
): string {
  return (packSquashOptions["output_file_path"] ??
    optionsSchemaJson.properties.output_file_path.default) as string;
}

export function renderOptionsControls(): [
  JSXElement[],
  Partial<Record<string, number | boolean | string | Set<string>>>
] {
  const [l10n] = useI18n();

  // TypeScript outputs some funky type inference errors without this double-cast.
  // It was manually checked that the types do indeed overlap for practical cases
  const optionsSchema = optionsSchemaJson as unknown as JSONSchema7;

  const globalOptions = optionsSchema.properties;
  const requiredGlobalOptions = new Set(optionsSchema.required || []);

  const renderedControls = [];
  const globalOptionsObject = {};

  const componentOwner = getOwner();
  if (!componentOwner) {
    throw new Error("No component owner for options controls");
  }

  for (const globalOption in globalOptions) {
    // Exclude the pack directory option, which is handled in a special way by the GUI.
    // So is the "warnings are errors" option, which is meant for CLI usage
    if (
      globalOption == PACK_DIRECTORY_OPTION ||
      globalOption == "treat_asset_warnings_as_errors"
    ) {
      continue;
    }

    const optionSchema = globalOptions[globalOption] as JSONSchema7;
    const optionType = getOptionType(optionSchema);

    if (!optionType) {
      console.warn(
        `The JSON schema for option ${globalOption} is of unknown or null type, ignoring`
      );
      continue;
    }

    const isRequired = requiredGlobalOptions.has(globalOption);
    switch (optionType) {
      case "number":
        renderedControls.push(
          renderNumberOptionControl(
            l10n,
            globalOption,
            optionSchema,
            isRequired,
            globalOptionsObject,
            componentOwner
          )
        );
        break;
      case "integer":
        renderedControls.push(
          renderIntegerOptionControl(
            l10n,
            globalOption,
            optionSchema,
            isRequired,
            globalOptionsObject,
            componentOwner
          )
        );
        break;
      case "boolean":
        renderedControls.push(
          renderBooleanOptionControl(
            l10n,
            globalOption,
            optionSchema,
            isRequired,
            globalOptionsObject,
            componentOwner
          )
        );
        break;
      case "string":
        renderedControls.push(
          renderStringOptionControl(
            l10n,
            globalOption,
            optionSchema,
            isRequired,
            globalOptionsObject,
            componentOwner
          )
        );
        break;
      case "array":
        renderedControls.push(
          renderStringSetOptionControl(
            l10n,
            globalOption,
            optionSchema,
            isRequired,
            globalOptionsObject,
            componentOwner
          )
        );
        break;
    }
  }

  return [renderedControls, globalOptionsObject];
}

function renderNumberOptionControl(
  l10n: LocalizationProvider,
  optionName: string,
  optionSchema: JSONSchema7,
  isRequired: boolean,
  optionsObject: Partial<Record<string, number>>,
  componentOwner: Owner
): JSXElement {
  return (
    <ConfigurationControlContainer optionName={optionName}>
      {(labelId) => (
        <input
          name={optionName}
          aria-labelledby={labelId}
          type="number"
          step="0.001"
          min={optionSchema.minimum}
          max={optionSchema.maximum}
          placeholder={optionSchema.default?.toString()}
          value={isRequired ? optionSchema.default?.toString() : undefined}
          required={isRequired}
          onChange={(event) =>
            (optionsObject[optionName] = event.currentTarget.valueAsNumber)
          }
          onBlur={(event) =>
            showValidationErrorToast(event, l10n, componentOwner)
          }
        />
      )}
    </ConfigurationControlContainer>
  );
}

function renderIntegerOptionControl(
  l10n: LocalizationProvider,
  optionName: string,
  optionSchema: JSONSchema7,
  isRequired: boolean,
  optionsObject: Partial<Record<string, number>>,
  componentOwner: Owner
): JSXElement {
  const defaultValue = optionSchema.default as number | undefined;

  let minimum: number | undefined;
  let maximum: number | undefined;

  // First, get the minimum and maximum inclusive values for this integer from its format.
  // JavaScript has a double-precision float number type, so (u)int32 is the highest Rust
  // (unsigned) integer data type we can represent without precision loss
  if (optionSchema.format) {
    switch (optionSchema.format) {
      case "uint8":
        minimum = 0;
        maximum = 255;
        break;
      case "uint16":
        minimum = 0;
        maximum = 65535;
        break;
      case "uint32":
      case "uint": // Rust usize. We can assume pointers at least 32-bit wide
        minimum = 0;
        maximum = 4294967295;
        break;
      case "int8":
        minimum = -128;
        maximum = 127;
        break;
      case "int16":
        minimum = -32768;
        maximum = 32767;
        break;
      case "int32":
      case "int": // Rust isize. We can assume pointers at least 32-bit wide
        minimum = -2147483648;
        maximum = 2147483647;
        break;
      default:
        console.warn(
          `Unsupported integer format in JSON schema for option ${optionName}:`,
          optionSchema.format
        );
    }
  }

  minimum = optionSchema.minimum || minimum;
  maximum = optionSchema.maximum || maximum;

  return (
    <ConfigurationControlContainer optionName={optionName}>
      {(labelId) => (
        <input
          name={optionName}
          aria-labelledby={labelId}
          type="number"
          min={minimum}
          max={maximum}
          placeholder={defaultValue?.toString()}
          value={isRequired ? optionSchema.default?.toString() : undefined}
          required={isRequired}
          onChange={(event) =>
            (optionsObject[optionName] = event.currentTarget.valueAsNumber)
          }
          onBlur={(event) =>
            showValidationErrorToast(event, l10n, componentOwner)
          }
        />
      )}
    </ConfigurationControlContainer>
  );
}

function renderBooleanOptionControl(
  l10n: LocalizationProvider,
  optionName: string,
  optionSchema: JSONSchema7,
  isRequired: boolean,
  optionsObject: Partial<Record<string, boolean>>,
  componentOwner: Owner
): JSXElement {
  return (
    <ConfigurationControlContainer optionName={optionName}>
      {(labelId) => (
        <input
          name={optionName}
          aria-labelledby={labelId}
          type="checkbox"
          checked={optionSchema.default as boolean | undefined}
          required={isRequired}
          onChange={(event) =>
            (optionsObject[optionName] = event.currentTarget.checked)
          }
          onBlur={(event) =>
            showValidationErrorToast(event, l10n, componentOwner)
          }
        />
      )}
    </ConfigurationControlContainer>
  );
}

function renderStringOptionControl(
  l10n: LocalizationProvider,
  optionName: string,
  optionSchema: JSONSchema7,
  isRequired: boolean,
  optionsObject: Partial<Record<string, string>>,
  componentOwner: Owner
): JSXElement {
  // It's convenient to handle file paths like if they were another type
  if (optionSchema.format === "maybe_nonexistent_file_path") {
    return renderSavePathOptionControl(
      l10n,
      optionName,
      optionSchema,
      isRequired,
      optionsObject,
      componentOwner
    );
  }

  const valueSet = allowedStringTypeValues(optionSchema);

  if (valueSet === undefined) {
    // No specific set of values allowed. Anything goes
    return (
      <ConfigurationControlContainer optionName={optionName}>
        {(labelId) => (
          <input
            name={optionName}
            aria-labelledby={labelId}
            type="text"
            minLength={optionSchema.minLength}
            maxLength={optionSchema.maxLength}
            pattern={optionSchema.pattern}
            placeholder={optionSchema.default?.toString()}
            {...(isRequired && optionSchema.default
              ? { value: optionSchema.default.toString() }
              : {})}
            required={isRequired}
            onChange={(event) =>
              (optionsObject[optionName] = event.currentTarget.value)
            }
            onBlur={(event) =>
              showValidationErrorToast(event, l10n, componentOwner)
            }
          />
        )}
      </ConfigurationControlContainer>
    );
  } else {
    // Only a finite set of values is allowed. Render listbox component
    return (
      <ConfigurationControlContainer
        optionName={optionName}
        disableChildrenClickEvents={true} // Prevent <Listbox> interaction from toggling the description
      >
        {() => (
          <ListboxControl
            values={valueSet}
            multiple={false}
            name={optionName}
            placeholderText={l10n("packsquash-option-select-placeholder")}
            required={isRequired}
            onChange={(optionValue) =>
              (optionsObject[optionName] = optionValue)
            }
            onInvalidChange={(nativeControl) =>
              showValidationErrorToast(
                Object.assign(new Event(""), { currentTarget: nativeControl }),
                l10n,
                componentOwner
              )
            }
          />
        )}
      </ConfigurationControlContainer>
    );
  }
}

function renderSavePathOptionControl(
  l10n: LocalizationProvider,
  optionName: string,
  optionSchema: JSONSchema7,
  isRequired: boolean,
  optionsObject: Partial<Record<string, string>>,
  componentOwner: Owner
): JSXElement {
  const defaultPath = optionSchema.default?.toString();

  return (
    <ConfigurationControlContainer optionName={optionName}>
      {(labelId) => (
        <input
          name={optionName}
          aria-labelledby={labelId}
          type="text"
          placeholder={defaultPath}
          onBeforeInput={(event) => {
            // Simulate click for accessibility. It'd be better to use onFocus
            // instead, but toggling the save dialog loses and regains the focus,
            // which is even worse and can freeze the application
            event.currentTarget.click();
            event.preventDefault();
          }}
          onClick={async (event) => {
            // The event target must be read before awaiting, or else its reference
            // may be stale by the time we use it, no longer pointing to the input.
            // This happens on Linux WebKit at least
            const input = event.currentTarget;

            const path = await save({
              defaultPath: await invoke<string>("get_parent_path", {
                path: selectedPackPath
              })
            });

            // Do not change the value if the user did not select a path (empty string)
            if (path) {
              optionsObject[optionName] = input.value = path;
            }
          }}
          required={isRequired}
          onBlur={(event) =>
            showValidationErrorToast(event, l10n, componentOwner)
          }
        />
      )}
    </ConfigurationControlContainer>
  );
}

function renderStringSetOptionControl(
  l10n: LocalizationProvider,
  optionName: string,
  optionSchema: JSONSchema7,
  isRequired: boolean,
  optionsObject: Partial<Record<string, string[]>>,
  componentOwner: Owner
): JSXElement {
  if (
    typeof optionSchema.items != "object" ||
    Array.isArray(optionSchema.items)
  ) {
    console.warn(
      `The JSON schema of option ${optionName} has an untyped or multityped array type, ignoring`
    );
    return <></>;
  }

  if (!optionSchema.uniqueItems) {
    console.warn(
      `The JSON schema of option ${optionName} has an array of non-unique items, ignoring`
    );
    return <></>;
  }

  const valueSet = allowedStringTypeValues(optionSchema.items);
  if (!valueSet) {
    console.warn(
      `The JSON schema of option ${optionName} does not have a set of allowed strings, ignoring`
    );
    return <></>;
  }

  return (
    <ConfigurationControlContainer
      optionName={optionName}
      disableChildrenClickEvents={true} // Prevent <ListboxControl> interaction toggling the description
    >
      {() => (
        <ListboxControl
          values={valueSet}
          multiple={true}
          name={optionName}
          placeholderText={l10n("packsquash-option-select-placeholder")}
          defaultValue={[]}
          required={isRequired}
          onChange={(optionValue) => (optionsObject[optionName] = optionValue)}
          onInvalidChange={(nativeControl) =>
            showValidationErrorToast(
              Object.assign(new Event(""), { currentTarget: nativeControl }),
              l10n,
              componentOwner
            )
          }
        />
      )}
    </ConfigurationControlContainer>
  );
}

function showValidationErrorToast(
  event: Event & { currentTarget: HTMLInputElement },
  l10n: LocalizationProvider,
  componentOwner: Owner
) {
  if (!event.currentTarget.validity.valid) {
    runWithOwner(componentOwner, () => {
      toast.error(l10n("configuration-screen-invalid-custom-option-error"));
    });
  }
}

function getOptionType(
  optionSchema: JSONSchema7
): JSONSchema7TypeName | undefined {
  const typeSchemas = [optionSchema, ...(optionSchema.anyOf ?? [])].filter(
    (schema): schema is JSONSchema7 => typeof schema == "object"
  );
  let optionType: JSONSchema7TypeName | undefined = undefined;

  for (const typeSchema of typeSchemas) {
    // An option may be of several types. For now, support a single basic data type,
    // optionally combined with null
    let possibleTypes =
      typeof typeSchema.type == "string" ? [typeSchema.type] : typeSchema.type;

    // If we don't have type information, try to guess it from subschemas
    if (!possibleTypes && allowedStringTypeValues(optionSchema)) {
      possibleTypes = ["string"];
    }

    // Get the actual basic data type, taking into account that it might be nullable
    // (i.e., the list may contain the null type)
    for (const possibleType of possibleTypes ?? []) {
      if (!optionType && possibleType != "null") {
        optionType = possibleType;
        break;
      }
    }

    // Get the type from subschemas if necessary
    if (!optionType) {
      optionType = getOptionType(typeSchema);
    }

    // If we finally know a type, there's nothing more to do
    if (optionType) {
      break;
    }
  }

  return optionType;
}

function allowedStringTypeValues(
  typeSchema: JSONSchema7
): string[] | undefined {
  return (
    stringEnumToAllowedValues(typeSchema) ||
    oneOfStringEnumToAllowedValues(typeSchema)
  );
}

function stringEnumToAllowedValues(
  enumTypeSchema: JSONSchema7
): string[] | undefined {
  return enumTypeSchema.type == "string"
    ? enumTypeSchema.enum?.filter(
        (value): value is string => typeof value == "string"
      )
    : undefined;
}

function oneOfStringEnumToAllowedValues(
  typeSchema: JSONSchema7
): string[] | undefined {
  let allowedValues: string[] | undefined = undefined;

  for (const subtypeSchema of typeSchema.oneOf ?? []) {
    if (typeof subtypeSchema == "object") {
      const subtypeAllowedValues = stringEnumToAllowedValues(subtypeSchema);
      if (subtypeAllowedValues) {
        (allowedValues ??= []).push(...subtypeAllowedValues);
      }
    }
  }

  return allowedValues;
}

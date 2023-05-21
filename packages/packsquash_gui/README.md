## Usage

Those templates dependencies are maintained via [pnpm](https://pnpm.io) via `pnpm up -Lri`.

This is the reason you see a `pnpm-lock.yaml`. That being said, any package manager will work. This file can be safely be removed once you clone a template.

```bash
$ npm install # or pnpm install or yarn install
```

### Learn more on the [Solid Website](https://solidjs.com) and come chat with us on our [Discord](https://discord.com/invite/solidjs)

## Available Scripts

In the project directory, you can run:

### `npm dev` or `npm start`

Runs the app in the development mode.<br>
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.<br>

### `npm run build`

Builds the app for production to the `dist` folder.<br>
It correctly bundles Solid in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.<br>
Your app is ready to be deployed!

## Deployment

You can deploy the `dist` folder to any static host provider (netlify, surge, now, etc.)

## Localization

The following tasks must be carried out in order to localize the app for a new
language.

- [ ] Add the new target language on
  [Crowdin](https://crowdin.com/project/packsquash), allowing translators to
  localize:
  - In-app text strings, which use the [Fluent localization framework](https://projectfluent.org/).
  - Windows Installer MSI strings, which use the [WiX toolset localization
    extension](https://wixtoolset.org/docs/v3/howtos/ui_and_localization/make_installer_localizable/).
    It should be noted that Windows Installer only supports a fixed set of
    languages, so some languages may not be able to have a localized installer.

- [ ] Add the new language to the `LOCALES` array defined in
  `src/contexts/i18n.tsx` and modify `AvailableLocales` accordingly. Languages
  are roughly ordered by their number of speakers.

- [ ] Add the new language to the `AdditionalLanguages` array defined in the
  `src-tauri/bundles/wix/build_multilingual_installer.ps1` script to register
  the new Windows Installer MSI language, if applicable. Also add it to tauri >
  bundle > windows > wix > language at `src-tauri/tauri.conf.json`.

- [ ] Manually localize the Linux desktop entry comment string at
  `src-tauri/bundles/deb/org.aylas.packsquash.gui.desktop` for the new language.

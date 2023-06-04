import Page from "./Page";

import scriptClick from "../util/scriptClick";

export default new (class extends Page {
  urlSlug = "/";

  get startButton() {
    return $('button[data-test-id="start"]');
  }

  get aboutButton() {
    return $('a[data-test-id="about"]');
  }

  public async selectLocale(locale: string) {
    await scriptClick(await $('button[data-test-id="locale-selector"]'));
    await scriptClick(await $(`button=${locale}`));
  }
})();

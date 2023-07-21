import { Given, When, Then } from "@wdio/cucumber-framework";

import Page from "../pageobjects/Page";
import HomePage from "../pageobjects/HomePage";
import AboutPage from "../pageobjects/AboutPage";

import scriptClick from "../util/scriptClick";

const pages: Record<string, Page> = {
  home: HomePage,
  about: AboutPage
};

Given(/^I have just opened the (\w+) page$/, async (page) => {
  await pages[page].open();
});

Given(/^I am at the (\w+) page$/, async (page) => {
  if (!(await pages[page].isCurrent())) {
    await pages[page].open();
  }
});

When(/^I select the "(.+)" locale$/, (locale) => HomePage.selectLocale(locale));

When(/^I click the (\w+) button$/, async (buttonId) => {
  for (const page of Object.values(pages)) {
    if (await page.isCurrent()) {
      await scriptClick(await (page as never)[`${buttonId}Button`]);
      return;
    }
  }
});

Then(/^[Tt]he start button text is "(.+)"$/, async (text) =>
  expect(await HomePage.startButton).toHaveText(text)
);

Then(/^I move to the (\w+) page$/, async (page) =>
    expect(await pages[page].isCurrent()).toBeTruthy()
);

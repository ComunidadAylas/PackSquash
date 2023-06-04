// Workaround for issue with Tauri's WRY and WebKit webdriver:
// https://github.com/tauri-apps/tauri/issues/6541
export default async (element: WebdriverIO.Element) => {
  await element.waitForClickable();
  await browser.execute("arguments[0].click()", element);
};

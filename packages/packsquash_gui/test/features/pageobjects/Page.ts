/// Base class for [page objects](https://www.selenium.dev/documentation/test_practices/encouraged/page_object_models/)
/// representing different views under test of the application.
export default abstract class Page {
  protected abstract readonly urlSlug: string;

  /// Brings this page into view.
  public async open() {
    await browser.url(`#${this.urlSlug}`);
  }

  /// Checks whether this page is the one currently shown.
  public async isCurrent() {
    const currentUrl = await browser.getUrl();
    return (
      currentUrl.endsWith(`#${this.urlSlug}`) ||
      (this.urlSlug == "/" && !currentUrl.match(/#.+$/))
    );
  }
}

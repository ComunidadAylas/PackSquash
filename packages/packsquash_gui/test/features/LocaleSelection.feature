Feature: Locale selection

  Scenario Outline: Select locale - <locale>
    #Given I have just opened the home page
    When I select the "<locale>" locale
    Then the start button text is "<startButtonText>"

    Examples:
      | locale       | startButtonText |
      | English (en) | Start           |
      | Español (es) | Empezar         |

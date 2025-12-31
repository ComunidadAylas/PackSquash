#!/bin/sh -e
mkdir -p target/debian

# Generate extended package description from README.md
python3 - <<'PY_CONVERSION_CODE'
try:
    from markdown_it import MarkdownIt
except ModuleNotFoundError as e:
    print(
        "❌ The markdown-it-py package is required. "
        "Install it with 'pip install markdown-it-py' or your distribution's package "
        "(e.g., 'apt install python3-markdown-it' on Debian-based distributions).\n"
    )
    raise e

import textwrap

from inspect import getmembers, ismethod


# Based on https://github.com/elespike/mdit_plain, but simplified and tweaked
# to better suit our use case and not require any additional packages
class RendererPlain:
    __output__ = "plain"

    def __init__(self, parser=None, stop_at_content=None):
        self._stop_at_content = stop_at_content
        self._stopped = False
        self.rules = {
            k: v
            for k, v in getmembers(self, predicate=ismethod)
            if not (k.startswith("render") or k.startswith("_"))
        }

    def render(self, tokens, options, env):
        result = ""

        for i, token in enumerate(tokens):
            if self._stopped:
                break

            if (
                self._stop_at_content is not None
                and token.content
                and self._stop_at_content == token.content
            ):
                self._stopped = True
                break

            rule = self.rules.get(token.type, self.skip_rendering)
            result += rule(tokens, i, options, env)
            if token.children is not None:
                result += self.render(token.children, options, env)

        return result.strip()

    def skip_rendering(self, tokens, i, options, env):
        return ""

    # Token renderers, named after the token type they handle

    def code_inline(self, tokens, i, options, env):
        return tokens[i].content

    def code_block(self, tokens, i, options, env):
        return f"\n{tokens[i].content}\n"

    def fence(self, tokens, i, options, env):
        return f"\n{tokens[i].content}\n"

    def hardbreak(self, tokens, i, options, env):
        return " "

    def softbreak(self, tokens, i, options, env):
        return " "

    def bullet_list_close(self, tokens, i, options, env):
        if len(tokens) == i + 1 or "list" in tokens[i + 1].type:
            return ""
        return "\n"

    def heading_close(self, tokens, i, options, env):
        return "\n"

    def heading_open(self, tokens, i, options, env):
        return "\n"

    def list_item_open(self, tokens, i, options, env):
        next_token = tokens[i + 1]
        if hasattr(next_token, "hidden") and not next_token.hidden:
            return ""
        return "\n- "

    def ordered_list_close(self, tokens, i, options, env):
        if len(tokens) == i + 1 or "list" in tokens[i + 1].type:
            return ""
        return "\n"

    def paragraph_open(self, tokens, i, options, env):
        if tokens[i].hidden:
            return ""
        return "\n"

    def paragraph_close(self, tokens, i, options, env):
        if tokens[i].hidden:
            return ""
        return "\n"

    def text(self, tokens, i, options, env):
        return tokens[i].content


with open("README.md", "rt", encoding="utf-8") as f:
    plain_readme_text = MarkdownIt(
        # Skip rendering everything after the contributors section because
        # it has a complex layout that does not translate well to plain text
        renderer_cls=lambda parser: RendererPlain(
            parser, stop_at_content="✨ Contributors"
        )
    ).render(f.read())

# The lines in the rendered plain text are not wrapped to any column width,
# so do so now to 80 chars to guarantee better readability in a extended
# Debian package description
wrapped_readme_text = "\n".join(
    [
        "\n".join(textwrap.wrap(line, width=80))
        for line in plain_readme_text.splitlines()
    ]
)

with open(
    "target/debian/extended_package_description.txt", "wt", encoding="utf-8"
) as f:
    f.write(wrapped_readme_text)
PY_CONVERSION_CODE

# Build the Debian package
cargo deb --no-strip "$@"

# Clean up any temporary files under target to stay away from poisoning caches of that folder
rm -f target/debian/extended_package_description.txt || true

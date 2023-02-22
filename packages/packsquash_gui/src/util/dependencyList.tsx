import cargoDependencies from "../data/cargoDependencies.json";
import nodeDependencies from "../data/nodeDependencies.json";

import { For } from "solid-js";

interface Dependency {
  name: string,
  version: string,
  license: string,
  homepage: string,
  authors: string[]
}

const dependencyListItems = cargoDependencies.concat(nodeDependencies.map((dependency): Dependency => ({
  name: dependency.name,
  version: dependency.installedVersion,
  license: dependency.licenseType,
  homepage: dependency.link.replace(/^git\+/, ""),
  authors: [dependency.author]
}))).map((dependency, i, arr) => <li>
  <article>
    <p class="mb-2"><cite>{dependency.name} v{dependency.version}</cite></p>
    <p class="mb-2 text-xs">{dependency.license}</p>
    <For each={dependency.authors}>
      {(author) => <address class="text-sm">{author}</address>}
    </For>
    <p classList={{ "mb-4": i != arr.length - 1 }}>
      <a href={dependency.homepage} class="text-sm">{dependency.homepage}</a>
    </p>
  </article>
</li>);

export function renderDependencyListItems() {
  return dependencyListItems;
}

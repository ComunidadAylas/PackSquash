import cargoDependencies from "../data/cargoDependencies.json";
import nodeDependencies from "../data/nodeDependencies.json";

interface Dependency {
  name: string;
  version: string;
  license: string;
  homepage: string;
  authors: string[];
}

const dependencyListItems = cargoDependencies
  .concat(
    nodeDependencies.map(
      (dependency): Dependency => ({
        name: dependency.name,
        version: dependency.installedVersion,
        license: dependency.licenseType,
        homepage: dependency.link.replace(/^git\+/, ""),
        authors: [dependency.author]
      })
    )
  )
  .map((dependency, i, arr) => (
    <li>
      <article>
        <p class="mb-2">
          <cite>
            {/* @once */ dependency.name} v{/* @once */ dependency.version}
          </cite>
        </p>
        <p class="mb-2 text-xs">{/* @once */ dependency.license}</p>
        {
          /* @once */ dependency.authors.map((author) => (
            <address class="text-sm">{author}</address>
          ))
        }
        <p classList={/* @once */ { "mb-4": i != arr.length - 1 }}>
          <a href={/* @once */ dependency.homepage} class="text-sm">
            {/* @once */ dependency.homepage}
          </a>
        </p>
      </article>
    </li>
  ));

export function renderDependencyListItems() {
  return dependencyListItems;
}

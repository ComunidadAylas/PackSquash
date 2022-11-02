import { invoke } from "@tauri-apps/api";

let buildVersion: string | undefined;
try {
  buildVersion = await invoke<string>("app_build_version");
} catch {
  // Ignore failure
}

export { buildVersion };

export function getClosestGitObjectForBuild(): string | undefined {
  let closestGitObject: string | undefined;

  // The build version string is in one of these formats:
  // - A single git tag we can use directly.
  // - A git tag, followed by a dash, a number, another dash, the last commit hash prefixed by "g",
  //   and an optionl "-custom" suffix.
  const buildInfo = buildVersion?.match(
    /^(?<tag>[^-]+)(?:-[0-9]+-g(?<commitHash>[0-9A-Fa-f]+)(?:-custom)?)?$/
  );
  if (buildInfo) {
    const buildTag = buildInfo.groups?.tag;
    const commitHash = buildInfo.groups?.commitHash;

    if (commitHash) {
      closestGitObject = commitHash;
    } else if (buildTag) {
      closestGitObject = buildTag;
    }
  }

  return closestGitObject;
}

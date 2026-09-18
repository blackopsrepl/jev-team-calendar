// Release configuration for commit-and-tag-version.
//
// Every version surface the app publishes is declared here so a release bumps
// Cargo metadata, both lockfiles, and the Python tool project in one commit.

const tomlPackageVersion = {
  readVersion: (contents) => contents.match(/^version = "([^"]+)"/m)[1],
  writeVersion: (contents, version) =>
    contents.replace(/^version = "([^"]+)"/m, `version = "${version}"`),
};

const lockPackageVersion = (packageName) => ({
  readVersion: (contents) =>
    contents.match(
      new RegExp(`name = "${packageName}"\\nversion = "([^"]+)"`)
    )[1],
  writeVersion: (contents, version) =>
    contents.replace(
      new RegExp(`(name = "${packageName}"\\nversion = ")[^"]+(")`),
      `$1${version}$2`
    ),
});

const versionSurfaces = [
  { filename: "Cargo.toml", updater: tomlPackageVersion },
  { filename: "Cargo.lock", updater: lockPackageVersion("jev-team-calendar") },
  { filename: "pyproject.toml", updater: tomlPackageVersion },
  {
    filename: "uv.lock",
    updater: lockPackageVersion("jev-team-calendar-tools"),
  },
];

module.exports = {
  tagPrefix: "v",
  releaseCommitMessageFormat: "chore(release): {{currentTag}}",
  commitUrlFormat:
    "https://github.com/blackopsrepl/jev-team-calendar/commit/{{hash}}",
  compareUrlFormat:
    "https://github.com/blackopsrepl/jev-team-calendar/compare/{{previousTag}}...{{currentTag}}",
  issueUrlFormat:
    "https://github.com/blackopsrepl/jev-team-calendar/issues/{{id}}",
  types: [
    { type: "feat", section: "Features" },
    { type: "fix", section: "Bug Fixes" },
    { type: "perf", section: "Performance Improvements" },
    { type: "refactor", section: "Refactors" },
    { type: "docs", section: "Documentation" },
    { type: "build", section: "Build System" },
    { type: "ci", section: "Continuous Integration" },
    { type: "revert", section: "Reverts" },
    { type: "test", section: "Tests", hidden: true },
    { type: "chore", section: "Chores", hidden: true },
  ],
  packageFiles: versionSurfaces,
  bumpFiles: versionSurfaces,
};

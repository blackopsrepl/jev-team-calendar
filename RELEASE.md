# Release

Releases are cut from conventional commits with
[`commit-and-tag-version`](https://github.com/absolute-version/commit-and-tag-version).
The tool owns the version bump, the `CHANGELOG.md` entry, the release commit,
and the tag. Never edit `CHANGELOG.md` or a version surface by hand.

## Version surfaces

`.versionrc.js` declares every surface a release must bump:

| File | What it carries |
|------|-----------------|
| `Cargo.toml` | the application crate version reported by `/info` |
| `Cargo.lock` | the locked `jev-team-calendar` package version |
| `pyproject.toml` | the Python tools project version |
| `uv.lock` | the locked `jev-team-calendar-tools` version |

## Pre-release gate

Run the full gate on the exact tree that will be released:

```bash
make check                          # solverforge check + cargo check
make test                           # solverforge test + uv run pytest
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
node --check static/app.js
```

The worktree must be clean and every change being released must already be
committed.

## Cut the release

```bash
# Named target version:
commit-and-tag-version --release-as vX.Y.Z

# Or let conventional commits choose (fix = patch, feat = minor):
commit-and-tag-version
```

Verify the result before pushing:

```bash
git show --stat --oneline HEAD   # version files + CHANGELOG.md in one commit
git tag --points-at HEAD         # the intended tag
git status --short               # clean
```

## Push and prove

The repository publishes to two remotes:

| Remote | Target |
|--------|--------|
| `origin` | local Forgejo `http://vigilance:3002/blackopsrepl/jev-team-calendar` |
| `github` | GitHub `https://github.com/blackopsrepl/jev-team-calendar` |

Push the branch and the tag to both, then verify the remote heads:

```bash
git push origin main
git push origin --tags
git push github main
git push github --tags
```

For the local Forgejo, use the `operate-local-forgejo` verification flow
(`scripts/verify-publication.sh`) to confirm equal full SHAs, `0 0` divergence,
and anonymous read-back. Then confirm the Actions runs from
`.github/workflows/ci.yml`, `.forgejo/workflows/ci.yml`, and the release
workflows reach a terminal success state.

## Automation

- `.github/workflows/ci.yml` and `.forgejo/workflows/ci.yml` run formatting,
  Clippy, tests, the model check when the CLI is available, and the browser
  script syntax check on every push and pull request to `main`.
- `.github/workflows/release.yml` creates a GitHub release with generated notes
  when a `v*` tag is pushed.
- `.forgejo/workflows/release.yml` creates the matching Forgejo release from the
  same tag, using the changelog section for that version.

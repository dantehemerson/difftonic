---
name: release-homebrew
description: Use ONLY when releasing a new Difftonic version through GitHub Releases and the Homebrew tap, including SemVer selection, release notes, version files, tags, and release verification. Do not use for crates.io or cargo publishing.
---

# Release Difftonic Through Homebrew

Release Difftonic from `dantehemerson/difftonic` through GitHub Releases and
`dantehemerson/homebrew-tap`. Update the Rust version files, but never publish
the crate to crates.io and never run `cargo publish`.

## Release Analysis

Before changing files or creating a release:

1. Confirm the current repository is `dantehemerson/difftonic`.
2. Check `git status`, the current branch, remotes, existing tags, and GitHub
   authentication.
3. Require a clean worktree. Do not stash, discard, or include unrelated
   changes. If it is dirty, stop and tell the user which files prevent the
   release.
4. Fetch `origin/main` and tags. Require local `main` to match `origin/main`.
   Do not work around branch protection or release from another branch.
5. Find the latest stable `vMAJOR.MINOR.PATCH` tag and verify its version
   matches both `Cargo.toml` and the Difftonic package entry in `Cargo.lock`.
6. Inspect every commit and the complete diff from that tag through `HEAD`.
   Do not infer release impact from commit subjects alone.
7. Stop if there are no releasable changes.

Prepare concise, user-facing release notes from those changes. Exclude merge
commits, version-bump bookkeeping, and internal details that do not help a
user. Group applicable entries under `Added`, `Changed`, `Fixed`, and
`Maintenance`. Omit empty sections. End with:

```markdown
**Full Changelog**: https://github.com/dantehemerson/difftonic/compare/PREVIOUS_TAG...NEW_TAG
```

## SemVer Recommendation

Recommend exactly one release type based on the highest-impact included
change:

- `major`: an intentional breaking CLI, configuration, output, installation,
  or compatibility change.
- `minor`: a backward-compatible user-facing feature or meaningful new
  capability.
- `patch`: a backward-compatible fix, performance improvement, documentation,
  test, build, packaging, or maintenance change.

Treat an explicit breaking change as major even while the project is on a
`0.x` version. Calculate the patch, minor, and major candidate versions from
the current version.

Before making any release change, show the user:

- The current version and latest tag.
- The commits and user-visible changes included in the release.
- The complete proposed GitHub Release description.
- The recommended release type, candidate version, and a short rationale.

Then use the question tool to ask the user to select `patch`, `minor`,
`major`, or cancel. Put the recommended option first and label it
`(Recommended)`. Always ask, even when the appropriate bump appears obvious.
The selected release type authorizes the release steps below; cancellation
must leave the repository unchanged.

## Create The Release

After the user selects a release type:

1. Calculate `NEW_VERSION` and `NEW_TAG=vNEW_VERSION`, then verify neither the
   local nor remote tag already exists and that a GitHub Release with that tag
   does not exist.
2. Update the package version in `Cargo.toml` and the Difftonic package version
   in `Cargo.lock`. Do not change dependency versions.
3. Run `cargo test --locked`. If it fails, stop before committing or tagging,
   leave the version edits visible, and report the failure.
4. Inspect the diff and stage only `Cargo.toml` and `Cargo.lock`.
5. Commit with `chore: release vNEW_VERSION` and create an annotated tag named
   `vNEW_VERSION` with message `Difftonic vNEW_VERSION`.
6. Push `main` and the tag to `origin` atomically. Do not force-push and do not
   bypass hooks or branch protection.

The existing `.github/workflows/release.yml` workflow triggered by the tag is
the source of truth for building the universal macOS archive, creating the
GitHub Release, and updating the Homebrew formula. Do not manually upload a
replacement archive or directly edit the tap during a normal release.

## Publish Notes And Verify

1. Locate the Release workflow run for the new tag, allowing for GitHub's
   scheduling delay, and wait for it with `gh run watch --exit-status`.
2. If the workflow fails, inspect and report the failing job. Do not delete
   the tag, release, commit, or assets automatically.
3. After the workflow succeeds, use `gh release edit` to set the release title
   to the new tag and replace the generated body with the exact release notes
   shown before approval. Use a temporary notes file when needed to preserve
   Markdown formatting, and remove it afterward.
4. Verify the GitHub Release is published and contains both the universal
   macOS archive and its `.sha256` file.
5. Verify `dantehemerson/homebrew-tap` has `Formula/difftonic.rb` on `main`
   with the new version, release URL, and checksum.
6. Report the new version, release URL, workflow result, and Homebrew formula
   status.

Never run `cargo publish`, create a crates.io release, or claim that the Rust
crate was published. Cargo is used only to test the source, and its manifest
and lockfile versions are kept synchronized for a possible future crates.io
release.

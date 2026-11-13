# Nodalia migration: Lámina

Category: secondary app.
Status: renamed to Lámina throughout, own release workflows, as of 2026-11-13. Evidence: the
pull request that adds this file.

## Current state

Pure-Rust presentation app (egui), MIT OR Apache-2.0. Cargo git dependencies come from
getnodalia/foundation and nodalia-design. The gate, FreeBSD, Nix, packaging lint, AGENTS.md and
git-sources checks call getnodalia/foundation's reusable workflows pinned by commit. Releases run
on Lámina's own workflows (`release-auto.yml` with Foundation's release-tag and release-publish,
then `release-linux.yml`, `release-macos.yml` and `release-windows.yml`). Every name is Lámina's,
internal ones included: `scripts/rebrand.py` with `.github/rebrand.toml` renames upstream's tree,
the weekly sync merges it renamed (`docs/upstream.md`), and CI fails if an old name comes back.

A new presentation is saved as PowerPoint (`.pptx`), the standard format, as Letra keeps `.docx`
(decision of 2026-11-13). Lámina's own format, which keeps everything, is `.lamina` and stays
one choice away in Save As; `.deckcraft` and `.slidecraft` files still open.

## Inherited dependencies still pending

| Dependency | Where | Replacement | Blocked on |
|---|---|---|---|
| upstream's repository (`docs/upstream.md`) | `.github/workflows/upstream-sync.yml` | none: the weekly sync reads upstream and renames it | — |

Resolved: craft-common-actions (release, contributors refresh, Windows ARM64) is gone; fonts and
the shared libraries come from getnodalia/foundation's vendor branches; the `[patch]` entries for
`storytold/*` in `Cargo.toml` only name the old home of crates fetched from getnodalia/foundation.

## Technical debt

- Windows ARM64 is not packaged yet (the inherited workflow was removed); `release-windows.yml`
  builds x64.
- The macOS bundle folder is `Lamina.app` (ASCII), so the in-app updater finds it whatever
  Unicode form the disk image stores; the name people see in the Dock and menu bar is Lámina.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, reusable workflows); getnodalia/nodalia-design
(theme, icons, `nodalia-update`).

## NDL work pending

The interface in the Nodalia Design Language: islands, the hamburger menu with About,
Preferences and Keyboard shortcuts, NDL window controls on Linux only, the global menu over D-Bus.

## Recommended order

1. The interface in NDL.
2. A Windows ARM64 package.

# Lámina upstream

| Field | Value |
|---|---|
| Upstream | `storytold/deckcraft` |
| Last observed | `0623b430324cf4c3d1962e6ae7885cf30c011c5e` |
| Last evaluated | `0623b430324cf4c3d1962e6ae7885cf30c011c5e` |
| Last integrated | `0623b430324cf4c3d1962e6ae7885cf30c011c5e` imported with full history on 2026-11-10 |
| Skipped | the community button and the brand files (Lámina has neither) |
| Adapted | every name (`scripts/rebrand.py`), the app icon, the Nodalia interface, PowerPoint as the format a new presentation is saved in |
| Conflicting | `README.md` (kept ours), `AGENTS.md` (kept ours) |

## How the sync works

Upstream's code arrives renamed: `scripts/rebrand.py` applies `.github/rebrand.toml` (DeckCraft
becomes Lámina, internal names included; links to the shared libraries point at
getnodalia/foundation) to upstream's tree, and the weekly sync
(`.github/workflows/upstream-sync.yml`) records the result on the `upstream-lamina` branch. Each
commit there has the previous mirror commit and the upstream commit it renames as parents, and
names that commit in an `Upstream-Commit:` trailer. The sync merges the newest mirror commit
into a branch and opens a pull request; files listed in `.github/upstream-keep` keep this
repository's version.

The mirror starts at the renamed `0623b430`, merged into `main` (with this repository's tree)
by the pull request that renamed the app. Before the first sync creates the `upstream-lamina`
branch, the workflow finds that commit through its trailer. `python3 scripts/rebrand.py check`
fails when an old name comes back; a line that must keep one says `rebrand: keep`.

## Names kept on purpose

- The native file's signature (the manifest's `"format": "deckcraft"` and the zip's mimetype
  entry): every file saved so far carries it.
- `.deckcraft` and `.slidecraft` files still open; new ones are `.lamina`.
- Upstream's copyright line in `NOTICE` and the licences, which the licences require.

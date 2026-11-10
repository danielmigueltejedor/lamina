<div align="center">
  <img src="https://raw.githubusercontent.com/danielmigueltejedor/lamina/main/assets/app-icon/hicolor/512x512/apps/ai.storyteller.deckcraft.png" width="176" height="176" alt="Lámina">
  <h1>Lámina</h1>
  <p><strong>A presentation application for the Nodalia suite, built on the DeckCraft engine.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/status-early%20stage-orange" alt="Early stage">
    <img src="https://img.shields.io/badge/engine-DeckCraft-7C1F41" alt="DeckCraft engine">
    <img src="https://img.shields.io/badge/Rust-native-dea584?logo=rust&logoColor=black" alt="Native Rust">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea44f" alt="MIT OR Apache-2.0">
  </p>

  <p>
    <a href="#overview">Overview</a> ·
    <a href="#what-you-get-today">Today</a> ·
    <a href="#build-and-run">Build</a> ·
    <a href="#project-status">Status</a> ·
    <a href="#keeping-up-with-upstream">Upstream</a> ·
    <a href="#the-nodalia-suite">Suite</a> ·
    <a href="#license">License</a>
  </p>
</div>

---

## Overview

**Lámina** is the presentation application of [Nodalia](https://getnodalia.com). It is built on the engine of
[DeckCraft](https://github.com/storytold/deckcraft), which lives in this repository and is kept up to
date with upstream.

> [!IMPORTANT]
> **Lámina is at an early stage.** What runs today is the upstream application.
> Nodalia's own interface is not wired in yet and there are no published builds.

## What you get today

- The DeckCraft engine.
- The upstream desktop application. Crate and binary names are still upstream's.
- The engine's own feature description, kept as [`docs/upstream-readme.md`](docs/upstream-readme.md).

## Build and run

You need a recent stable Rust toolchain.

```bash
git clone https://github.com/danielmigueltejedor/lamina.git
cd lamina
cargo run --release -p deckcraft
```

The crate is still called `deckcraft`; it will be renamed with the product.

## Project status

| Area | State |
|---|---|
| Engine | Imported from `storytold/deckcraft` and tracked in [`docs/upstream.md`](docs/upstream.md) |
| Product name | Pending: crates and binary still carry upstream's name (`deckcraft`) |
| App icon | Done: portrait of Cicero |
| Nodalia interface | Not started |
| Shared interface resources | Planned: the Nodalia apps will share interface components and styles |
| Installable packages | None published |

## Keeping up with upstream

[`docs/upstream.md`](docs/upstream.md) records which upstream commit is integrated. Updates are
merges of upstream's `main`, so Lámina's own changes are kept and conflicts are resolved in the open.

## App icon

Nodalia app icons are black-and-white portraits of historical figures over the app colour.
Lámina's is Cicero, the great orator. Provenance and licence are in [`assets/app-icon/`](assets/app-icon/).

## The Nodalia suite

Lámina is one of the applications of [Nodalia](https://getnodalia.com), an independent software
project developed in public by [Daniel Miguel Tejedor](https://danielmigueltejedor.com).

## License

`MIT OR Apache-2.0`, as upstream. The licence files and `NOTICE` of DeckCraft are kept.
See [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).

## Credits

Lámina would not exist without [DeckCraft](https://github.com/storytold/deckcraft) and its contributors.

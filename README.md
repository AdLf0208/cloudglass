# cloudglass (Horizon port)

A Horizon port of [cloudglass](Mrbysco/CloudGlass) — MIT.

**Status: first slices.** What is below is what the port actually does today.

## Implemented so far

- Each colour has:
- a `CloudGlassBlock` (`CloudGlassBlock.java:14-45`): `TransparentBlock` with
- `noCollission()`, `noOcclusion()`, `isValidSpawn(false)`,
- `isRedstoneConductor(false)`, `isSuffocating(false)`, `isViewBlocking(false)`,
- `strength(0.3F)`, `sound(SoundType.GLASS)` and `mapColor(dye)`
- (`CloudRegistry.java:31-45`); not waterloggable (`canPlaceLiquid`/
- `placeLiquid` return false);
- a self-drop loot table (`CloudLootProvider.java:29-34`);

## Not yet ported

- No panes.** Upstream 1.1.0 has no pane block, blockstate, model, recipe
- or lang key; the dispatch's "blocks/panes" was an assumption. The port
- ships the sixteen blocks the README and registry define.
- Template facts.** `minecraft:cobweb` is the only 26.3 block with a single
- state, an empty collision table, `can_occlude: false`, a `MODEL` render
- shape and an item. Two template facts differ from upstream's
- `noOcclusion()`: `dampening` is 1 and `skylight_down` is false (vanilla
- glass is 0/true), so cloud glass slightly attenuates skylight; and

## Install

Build the bundle with a native-mod capable launcher, or add the file from the
registry once it is listed. Pinned to Horizon `1d63e0a` and horizon-mixins
`5a64af93` for game 26.3.

## Build from source

```sh
cargo test
python3 tools/horizon.py check --horizon <horizon checkout>
python3 tools/horizon.py build --horizon <horizon checkout>
python3 tools/package.py
```

## Credits and licence

Original mod: Mrbysco/CloudGlass. Port: AdLf0208. Licence: MIT — see [LICENSE](LICENSE);
the original notices are preserved.

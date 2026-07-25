# wilhelmos-kiosk-demo

The **WilhelmOS reference kiosk application**: a fullscreen
[wilhelm_renderer](https://github.com/algonents/wilhelm_renderer) +
[Dear ImGui](https://github.com/ocornut/imgui) (via
[wilhelm_renderer_imgui](https://github.com/algonents/wilhelm_renderer_imgui))
demo that validates the WilhelmOS graphical kiosk stack end to end:

```
systemd → cage → wilhelmos-kiosk-demo → OpenGL → DRM/KMS → display
```

It is also the worked example for integrators: a WilhelmOS kiosk
application is a fullscreen binary crate exactly like this one, packaged
by a Yocto cargo recipe that pins a release tag of its repo (see
`meta-wilhelmos/recipes-graphics/wilhelmos-kiosk-demo/` in
[WilhelmOS](https://github.com/algonents/wilhelmos)).

## Running (desktop)

```
cargo run
```

Creates a fullscreen window on the primary monitor with a triangle and an
ImGui control panel. On a desktop this runs under your normal session; on
WilhelmOS it is launched by the cage compositor as the kiosk session.

## Releasing

1. All dependencies must be published crates.io versions — a tagged
   release must not contain git dependencies in its `Cargo.lock`.
2. `cargo build` to refresh `Cargo.lock`, commit it.
3. Tag `vX.Y.Z` and push the tag.
4. In WilhelmOS: bump the recipe `SRCREV`/`PV` to the tag and regenerate
   the crates include (`bitbake -c update_crates wilhelmos-kiosk-demo`).

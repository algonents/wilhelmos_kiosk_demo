# kiosk_app_demo

The **WilhelmOS reference kiosk application**: a fullscreen demo built on
the [wilhelmos_kiosk](https://github.com/algonents/wilhelmos_kiosk)
application framework, validating the WilhelmOS graphical kiosk stack end
to end:

```
systemd → cage → kiosk_app_demo → OpenGL → DRM/KMS → display
```

## What this repo is (and isn't)

This repo is deliberately a **packaging shell**. The application code
mirrors the framework's own
[`hello_kiosk`](https://github.com/algonents/wilhelmos_kiosk/blob/master/examples/hello_kiosk.rs)
example — *that* is the canonical "how do I write a kiosk app". What this
repo demonstrates is the **packaging contract** an integrator copies:

- a standalone fullscreen binary crate,
- a committed `Cargo.lock` with **crates.io dependencies only**,
- release tags,
- consumed by a Yocto cargo recipe that pins the tag and provides
  `virtual/kiosk-app` / `/usr/libexec/kiosk-app` (see
  `meta-wilhelmos/recipes-graphics/kiosk-app-demo/` in
  [WilhelmOS](https://github.com/algonents/wilhelmos), and the composition
  contract in its `docs/DESIGN.md` §7).

A customer kiosk application is a crate shaped exactly like this one, with
their application where `DemoApp` is.

## Running (desktop)

```
cargo run
```

Fullscreen window on the primary monitor: a triangle with an ImGui control
panel and an FPS overlay. Escape exits (via the framework's
`request_exit`); on WilhelmOS the app is launched and supervised by the
cage kiosk session. On a Wayland/X11 desktop it runs under your normal
session.

## Releasing

1. All dependencies must be published crates.io versions — a tagged
   release must not contain git dependencies in its `Cargo.lock`.
2. `cargo build` to refresh `Cargo.lock`, commit it.
3. Tag `vX.Y.Z` and push the tag.
4. In WilhelmOS: bump the recipe `SRCREV`/`PV` to the tag and regenerate
   the crates include (`bitbake -c update_crates kiosk-app-demo`).

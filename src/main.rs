//! WilhelmOS reference kiosk application.
//!
//! Runs fullscreen on the primary monitor — launched by the cage
//! compositor as the WilhelmOS kiosk session.
//!
//! Since v0.2.0 this repo is deliberately a **packaging shell**: the
//! application code below mirrors the `hello_kiosk` example of the
//! [`wilhelmos_kiosk`](https://github.com/algonents/wilhelmos_kiosk)
//! framework (the canonical "how to write a kiosk app"), and what this
//! repo demonstrates is the *packaging contract* an integrator copies —
//! a standalone binary crate with a committed lockfile and release tags,
//! consumed by a Yocto cargo recipe that provides `virtual/kiosk-app`.

use wilhelm_renderer::graphics2d::shapes::Triangle;
use wilhelmos_kiosk::{
    Color, Context, Event, FpsOverlay, Key, Kiosk, KioskApp, KioskError, ShapeId, ShapeKind,
    ShapeRenderable, ShapeStyle, Ui,
};

#[derive(Default)]
struct DemoApp {
    triangle: Option<ShapeId>,
    pos: (f32, f32),
    scale: f32,
    size: (f32, f32),
    fps: FpsOverlay,
}

impl KioskApp for DemoApp {
    fn init(&mut self, ctx: &mut Context) -> Result<(), KioskError> {
        let (w, h) = ctx.size();
        self.size = (w as f32, h as f32);
        self.pos = (self.size.0 / 2.0, self.size.1 / 2.0);
        self.scale = 1.0;

        // Everything is sized relative to the display, so the app works at
        // any resolution.
        let half = self.size.1 * 0.15;
        let triangle = ShapeRenderable::from_shape(
            ShapeKind::Triangle(Triangle::new([
                (-half, half * 0.5),
                (half, half * 0.5),
                (0.0, -half),
            ])),
            ShapeStyle::fill(Color::from_rgb(0.2, 0.6, 0.9)),
        );
        self.triangle = Some(ctx.add_shape(triangle));
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, _dt: f32) {
        if let Some(id) = self.triangle {
            if let Some(shape) = ctx.shape_mut(id) {
                shape.set_position(self.pos.0, self.pos.1);
                shape.set_scale(self.scale);
            }
        }
    }

    fn ui(&mut self, ui: &Ui<'_>, ctx: &mut Context) {
        ui.window("Shape Controls", 0, |im| {
            im.text("Position");
            im.slider_float("X", &mut self.pos.0, 0.0, self.size.0);
            im.slider_float("Y", &mut self.pos.1, 0.0, self.size.1);
            im.separator();
            im.text("Transform");
            im.slider_float("Scale", &mut self.scale, 0.1, 3.0);
        });
        self.fps.ui(ui, ctx);
    }

    fn on_event(&mut self, event: &Event, ctx: &mut Context) {
        if let Event::Key {
            key: Key::ESCAPE,
            action,
            ..
        } = event
        {
            if action.is_press() {
                ctx.request_exit();
            }
        }
    }
}

fn main() -> Result<(), KioskError> {
    Kiosk::new("WilhelmOS Kiosk Demo")
        .background(Color::from_rgb(0.1, 0.1, 0.15))
        .run(DemoApp::default())
}

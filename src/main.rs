//! WilhelmOS reference kiosk application — **the canonical
//! [`wilhelmos_kiosk`](https://github.com/algonents/wilhelmos_kiosk)
//! example** (the framework repo deliberately ships none of its own).
//!
//! Runs fullscreen on the primary monitor — launched by the cage
//! compositor as the WilhelmOS kiosk session. The code below shows the
//! framework in practice: state in plain struct fields (no
//! `Rc<RefCell<..>>`), a composed component (`FpsOverlay`), scoped ImGui
//! via `Ui`. The repo around it demonstrates the *packaging contract* an
//! integrator copies — a standalone binary crate with a committed
//! lockfile and release tags, consumed by a Yocto cargo recipe that
//! provides `virtual/kiosk-app`.
//!
//! Deliberately no key-to-exit binding: a kiosk app's lifecycle belongs
//! to the supervisor, and a bound key would be an operator-seat kill
//! switch. To exit during desktop testing, Ctrl+C — SIGINT takes the
//! framework's clean shutdown path (wilhelmos_kiosk DESIGN.md §3, §9).

use wilhelm_renderer::graphics2d::shapes::Triangle;
use wilhelmos_kiosk::{
    Color, Context, FpsOverlay, Kiosk, KioskApp, KioskError, ShapeId, ShapeKind, ShapeRenderable,
    ShapeStyle, Ui,
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
}

fn main() -> Result<(), KioskError> {
    Kiosk::new("WilhelmOS Kiosk Demo")
        .background(Color::from_rgb(0.1, 0.1, 0.15))
        .run(DemoApp::default())
}

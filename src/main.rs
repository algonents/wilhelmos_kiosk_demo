//! WilhelmOS reference kiosk application.
//!
//! Runs fullscreen on the primary monitor — launched by the cage
//! compositor as the WilhelmOS kiosk session. All positions are derived
//! from the actual window dimensions instead of being hardcoded, so the
//! app works at any display resolution.

use std::cell::RefCell;
use std::rc::Rc;

use wilhelm_renderer::core::{App, Color, Window};
use wilhelm_renderer::graphics2d::shapes::{ShapeKind, ShapeRenderable, ShapeStyle, Triangle};
use wilhelm_renderer_imgui::ImGui;

fn main() {
    let window = Window::new_fullscreen("Wilhelm Renderer Kiosk", Color::from_rgb(0.1, 0.1, 0.15));
    let (width, height) = (window.width() as f32, window.height() as f32);
    let (center_x, center_y) = (width / 2.0, height / 2.0);
    let mut app = App::new(window);

    // Triangle centered on the screen, sized relative to the display
    let half = height * 0.15;
    let mut triangle = ShapeRenderable::from_shape(
        ShapeKind::Triangle(Triangle::new([
            (-half, half * 0.5),
            (half, half * 0.5),
            (0.0, -half),
        ])),
        ShapeStyle::fill(Color::from_rgb(0.2, 0.6, 0.9)),
    );
    triangle.set_position(center_x, center_y);
    app.add_shape(triangle);

    let imgui = ImGui::new(app.window.glfw_window_ptr(), true);

    let pos_x = Rc::new(RefCell::new(center_x));
    let pos_y = Rc::new(RefCell::new(center_y));
    let scale = Rc::new(RefCell::new(1.0f32));

    let pos_x_update = Rc::clone(&pos_x);
    let pos_y_update = Rc::clone(&pos_y);
    let scale_update = Rc::clone(&scale);

    app.on_pre_render(move |shapes, _renderer| {
        if let Some(shape) = shapes.first_mut() {
            shape.set_position(*pos_x_update.borrow(), *pos_y_update.borrow());
            shape.set_scale(*scale_update.borrow());
        }
    });

    app.on_render(move |_renderer, _camera| {
        imgui.new_frame();

        imgui.begin("Shape Controls", None, 0);
        imgui.text("Position");
        imgui.slider_float("X", &mut pos_x.borrow_mut(), 0.0, width);
        imgui.slider_float("Y", &mut pos_y.borrow_mut(), 0.0, height);
        imgui.separator();
        imgui.text("Transform");
        imgui.slider_float("Scale", &mut scale.borrow_mut(), 0.1, 3.0);
        imgui.end();

        imgui.render();
    });

    app.run();
}

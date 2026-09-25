use godot::{init::is_editor_hint, prelude::*};

#[derive(GodotClass)]
#[class(init, tool, base = Node2D)]
pub(crate) struct CameraLimitRect {
    base: Base<Node2D>,

    #[export(range = (0.0, 1000.0, 2.0, or_greater, suffix = "px"))]
    #[init(val = 100.0)]
    #[var(get, set)]
    width: f32,

    #[export(range = (0.0, 1000.0, 2.0, or_greater, suffix = "px"))]
    #[init(val = 100.0)]
    #[var(get, set)]
    height: f32,
}

#[godot_api]
impl INode2D for CameraLimitRect {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        if let Some(view_port) = self.base().get_viewport()
            && let Some(mut camera) = view_port.get_camera_2d()
        {
            use godot::builtin::Side;

            let pos = self.base().get_global_position().to_vector2i();
            camera.set_limit(Side::LEFT, pos.x);
            camera.set_limit(Side::TOP, pos.y);
            camera.set_limit(Side::RIGHT, pos.x + self.width as i32);
            camera.set_limit(Side::BOTTOM, pos.y + self.height as i32);
        } else {
            godot_print!("没有发现摄像头");
        }
    }
    fn draw(&mut self) {
        if is_editor_hint() {
            let r = Rect2::new(Vector2::ZERO, Vector2::new(self.width, self.height));
            self.base_mut()
                .draw_rect_ex(r, Color::BLUE)
                .filled(false)
                .width(1.0)
                .done();
        }
    }
}

#[godot_api]
impl CameraLimitRect {
    #[func]
    fn get_width(&self) -> f32 {
        self.width
    }
    #[func]
    fn set_width(&mut self, v: f32) {
        self.width = v;
        self.base_mut().queue_redraw();
    }
    #[func]
    fn get_height(&self) -> f32 {
        self.height
    }
    #[func]
    fn set_height(&mut self, v: f32) {
        self.height = v;
        self.base_mut().queue_redraw();
    }
}

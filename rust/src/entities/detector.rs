use godot::{
    classes::{Area2D, CircleShape2D, CollisionShape2D, IArea2D, object::ConnectFlags},
    init::is_editor_hint,
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, tool, base = Area2D)]
pub(crate) struct Detector {
    base: Base<Area2D>,
    #[export]
    #[init(val = 10.0)]
    #[var(get, set)]
    size: f32,
}

#[godot_api]
impl IArea2D for Detector {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.base()
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_body_enter);
    }

    fn draw(&mut self) {
        if is_editor_hint() {
            let radius = self.size;
            self.base_mut()
                .draw_circle_ex(Vector2::ZERO, radius, Color::RED)
                .filled(false)
                .width(1.0)
                .done();
            return;
        }
    }
}

#[godot_api]
impl Detector {
    #[signal]
    pub fn found(body: Gd<Node2D>);

    #[signal]
    pub fn miss();

    #[signal]
    pub fn detected(body: Gd<Node2D>);

    #[signal]
    pub fn dismiss();

    fn on_body_enter(&mut self, body: Gd<Node2D>) {
        self.signals().found().emit(&body);

        self.signals().detected().emit(&body);

        let mut this = self.to_gd().clone();
        self.base_mut().connect_flags(
            "body_exited",
            &Callable::from_fn("on_detector_miss", move |_| {
                this.bind_mut().signals().dismiss().emit();
                this.bind_mut().signals().miss().emit();
            }),
            ConnectFlags::ONE_SHOT,
        );
    }

    #[func]
    fn set_size(&mut self, v: f32) {
        self.size = v;
        self.base_mut().queue_redraw();
        self.draw_area();
    }

    #[func]
    fn get_size(&self) -> f32 {
        self.size
    }

    fn draw_area(&mut self) {
        if let Some(circle) = self
            .base()
            .try_get_node_as::<CollisionShape2D>("CollisionShape2D")
            && let Some(circle) = circle.get_shape()
            && let Ok(mut circle) = circle.try_cast::<CircleShape2D>()
        {
            circle.set_radius(self.size);
        }
    }
}

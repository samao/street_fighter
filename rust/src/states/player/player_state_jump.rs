use crate::states::base_state::IBaseState;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateJump {
    base: Base<Node>,

    #[export]
    #[init(val = 1.0)]
    duration: f32,

    #[init(val = 0.0)]
    time: f32,

    #[export]
    #[init(val = 200.0)]
    height: f32,
}

impl IBaseState for PlayerStateJump {}

#[godot_api]
impl PlayerStateJump {
    #[func]
    fn enter(&mut self) {
        godot_print!("进入jump");
        self.time = 0.0;
        self.play_anim();
        self.set_velocity(Vector2::new(0.0, self.height));
    }

    #[func]
    fn exit(&mut self) {
        godot_print!("退出jump");
    }
    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.time += delta as f32;
        if self.time >= self.duration {
            godot_print!("时间到了，{}/{}", self.time, self.duration);
            return "idle".to_variant();
        } else {
            self.apply_gravity(delta);
        }
        Variant::nil()
    }
}

impl PlayerStateJump {
    fn apply_gravity(&self, delta: f64) {
        let mut velocity = self.get_velocity();
        if self.time < self.duration {
            velocity.y += self.get_gravity().y * delta as f32;
            self.set_velocity(velocity);
        }
    }
}

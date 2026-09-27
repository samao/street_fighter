use crate::states::base_state::IBaseState;
use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateJump {
    base: Base<Node>,

    #[export]
    #[init(val = 200.0)]
    height: f32,

    #[export]
    #[init(val = 25.0)]
    air_speed: f32,

    #[init(val = 0.0)]
    duration: f32,

    #[init(val = Vector2::ZERO)]
    jump_positon: Vector2,
}

impl IBaseState for PlayerStateJump {}

#[godot_api]
impl PlayerStateJump {
    #[func]
    fn enter(&mut self) {
        godot_print!("进入jump");
        self.set_collision_disabled(true);
        self.play_anim();
        self.jump_positon = self.get_global_position();
        //1/2 g * t2 = h
        self.duration = (self.height * 2.0 / self.get_gravity().y).abs().sqrt();
        let vel_y = self.get_gravity().y * self.duration;
        self.duration *= 2.0;
        self.set_velocity(Vector2::new(0.0, -vel_y));
        self.set_shadow_fixed(true);
    }

    #[func]
    fn exit(&mut self) {
        let mut pos = self.get_global_position();
        pos.y = self.jump_positon.y;
        self.set_global_position(pos);
        self.set_collision_disabled(false);
        self.set_shadow_fixed(false);
        self.set_attack_active(false);
        self.set_velocity(Vector2::ZERO);
    }

    #[func]
    fn handle_input(&mut self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("kick") {
            self.play_anim_by_name("jump_kick".to_string());
            self.set_attack_active(true);
        }
        Variant::nil()
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.duration -= delta;
        if self.duration <= 0.0 {
            return "land".to_variant();
        }

        let dir = Input::singleton().get_axis("left", "right");
        let mut velocity = self.get_velocity();
        velocity.x = dir * self.air_speed;
        velocity.y += self.get_gravity().y * delta;
        self.set_velocity(velocity);

        Variant::nil()
    }
}

use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

use crate::character::states::player::IPlayerBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct PlayerJump {
    base: Base<Node>,

    #[init(val = 0.0)]
    duration: f32,

    #[export]
    #[init(val = 30.0)]
    air_speed: f32,

    #[export]
    #[init(val = 50.0)]
    jump_height: f32,

    #[init(val = Vector2::ZERO)]
    jump_pos: Vector2,
}

#[godot_api]
impl PlayerJump {
    #[func]
    fn enter(&mut self) {
        self.jump_pos = self.get_agent_position();
        self.play_anim("jump");
        let gravity = self.get_gravity().y;
        self.duration = (self.jump_height * 2.0 / gravity).abs().sqrt();
        let vt = gravity * self.duration;
        self.duration *= 2.0;
        self.set_agent_velocity(Vector2::new(0.0, -vt));
        self.set_agent_collider_disabled(true);
    }

    #[func]
    fn exit(&mut self) {
        self.set_agent_velocity(Vector2::ZERO);
        let mut pos = self.get_agent_position();
        pos.y = self.jump_pos.y;
        self.set_agent_position(pos);
        self.set_agent_collider_disabled(false);
    }

    #[func]
    fn input_handle(&mut self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("kick") {
            self.play_anim("jump_kick");
        }
        Variant::nil()
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.duration -= delta;
        if self.duration <= 0.0 {
            return "PlayerLand".to_variant();
        }
        let dir = Input::singleton().get_axis("left", "right");
        let mut velocity = self.get_agent_velocity();
        velocity.x = dir * self.air_speed;
        velocity.y += self.get_gravity().y * delta;
        self.set_agent_velocity(velocity);
        Variant::nil()
    }
}

impl IPlayerBaseState for PlayerJump {}

use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

use crate::character::states::player::IPlayerBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct PlayerJumpKick {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f32,
    #[export]
    #[init(val = 30.0)]
    air_speed: f32,
}

#[godot_api]
impl PlayerJumpKick {
    #[func]
    fn enter(&mut self) {
        self.play_anim("jump_kick");
        self.duration = self.get_anim_length("jump_kick");
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn input_handle(&self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("kick") {
            return "PlayerJumpKick".to_variant();
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

impl IPlayerBaseState for PlayerJumpKick {}

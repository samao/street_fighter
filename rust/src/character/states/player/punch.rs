use godot::{classes::InputEvent, prelude::*};

use crate::character::states::player::IPlayerBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct PlayerPunch {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f32,
}

#[godot_api]
impl PlayerPunch {
    #[func]
    fn enter(&mut self) {
        self.play_anim("punch");
        self.set_agent_velocity(Vector2::ZERO);
        self.duration = self.get_anim_length("punch");
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn input_handle(&self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("jump") {
            return "PlayerJump".to_variant();
        }
        Variant::nil()
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.duration -= delta as f32;
        if self.duration <= 0.0 {
            return "PlayerIdle".to_variant();
        }
        Variant::nil()
    }
}

impl IPlayerBaseState for PlayerPunch {}

use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

use crate::character::states::player::IPlayerBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct PlayerIdle {
    base: Base<Node>,
}

#[godot_api]
impl PlayerIdle {
    #[func]
    fn enter(&mut self) {
        self.play_anim("idle");
        self.set_agent_velocity(Vector2::ZERO);
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn input_handle(&self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("jump") {
            return "PlayerTakeOff".to_variant();
        }
        if event.is_action_pressed("kick") {
            return "PlayerKick".to_variant();
        }
        if event.is_action_pressed("punch") {
            return "PlayerPunch".to_variant();
        }
        Variant::nil()
    }

    #[func]
    fn update(&self, _delta: f64) -> Variant {
        if Input::singleton().get_vector("left", "right", "up", "down") != Vector2::ZERO {
            return "PlayerWalk".to_variant();
        }
        Variant::nil()
    }
}

impl IPlayerBaseState for PlayerIdle {}

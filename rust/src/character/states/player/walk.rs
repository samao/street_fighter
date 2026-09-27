use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

use crate::character::states::player::IPlayerBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct PlayerWalk {
    base: Base<Node>,

    #[export]
    #[init(val = 50.0)]
    speed: f32,
}

#[godot_api]
impl PlayerWalk {
    #[func]
    fn enter(&mut self) {
        self.play_anim("walk");
    }

    #[func]
    fn exit(&mut self) {}

    #[func]
    fn input_handle(&self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("jump") {
            return "PlayerJump".to_variant();
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
    fn update(&mut self, _delta: f64) -> Variant {
        let dir = Input::singleton().get_vector("left", "right", "up", "down");
        if dir == Vector2::ZERO {
            return "PlayerIdle".to_variant();
        }
        self.set_agent_velocity(dir * self.speed);
        Variant::nil()
    }
}

impl IPlayerBaseState for PlayerWalk {}

use godot::prelude::*;

use crate::character::states::player::IPlayerBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct PlayerTakeOff {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f32,
}

#[godot_api]
impl PlayerTakeOff {
    #[func]
    fn enter(&mut self) {
        self.play_anim("take_off");
        self.set_agent_velocity(Vector2::ZERO);
        self.duration = self.get_anim_length("take_off");
    }

    #[func]
    fn exit(&mut self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.duration -= delta as f32;
        if self.duration <= 0.0 {
            return "PlayerJump".to_variant();
        }
        Variant::nil()
    }
}

impl IPlayerBaseState for PlayerTakeOff {}

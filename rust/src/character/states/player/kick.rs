use godot::prelude::*;

use crate::character::states::player::IPlayerBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct PlayerKick {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f32,
}

#[godot_api]
impl PlayerKick {
    #[func]
    fn enter(&mut self) {
        self.play_anim("kick");
        self.set_agent_velocity(Vector2::ZERO);
        self.duration = self.get_anim_length("kick");
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.duration -= delta as f32;
        if self.duration <= 0.0 {
            return "PlayerIdle".to_variant();
        }
        Variant::nil()
    }
}

impl IPlayerBaseState for PlayerKick {}

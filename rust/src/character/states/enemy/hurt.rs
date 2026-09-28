use godot::prelude::*;

use crate::character::states::enemy::IEnemyBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct EnemyHurt {
    base: Base<Node>,
    #[init(val = 0.0)]
    time: f32,

    #[export(range = (0.0, 1.0, 0.1))]
    #[init(val = 0.5)]
    knock_strenth: f32,
}

#[godot_api]
impl EnemyHurt {
    #[func]
    fn enter(&mut self) {
        self.time = self.get_anim_length("hurt");
        self.play_anim("hurt");
        let dir = -self.agent_to_player_direction();
        self.set_agent_velocity(dir * self.knock_strenth * 30.0);
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.time -= delta;
        if self.time <= 0.0 {
            if self.get_agent_current_hp() <= 0.0 {
                return "EnemyDeath".to_variant();
            }
            if self.get_nearest_player().is_some() {
                return "EnemyIdle".to_variant();
            }
        }
        Variant::nil()
    }
}

impl IEnemyBaseState for EnemyHurt {}

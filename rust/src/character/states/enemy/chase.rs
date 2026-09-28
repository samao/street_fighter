use godot::prelude::*;

use crate::character::states::enemy::IEnemyBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct EnemyChase {
    base: Base<Node>,

    #[export]
    #[init(val = 0.2)]
    delay: f32,

    #[init(val = 0.0)]
    time: f32,

    #[export]
    #[init(val = 40.0)]
    speed: f32,

    #[export]
    #[init(val = 20.0)]
    min_distance: f32,
}

#[godot_api]
impl EnemyChase {
    #[func]
    fn enter(&mut self) {
        self.time = self.delay;
        self.play_anim("walk");
    }

    #[func]
    fn exit(&mut self) {
        self.set_agent_velocity(Vector2::ZERO);
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        if self.get_nearest_player().is_none() {
            return "EnemyIdle".to_variant();
        }

        if self.get_player_distance_squared() < self.min_distance.powi(2) {
            return "EnemyPunch".to_variant();
        }

        let delta = delta as f32;
        self.time -= delta;
        if self.time <= 0.0 {
            self.time = self.delay;
            let chase_dir = self.agent_to_player_direction();
            self.set_agent_velocity(chase_dir * self.speed);
        }

        Variant::nil()
    }
}

impl IEnemyBaseState for EnemyChase {}

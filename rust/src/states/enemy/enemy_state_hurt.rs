use godot::prelude::*;

use crate::states::base_state::IEnemyBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct EnemyStateHurt {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f32,

    #[export(range = (0.0, 1.0, 0.1))]
    #[init(val = 0.5)]
    strength: f32,
}

impl IEnemyBaseState for EnemyStateHurt {}

#[godot_api]
impl EnemyStateHurt {
    #[func]
    fn enter(&mut self) {
        self.play_anim();
        self.duration = self.get_anim_length_by_name("hurt".to_string());
        self.set_damage_receiver_enabled(false);
        let dir = self.get_attack_dir();
        self.set_velocity(dir * Vector2::splat(10.0 * self.strength));
    }

    #[func]
    fn exit(&mut self) {
        self.set_damage_receiver_enabled(true);
        self.set_velocity(Vector2::ZERO);
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.duration -= delta as f32;

        if self.duration <= 0.0 {
            return "idle".to_variant();
        }
        Variant::nil()
    }
}

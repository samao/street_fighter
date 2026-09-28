use godot::prelude::*;

use crate::character::states::enemy::IEnemyBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct EnemyPunch {
    base: Base<Node>,
    #[export]
    #[init(val = 0.05)]
    delay: f32,

    #[init(val = 0.0)]
    time: f32,

    #[init(val = 0.0)]
    attack_duration: f32,
}

#[godot_api]
impl EnemyPunch {
    #[func]
    fn enter(&mut self) {
        self.time = self.delay;
        self.play_anim("punch");
        self.set_agent_velocity(Vector2::ZERO);
        self.face_to_target();
        self.attack_duration = self.get_anim_length("punch");
        self.time = self.delay;
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.attack_duration -= delta;
        if self.attack_duration <= 0.0 {
            self.time -= delta;
            if self.time <= 0.0 {
                if self.get_nearest_player().is_some() {
                    return "EnemyChase".to_variant();
                } else {
                    return "EnemyIdle".to_variant();
                }
            }
        }
        Variant::nil()
    }
}

impl IEnemyBaseState for EnemyPunch {}

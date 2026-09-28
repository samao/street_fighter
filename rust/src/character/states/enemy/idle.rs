use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

use crate::character::states::enemy::IEnemyBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct EnemyIdle {
    base: Base<Node>,
    #[export]
    #[init(val = 0.5)]
    delay: f32,

    #[init(val = 0.0)]
    time: f32,
}

#[godot_api]
impl EnemyIdle {
    #[func]
    fn enter(&mut self) {
        self.time = self.delay;
        self.play_anim("idle");
        self.set_agent_velocity(Vector2::ZERO);
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.time -= delta;
        if self.time <= 0.0 {
            self.time = self.delay;
            if self.get_nearest_player().is_some() {
                return "EnemyChase".to_variant();
            }
        }
        Variant::nil()
    }
}

impl IEnemyBaseState for EnemyIdle {}

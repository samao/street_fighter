use godot::prelude::*;

use crate::character::states::enemy::IEnemyBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct EnemyDeath {
    base: Base<Node>,
    #[init(val = 0.0)]
    time: f32,
}

#[godot_api]
impl EnemyDeath {
    #[func]
    fn enter(&mut self) {
        self.play_anim("death");
        self.time = self.get_anim_length("death");
        self.set_agent_velocity(Vector2::ZERO);
    }

    #[func]
    fn exit(&self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.time -= delta;
        if self.time <= 0.0 {
            //死球了
            self.release_character();
        }
        Variant::nil()
    }
}

impl IEnemyBaseState for EnemyDeath {}

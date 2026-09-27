use godot::prelude::*;

use crate::states::base_state::IEnemyBaseState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct EnemyStateIdle {
    base: Base<Node>,
}

impl IEnemyBaseState for EnemyStateIdle {}

#[godot_api]
impl EnemyStateIdle {
    #[func]
    fn enter(&mut self) {
        self.play_anim();
        self.set_velocity(Vector2::ZERO);
    }

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        if self.has_target() {
            return "chase".to_variant();
        }
        Variant::nil()
    }
}

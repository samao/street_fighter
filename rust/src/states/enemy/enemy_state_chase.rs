use crate::states::base_state::IEnemyBaseState;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct EnemyStateChase {
    base: Base<Node>,
}

impl IEnemyBaseState for EnemyStateChase {}

#[godot_api]
impl EnemyStateChase {
    #[func]
    fn enter(&mut self) {
        self.play_anim_by_name("walk".to_owned());
    }

    #[func]
    fn exit(&mut self) {
        self.set_velocity(Vector2::ZERO);
    }

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        if self.can_attack() {
            return "punch".to_variant();
        } else if self.has_target() {
            self.move_towards_target();
        } else if !self.has_target() {
            return "idle".to_variant();
        }
        Variant::nil()
    }
}

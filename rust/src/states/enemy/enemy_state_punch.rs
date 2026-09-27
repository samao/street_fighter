use crate::states::base_state::IEnemyBaseState;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct EnemyStatePunch {
    base: Base<Node>,

    #[init(val = 0.0)]
    duration: f32,
}

impl IEnemyBaseState for EnemyStatePunch {}

#[godot_api]
impl EnemyStatePunch {
    #[func]
    fn enter(&mut self) {
        self.play_anim_by_name("punch".to_owned());
        self.duration = self.get_anim_length_by_name("punch".to_owned());
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.duration -= delta as f32;

        if self.duration <= 0.0 {
            if self.can_attack() {
                self.duration = self.get_anim_length_by_name("punch".to_owned());
                return Variant::nil();
            } else {
                return "idle".to_variant();
            }
        }

        Variant::nil()
    }
}

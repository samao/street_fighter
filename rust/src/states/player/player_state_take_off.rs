use crate::states::base_state::IBaseState;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateTakeOff {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f64,
}

impl IBaseState for PlayerStateTakeOff {}

#[godot_api]
impl PlayerStateTakeOff {
    #[func]
    fn enter(&mut self) {
        godot_print!("进入take_off");
        self.play_anim();
        self.duration = self.get_current_animation_length();
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.duration -= delta;

        if self.duration <= 0.0 {
            return "jump".to_variant();
        }

        Variant::nil()
    }
}

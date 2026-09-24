use crate::states::base_state::IBaseState;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateIdle {
    base: Base<Node>,
}

impl IBaseState for PlayerStateIdle {}

#[godot_api]
impl PlayerStateIdle {
    #[func]
    fn enter(&mut self) {
        godot_print!("进入idle");
        self.play_anim();
    }

    #[func]
    fn exit(&mut self) {
        godot_print!("退出 idle");
    }

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        Variant::nil()
    }
}

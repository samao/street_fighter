use crate::states::base_state::IBaseState;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateRun {
    base: Base<Node>,
}

impl IBaseState for PlayerStateRun {}

#[godot_api]
impl PlayerStateRun {
    #[func]
    fn enter(&mut self) {
        godot_print!("进入idel");
        self.play_anim();
    }

    #[func]
    fn exit(&mut self) {
        godot_print!("退出idle");
    }

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        Variant::nil()
    }
}

use crate::states::base_state::IBaseState;
use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateRun {
    base: Base<Node>,
    #[export]
    #[init(val = 50.0)]
    speed: f32,
    #[init(val = false)]
    is_option_pressed: bool,
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
    fn handle_input(&mut self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("jump") {
            return "jump".to_variant();
        } else if event.is_action_pressed("punch") {
            if self.is_option_pressed {
                return "heavy_punch".to_variant();
            }
            return "punch".to_variant();
        } else if event.is_action_pressed("kick") {
            return "kick".to_variant();
        }

        Variant::nil()
    }

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        let dir = Input::singleton().get_vector("left", "right", "up", "down");
        if dir == Vector2::ZERO {
            return "idle".to_variant();
        }
        self.set_velocity(dir * self.speed);
        self.is_option_pressed = Input::singleton().is_action_pressed("option");
        Variant::nil()
    }
}

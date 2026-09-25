use crate::states::base_state::IBaseState;
use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateIdle {
    base: Base<Node>,
    #[init(val = false)]
    is_option_pressed: bool,
}

impl IBaseState for PlayerStateIdle {}

#[godot_api]
impl PlayerStateIdle {
    #[func]
    fn enter(&mut self) {
        godot_print!("进入idle");
        self.play_anim();
        self.set_velocity(Vector2::ZERO);
    }

    #[func]
    fn exit(&mut self) {
        godot_print!("退出 idle");
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
        if Input::singleton().get_vector("left", "right", "up", "down") != Vector2::ZERO {
            return "run".to_variant();
        }
        self.is_option_pressed = Input::singleton().is_action_pressed("option");
        Variant::nil()
    }
}

use crate::states::base_state::IBaseState;
use godot::{
    classes::{Input, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStatePunch {
    base: Base<Node>,
    #[init(val = false)]
    is_option_pressed: bool,
    #[init(val = false)]
    combo: bool,
    #[export]
    #[init(val = 0.1)]
    combo_window: f64,
    #[init(val = 0.0)]
    duration: f64,
}

impl IBaseState for PlayerStatePunch {}

#[godot_api]
impl PlayerStatePunch {
    fn is_in_window(&self) -> bool {
        self.duration > 0.0 && self.duration < self.combo_window
    }
    #[func]
    fn enter(&mut self) {
        godot_print!("进入punch");
        self.play_anim();
        self.set_velocity(Vector2::ZERO);
        self.duration = self.get_current_animation_length();
    }

    #[func]
    fn exit(&mut self) {
        godot_print!("退出 puhch");
    }

    #[func]
    fn handle_input(&mut self, event: Gd<InputEvent>) -> Variant {
        if event.is_action_pressed("jump") {
            return "jump".to_variant();
        } else if event.is_action_pressed("punch") && self.is_in_window() {
            self.combo = true;
        }
        Variant::nil()
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.is_option_pressed = Input::singleton().is_action_pressed("option");
        self.duration -= delta;
        if self.duration <= 0.0 {
            if self.combo && self.is_option_pressed {
                return "heavy_punch".to_variant();
            } else if self.combo {
                self.duration = self.get_current_animation_length();
                self.combo = false;
                return Variant::nil();
            }
            return "idle".to_variant();
        }
        Variant::nil()
    }
}

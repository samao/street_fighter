use crate::states::base_state::IBaseState;
use godot::{classes::InputEvent, prelude::*};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct PlayerStateKick {
    base: Base<Node>,
    #[init(val = false)]
    combo: bool,
    #[export]
    #[init(val = 0.1)]
    combo_window: f64,
    #[init(val = 0.0)]
    duration: f64,
}

impl IBaseState for PlayerStateKick {}

#[godot_api]
impl PlayerStateKick {
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
        } else if event.is_action_pressed("kick") && self.is_in_window() {
            self.combo = true;
        }
        Variant::nil()
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        self.duration -= delta;
        if self.duration <= 0.0 {
            if self.combo {
                self.duration = self.get_current_animation_length();
                self.combo = false;
                return Variant::nil();
            }
            return "idle".to_variant();
        }
        Variant::nil()
    }
}

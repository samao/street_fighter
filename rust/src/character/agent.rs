use godot::{classes::AnimationPlayer, init::is_editor_hint, prelude::*};

use crate::{
    character::{
        Character,
        state_machine::{CharacterStateMachine, PlayerStates},
    },
    entities::{damage_emitter::DamageEmitter, damage_receiver::DamageReceiver},
};

/// 玩家代理
#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct Agent {
    base: Base<Node>,

    #[export]
    #[init(val = PlayerStates::default())]
    init_state: PlayerStates,

    #[export]
    character: OnEditor<Gd<Character>>,

    #[export]
    animation: OnEditor<Gd<AnimationPlayer>>,

    #[export]
    state_machine: OnEditor<Gd<CharacterStateMachine>>,

    #[export]
    damage_receiver: OnEditor<Gd<DamageReceiver>>,
    #[export]
    damage_emitter: OnEditor<Gd<DamageEmitter>>,
}

#[godot_api]
impl INode for Agent {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.state_machine
            .signals()
            .change_player_state()
            .emit(self.init_state);
    }
}

#[godot_api]
impl Agent {
    #[func]
    pub fn set_character_velocity(&mut self, velocity: Vector2) {
        self.character.set_velocity(velocity);
    }

    #[func]
    pub fn play_anim(&mut self, anim_name: String) {
        if self.animation.has_animation(&anim_name) {
            self.animation.play_ex().name(&anim_name).done();
        } else {
            godot_print!("不存在动画: {anim_name}");
        }
    }

    #[func]
    pub fn get_anim_length(&self, anim_name: String) -> f32 {
        if let Some(animation) = self.animation.get_animation(&anim_name) {
            return animation.get_length();
        }
        0.0
    }
}

use godot::{
    classes::{AnimationPlayer, Marker2D},
    init::is_editor_hint,
    prelude::*,
};

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
    pub fn set_collider_disabled(&mut self, v: bool) {
        self.character.bind_mut().set_collider_disabled(v);
    }

    #[func]
    pub fn set_damage_emitter_monitoring(&mut self, v: bool) {
        self.character.bind_mut().set_damage_emitter_monitoring(v);
    }

    #[func]
    pub fn set_damage_receiver_monitorable(&mut self, v: bool) {
        self.character.bind_mut().set_damage_receiver_monitorable(v);
    }

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

    pub(crate) fn get_gravity(&self) -> Vector2 {
        self.character.get_gravity()
    }

    pub(crate) fn get_character_velocity(&self) -> Vector2 {
        self.character.get_velocity()
    }

    pub(crate) fn get_character_position(&self) -> Vector2 {
        self.character.get_global_position()
    }

    #[func]
    pub(crate) fn set_character_position(&mut self, v: Vector2) {
        self.character.set_global_position(v);
    }
}

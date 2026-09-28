use godot::{
    classes::{
        AnimationPlayer, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Sprite2D, Texture2D,
    },
    prelude::*,
};

use crate::entities::{damage_emitter::DamageEmitter, damage_receiver::DamageReceiver};

pub(self) mod agent;
pub(self) mod ai_agent;
pub(self) mod state_machine;
pub(self) mod states;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub(crate) struct Character {
    base: Base<CharacterBody2D>,

    ///形象
    #[init(node = "%Body")]
    body: OnReady<Gd<Sprite2D>>,

    //动画播放器
    #[init(node = "%AnimationPlayer")]
    ainimation: OnReady<Gd<AnimationPlayer>>,

    //攻击触发
    #[init(node = "%DamageEmitter")]
    damage_emitter: OnReady<Gd<DamageEmitter>>,

    //伤害接收
    #[init(node = "%DamageReceiver")]
    damage_receiver: OnReady<Gd<DamageReceiver>>,

    #[init(node = "%CollisionShape2D")]
    collider: OnReady<Gd<CollisionShape2D>>,
}

#[godot_api]
impl ICharacterBody2D for Character {
    fn process(&mut self, _delta: f64) {
        let velocity = self.base().get_velocity();
        self.change(velocity);
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

impl Character {
    fn change(&mut self, new_dir: Vector2) {
        if new_dir.x < 0.0 {
            self.body.set_scale(Vector2::new(-1.0, 1.0));
            self.damage_emitter.set_scale(Vector2::new(-1.0, 1.0));
        } else if new_dir.x > 0.0 {
            self.body.set_scale(Vector2::new(1.0, 1.0));
            self.damage_emitter.set_scale(Vector2::new(1.0, 1.0));
        }
    }

    pub fn set_collider_disabled(&mut self, v: bool) {
        self.collider.set_disabled(v);
    }

    pub fn set_damage_emitter_monitoring(&mut self, v: bool) {
        self.damage_emitter.set_monitoring(v);
    }

    pub fn set_damage_receiver_monitorable(&mut self, v: bool) {
        self.damage_receiver.set_monitorable(v);
    }
}

#[godot_api]
impl Character {
    #[func]
    pub fn set_texture(&mut self, texture: Gd<Texture2D>) {
        // godot_print!("设置皮肤: {:?}", texture);
        self.body.set_texture(&texture);
    }
}

use godot::{
    classes::{AnimationPlayer, CharacterBody2D, ICharacterBody2D, Sprite2D},
    prelude::*,
};

use crate::entities::{damage_emitter::DamageEmitter, damage_receiver::DamageReceiver};

pub(self) mod agent;
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
        } else if new_dir.x > 0.0 {
            self.body.set_scale(Vector2::new(1.0, 1.0));
        }
    }
}

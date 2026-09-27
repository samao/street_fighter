use godot::{
    classes::{AnimationPlayer, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Sprite2D},
    init::is_editor_hint,
    prelude::*,
};

use crate::{
    entities::{damage_emitter::DamageEmitter, damage_receiver::DamageReceiver},
    resources::health::Health,
};

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub(crate) struct Actor {
    base: Base<CharacterBody2D>,
    #[export]
    health: OnEditor<Gd<Health>>,
    #[init(node = "%AnimationPlayer")]
    anim: OnReady<Gd<AnimationPlayer>>,
    #[init(node = "%Sprite2D")]
    body: OnReady<Gd<Sprite2D>>,
    #[init(node = "%Shadow")]
    shadow: OnReady<Gd<Sprite2D>>,
    #[init(node = "%CollisionShape2D")]
    collider: OnReady<Gd<CollisionShape2D>>,
    #[init(node = "%DamageEmitter")]
    damage_emitter: OnReady<Gd<DamageEmitter>>,
    #[init(node = "%DamageReceiver")]
    damage_receiver: OnReady<Gd<DamageReceiver>>,
    #[init(val = None)]
    shadow_fix_pos: Option<Vector2>,
}

#[godot_api]
impl ICharacterBody2D for Actor {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.health
            .signals()
            .die()
            .connect_other(&*self, Self::on_actor_die);
        self.damage_receiver
            .signals()
            .damage_received()
            .connect_other(&*self, Self::on_damage_received);
    }

    fn process(&mut self, _delta: f64) {
        if let Some(shadow_pos) = self.shadow_fix_pos.as_ref() {
            let mut pos = self.base().get_global_position();
            pos.y = shadow_pos.y;
            self.shadow.set_global_position(pos);
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[allow(dead_code)]
impl Actor {
    pub fn get_actor_health(&self) -> Gd<Health> {
        self.health.clone()
    }

    fn on_damage_received(&mut self, from: Gd<DamageEmitter>) {
        let damage = from.bind().get_amount();
        self.health
            .call_deferred("take_damage", &[damage.to_variant()]);
        self.signals().was_hit().emit();
    }

    fn on_actor_die(&mut self) {
        godot_print!("死球了");
        self.signals().was_die().emit();
    }

    pub(crate) fn set_velocity(&mut self, v: Vector2) {
        // godot_print!("设置actor速度: {}", v);
        self.base_mut().set_velocity(v);

        if v.x > 0.0 {
            self.body.set_scale(Vector2::new(1.0, 1.0));
            self.damage_emitter.set_scale(Vector2::new(1.0, 1.0));
        } else if v.x < 0.0 {
            self.body.set_scale(Vector2::new(-1.0, 1.0));
            self.damage_emitter.set_scale(Vector2::new(-1.0, 1.0));
        }
    }

    pub(crate) fn get_velocity(&self) -> Vector2 {
        self.base().get_velocity()
    }

    pub(crate) fn get_gravity(&self) -> Vector2 {
        self.base().get_gravity()
    }

    pub(crate) fn set_collision_disable(&mut self, v: bool) {
        self.collider.set_disabled(v);
    }

    pub(crate) fn get_animation_length(&self) -> f64 {
        self.anim.get_current_animation_length()
    }

    pub(crate) fn set_shadow_fixed(&mut self, fixed: bool) {
        if fixed {
            let pos = self.base().get_global_position();
            self.shadow_fix_pos = Some(pos);
        } else {
            self.shadow_fix_pos = None;
            self.shadow.set_position(Vector2::ZERO);
        }
    }

    pub(crate) fn set_damage_reciver_enable(&mut self, v: bool) {
        self.damage_receiver.set_monitorable(v);
    }

    pub(crate) fn set_attack_active(&mut self, v: bool) {
        self.damage_emitter.set_monitoring(v);
    }
}

#[godot_api]
impl Actor {
    #[signal]
    pub fn was_hit();
    #[signal]
    pub fn was_die();

    #[func]
    pub fn play_anim(&mut self, anim_name: String) {
        if self.anim.has_animation(anim_name.as_str()) {
            godot_print!("播放动画: {anim_name}");
            self.anim.play_ex().name(&anim_name).done();
        } else {
            godot_print!("no animation name {} exist!", anim_name);
        }
    }

    #[func]
    pub fn get_anim_length_by_name(&self, anim_name: String) -> f32 {
        if let Some(anim) = self.anim.get_animation(anim_name.as_str()) {
            return anim.get_length();
        }
        0.0
    }
}

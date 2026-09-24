use godot::{
    classes::{AnimationPlayer, CharacterBody2D, ICharacterBody2D},
    init::is_editor_hint,
    prelude::*,
};

use crate::resources::health::Health;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub(crate) struct Actor {
    base: Base<CharacterBody2D>,
    #[export]
    health: OnEditor<Gd<Health>>,
    #[init(node = "%AnimationPlayer")]
    anim: OnReady<Gd<AnimationPlayer>>,
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
    fn on_actor_die(&mut self) {
        // godot_print!("死球了");
    }

    pub(crate) fn set_velocity(&mut self, v: Vector2) {
        godot_print!("设置actor速度: {}", v);
        self.base_mut().set_velocity(v);
    }

    pub(crate) fn get_velocity(&self) -> Vector2 {
        self.base().get_velocity()
    }

    pub(crate) fn get_gravity(&self) -> Vector2 {
        self.base().get_gravity()
    }
}

#[godot_api]
impl Actor {
    #[func]
    pub fn play_anim(&mut self, anim_name: String) {
        if self.anim.has_animation(anim_name.as_str()) {
            godot_print!("播放动画: {anim_name}");
            self.anim.play_ex().name(&anim_name).done();
        } else {
            godot_print!("no animation name {} exist!", anim_name);
        }
    }
}

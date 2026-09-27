use godot::{classes::node::ProcessMode, obj::WithBaseField, prelude::*};

use crate::entities::{actor::Actor, enemy_ai::EnemyAI};

#[allow(dead_code)]
pub(crate) trait IBaseState: WithBaseField<Base = Node> {
    fn actor(&self) -> Option<Gd<Actor>> {
        let node = self.to_gd().upcast();
        if let Some(owner) = node.get_owner()
            && let Ok(owner) = owner.try_cast::<Actor>()
        {
            Some(owner)
        } else {
            None
        }
    }

    fn set_attack_active(&self, v: bool) {
        if let Some(ref mut actor) = self.actor() {
            actor.bind_mut().set_attack_active(v);
        }
    }

    fn set_enable(&mut self, v: bool) {
        self.base_mut().set_process_mode(if v {
            ProcessMode::INHERIT
        } else {
            ProcessMode::DISABLED
        });
    }

    fn play_anim(&self) {
        let node_name = self.to_gd().get_name();
        self.play_anim_by_name(node_name.to_string());
    }

    fn play_anim_by_name(&self, anim_name: String) {
        if let Some(ref mut actor) = self.actor() {
            actor.bind_mut().play_anim(anim_name);
        }
    }

    fn set_velocity(&self, velocity: Vector2) {
        if let Some(ref mut actor) = self.actor() {
            actor.bind_mut().set_velocity(velocity);
        }
    }

    fn get_velocity(&self) -> Vector2 {
        if let Some(actor) = self.actor().as_ref() {
            return actor.bind().get_velocity();
        }
        Vector2::ZERO
    }

    fn get_current_animation_length(&self) -> f64 {
        if let Some(actor) = self.actor().as_ref() {
            return actor.bind().get_animation_length();
        }
        0.0
    }

    fn get_global_position(&self) -> Vector2 {
        if let Some(actor) = self.actor().as_ref() {
            return actor.get_global_position();
        }
        Vector2::ZERO
    }

    fn set_global_position(&self, v: Vector2) {
        if let Some(actor) = self.actor().as_mut() {
            actor.set_global_position(v);
        }
    }

    fn set_collision_disabled(&self, v: bool) {
        if let Some(actor) = self.actor().as_mut() {
            actor.bind_mut().set_collision_disable(v);
        }
    }

    fn set_shadow_fixed(&self, v: bool) {
        if let Some(actor) = self.actor().as_mut() {
            actor.bind_mut().set_shadow_fixed(v);
        }
    }

    fn get_gravity(&self) -> Vector2 {
        if let Some(actor) = self.actor().as_ref() {
            return actor.bind().get_gravity();
        }
        Vector2::ZERO
    }
}

pub(crate) trait IEnemyBaseState: WithBaseField<Base = Node> {
    fn agent(&self) -> Option<Gd<EnemyAI>> {
        let node = self.to_gd().upcast();
        if let Some(owner) = node.get_owner()
            && let Some(agent) = owner.try_get_node_as::<EnemyAI>("EnemyAI")
        {
            Some(agent)
        } else {
            None
        }
    }

    fn set_velocity(&self, velocity: Vector2) {
        if let Some(ref mut actor) = self.agent() {
            actor.call_deferred("set_velocity", &[velocity.to_variant()]);
        }
    }

    fn play_anim(&self) {
        let node_name = self.to_gd().get_name();
        self.play_anim_by_name(node_name.to_string());
    }

    fn play_anim_by_name(&self, anim_name: String) {
        if let Some(ref mut actor) = self.agent() {
            actor.call_deferred("play_anim", &[anim_name.to_variant()]);
            // actor.bind_mut().play_anim(anim_name);
        }
    }

    fn get_anim_length_by_name(&self, anim_name: String) -> f32 {
        if let Some(ref actor) = self.agent() {
            return actor.bind().get_animation_length(anim_name);
        }
        0.0
    }

    fn move_towards_target(&self) {
        if let Some(ref mut actor) = self.agent() {
            // actor.bind_mut().move_towards_target();
            actor.call_deferred("move_towards_target", &[]);
        }
    }

    fn can_attack(&self) -> bool {
        if let Some(ref actor) = self.agent() {
            return actor.bind().can_attack();
        }
        false
    }

    fn set_damage_receiver_enabled(&self, v: bool) {
        if let Some(ref mut actor) = self.agent() {
            actor.call_deferred("set_damage_receiver_enabled", &[v.to_variant()]);
        }
    }

    fn get_attack_dir(&self) -> Vector2 {
        if let Some(ref actor) = self.agent() {
            return actor.bind().get_attack_dir();
        }
        Vector2::ZERO
    }

    fn has_target(&self) -> bool {
        if let Some(ref actor) = self.agent() {
            return actor.bind().has_target();
        }
        false
    }

    fn is_arrived_target(&self) -> bool {
        if let Some(ref mut actor) = self.agent() {
            return actor.bind().is_arrived_target();
        }
        false
    }
}

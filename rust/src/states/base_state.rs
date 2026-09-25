use godot::{classes::node::ProcessMode, obj::WithBaseField, prelude::*};

use crate::entities::actor::Actor;

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

    fn set_enable(&mut self, v: bool) {
        self.base_mut().set_process_mode(if v {
            ProcessMode::INHERIT
        } else {
            ProcessMode::DISABLED
        });
    }

    fn play_anim(&self) {
        if let Some(ref mut actor) = self.actor() {
            let node_name = self.to_gd().get_name();
            actor.bind_mut().play_anim(node_name.to_string());
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

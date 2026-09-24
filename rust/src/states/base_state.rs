use godot::{obj::WithBaseField, prelude::*};

use crate::entities::actor::Actor;

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

    fn get_gravity(&self) -> Vector2 {
        if let Some(actor) = self.actor().as_ref() {
            return actor.bind().get_gravity();
        }
        Vector2::ZERO
    }
}

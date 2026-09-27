use godot::{obj::WithBaseField, prelude::*};

use crate::character::agent::Agent;

pub(super) mod idle;
pub(super) mod kick;
pub(super) mod punch;
pub(super) mod walk;

trait IPlayerBaseState: WithBaseField<Base = Node> {
    ///设置控体速度
    fn set_agent_velocity(&mut self, velocity: Vector2) {
        if let Some(agent) = self.get_agent().as_mut() {
            agent.call_deferred("set_character_velocity", &[velocity.to_variant()]);
        }
    }

    ///获取动画长度
    fn get_anim_length(&self, anim_name: &str) -> f32 {
        if let Some(agent) = self.get_agent().as_mut() {
            return agent.bind().get_anim_length(anim_name.to_owned());
        }
        0.0
    }

    ///播放动画
    fn play_anim(&mut self, anim_name: &str) {
        if let Some(agent) = self.get_agent().as_mut() {
            agent.call_deferred("play_anim", &[anim_name.to_variant()]);
        }
    }

    fn get_agent(&self) -> Option<Gd<Agent>> {
        if let Some(owner) = self.base().get_owner() {
            return owner.try_get_node_as::<Agent>("Agent");
        }
        None
    }
}

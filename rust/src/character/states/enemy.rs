use godot::{obj::WithBaseField, prelude::*};

use crate::character::{Character, ai_agent::AIAgent};

pub(super) mod chase;
pub(super) mod death;
pub(super) mod hurt;
pub(super) mod idle;
pub(super) mod punch;

trait IEnemyBaseState: WithBaseField<Base = Node> {
    ///设置控体速度
    fn set_agent_velocity(&mut self, velocity: Vector2) {
        if let Some(agent) = self.get_agent().as_mut() {
            agent.call_deferred("set_character_velocity", &[velocity.to_variant()]);
        }
    }
    ///设置控体速度
    fn release_character(&mut self) {
        if let Some(agent) = self.get_agent().as_mut() {
            agent.call_deferred("release_character", &[]);
        }
    }

    ///面向目标
    fn face_to_target(&mut self) {
        if let Some(agent) = self.get_agent().as_mut() {
            agent.call_deferred("face_to_target", &[]);
        }
    }

    //获取目标
    fn get_nearest_player(&self) -> Option<Gd<Character>> {
        if let Some(agent) = self.get_agent() {
            return agent.bind().get_target();
        }
        None
    }

    fn agent_to_player_direction(&self) -> Vector2 {
        if let Some(pos) = self.get_nearest_marker() {
            return self.get_agent_position().direction_to(pos);
        }
        Vector2::ZERO
    }

    fn get_player_distance_squared(&self) -> f32 {
        if let Some(pos) = self.get_nearest_marker() {
            return self.get_agent_position().distance_squared_to(pos);
        }
        f32::MAX
    }

    //获取玩家最近的攻击位置
    fn get_nearest_marker(&self) -> Option<Vector2> {
        if let Some(agent) = self.get_agent() {
            let pos = agent.bind().get_character_position();
            return agent.bind().get_nearest_attack_marker(pos);
        }
        None
    }

    //获取物理体的速度
    fn get_agent_velocity(&self) -> Vector2 {
        if let Some(agent) = self.get_agent() {
            return agent.bind().get_character_velocity();
        }
        Vector2::ZERO
    }

    //获取当前血量
    fn get_agent_current_hp(&self) -> f32 {
        if let Some(agent) = self.get_agent() {
            return agent.bind().get_current_hp();
        }
        0.0
    }

    ///设置控体位置
    fn set_agent_position(&mut self, velocity: Vector2) {
        if let Some(agent) = self.get_agent().as_mut() {
            agent.call_deferred("set_character_position", &[velocity.to_variant()]);
        }
    }

    //设置碰撞体
    fn set_agent_collider_disabled(&mut self, v: bool) {
        if let Some(agent) = self.get_agent().as_mut() {
            agent.call_deferred("set_collider_disabled", &[v.to_variant()]);
        }
    }

    //获取物理体的位置
    fn get_agent_position(&self) -> Vector2 {
        if let Some(agent) = self.get_agent() {
            return agent.bind().get_character_position();
        }
        Vector2::ZERO
    }

    //获取物理体的重力加速度
    fn get_gravity(&self) -> Vector2 {
        if let Some(agent) = self.get_agent() {
            return agent.bind().get_gravity();
        }
        Vector2::ZERO
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

    fn get_agent(&self) -> Option<Gd<AIAgent>> {
        if let Some(owner) = self.base().get_owner() {
            return owner.try_get_node_as::<AIAgent>("AIAgent");
        }
        None
    }
}

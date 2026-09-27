use godot::{init::is_editor_hint, prelude::*};

use crate::{
    entities::{actor::Actor, detector::Detector},
    states::state_machine::StateMachine,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct EnemyAI {
    base: Base<Node>,

    #[export]
    detector: OnEditor<Gd<Detector>>,
    #[export]
    state_machine: OnEditor<Gd<StateMachine>>,

    #[init(val = None)]
    target: Option<Gd<Node2D>>,

    #[export]
    slaver: OnEditor<Gd<Actor>>,

    #[init(val = 25.0)]
    walk_speed: f32,

    #[export]
    #[init(val = 24.0)]
    chase_min_distance: f32,
}

#[godot_api]
impl INode for EnemyAI {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.detector
            .signals()
            .found()
            .connect_other(&*self, Self::on_player_found);
        self.detector
            .signals()
            .miss()
            .connect_other(&*self, Self::on_player_miss);

        self.state_machine.bind_mut().transition("idle".to_owned());
        self.slaver
            .signals()
            .was_hit()
            .connect_other(&*self, Self::on_slaver_hit);
    }
}

#[godot_api]
impl EnemyAI {
    fn on_slaver_hit(&mut self) {
        self.state_machine
            .call_deferred("transition", &["hurt".to_variant()]);
    }

    fn on_player_found(&mut self, body: Gd<Node2D>) {
        self.target = Some(body);
        // self.state_machine.bind_mut().transition("chase".to_owned());
    }

    fn on_player_miss(&mut self) {
        self.target = None;
        // self.state_machine.bind_mut().transition("idle".to_owned());
    }

    #[func]
    pub(crate) fn set_velocity(&mut self, v: Vector2) {
        self.slaver.bind_mut().set_velocity(v);
    }

    #[func]
    pub(crate) fn get_attack_dir(&self) -> Vector2 {
        if let Some(target) = self.target.as_ref() {
            let slaver_pos = self.slaver.get_global_position();
            return target.get_global_position().direction_to(slaver_pos);
        }
        Vector2::ZERO
    }

    #[func]
    pub(crate) fn set_damage_receiver_enabled(&mut self, v: bool) {
        self.slaver.bind_mut().set_damage_reciver_enable(v);
    }

    #[func]
    pub(crate) fn play_anim(&mut self, anim_name: String) {
        self.slaver.bind_mut().play_anim(anim_name);
    }

    #[func]
    pub(crate) fn move_towards_target(&mut self) {
        if let Some(ref target) = self.target
            && !self.is_arrived_target()
        {
            let slaver_pos = self.slaver.get_global_position();
            let speed = self.walk_speed;
            let dir = slaver_pos.direction_to(target.get_global_position());
            self.slaver.bind_mut().set_velocity(dir * speed);
        }
    }

    #[func]
    pub(crate) fn can_attack(&self) -> bool {
        if self.target.is_some() && self.is_arrived_target() {
            return true;
        }
        false
    }

    #[func]
    pub(crate) fn get_animation_length(&self, anim_name: String) -> f32 {
        self.slaver.bind().get_anim_length_by_name(anim_name)
    }

    pub(crate) fn is_arrived_target(&self) -> bool {
        if let Some(ref target) = self.target {
            let slaver_pos = self.slaver.get_global_position();
            let dis_sqruared = target.get_global_position().distance_squared_to(slaver_pos);
            return dis_sqruared < self.chase_min_distance.powi(2);
        }

        return true;
    }

    pub(crate) fn has_target(&self) -> bool {
        self.target.is_some()
    }
}

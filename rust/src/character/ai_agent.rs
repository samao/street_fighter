use godot::{
    classes::{AnimationPlayer, Marker2D, Sprite2D},
    init::is_editor_hint,
    prelude::*,
};

use crate::{
    character::{
        Character,
        state_machine::{CharacterStateMachine, PlayerStates},
    },
    entities::{
        damage_emitter::DamageEmitter, damage_receiver::DamageReceiver, detector::Detector,
    },
    resources::enemy_setting::EnemySetting,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct AIAgent {
    base: Base<Node>,

    #[export]
    config: OnEditor<Gd<EnemySetting>>,

    #[export]
    #[init(val = PlayerStates::default())]
    init_state: PlayerStates,

    #[export]
    character: OnEditor<Gd<Character>>,

    #[export]
    animation: OnEditor<Gd<AnimationPlayer>>,

    #[export]
    state_machine: OnEditor<Gd<CharacterStateMachine>>,

    #[export]
    damage_receiver: OnEditor<Gd<DamageReceiver>>,

    #[export]
    damage_emitter: OnEditor<Gd<DamageEmitter>>,

    #[export]
    detector: OnEditor<Gd<Detector>>,

    #[init(val = None)]
    target: Option<Gd<Character>>,

    #[init(val = 1.0)]
    hp: f32,

    #[export]
    body: OnEditor<Gd<Sprite2D>>,
}

impl AIAgent {}

#[godot_api]
impl INode for AIAgent {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }
        self.update_texture();
        self.base_mut().call_deferred("init_after_next_frame", &[]);
    }
}

impl AIAgent {
    fn on_player_detected(&mut self, body: Gd<Node2D>) {
        if let Ok(player) = body.try_cast::<Character>() {
            self.target = Some(player);
        }
    }

    fn on_player_out_range(&mut self) {
        self.target = None;
    }

    fn update_texture(&mut self) {
        let texture = self.config.bind().get_texture();
        self.character
            .call_deferred("set_texture", &[texture.to_variant()]);
    }

    fn on_damage_received(&mut self, _from: Gd<DamageEmitter>) {
        self.hp -= 1.0;
        self.state_machine
            .signals()
            .change_player_state()
            .emit(PlayerStates::EnemyHurt);
    }
}

#[godot_api]
impl AIAgent {
    #[func]
    fn init_after_next_frame(&mut self) {
        self.detector
            .signals()
            .detected()
            .connect_other(&*self, Self::on_player_detected);
        self.detector
            .signals()
            .dismiss()
            .connect_other(&*self, Self::on_player_out_range);

        self.damage_receiver
            .signals()
            .damage_received()
            .connect_other(&*self, Self::on_damage_received);

        self.state_machine
            .signals()
            .change_player_state()
            .emit(self.init_state);

        self.hp = self.config.bind().get_max_hp();
    }

    pub(crate) fn get_current_hp(&self) -> f32 {
        self.hp
    }

    #[func]
    pub fn set_collider_disabled(&mut self, v: bool) {
        self.character.bind_mut().set_collider_disabled(v);
    }

    #[func]
    pub fn set_damage_emitter_monitoring(&mut self, v: bool) {
        self.character.bind_mut().set_damage_emitter_monitoring(v);
    }

    #[func]
    pub fn set_damage_receiver_monitorable(&mut self, v: bool) {
        self.character.bind_mut().set_damage_receiver_monitorable(v);
    }

    #[func]
    pub fn set_character_velocity(&mut self, velocity: Vector2) {
        self.character.set_velocity(velocity);
    }

    #[func]
    pub fn play_anim(&mut self, anim_name: String) {
        if self.animation.has_animation(&anim_name) {
            self.animation.play_ex().name(&anim_name).done();
        } else {
            godot_print!("不存在动画: {anim_name}");
        }
    }

    #[func]
    pub fn get_anim_length(&self, anim_name: String) -> f32 {
        if let Some(animation) = self.animation.get_animation(&anim_name) {
            return animation.get_length();
        }
        0.0
    }

    pub(crate) fn get_gravity(&self) -> Vector2 {
        self.character.get_gravity()
    }

    pub(crate) fn get_character_velocity(&self) -> Vector2 {
        self.character.get_velocity()
    }

    #[func]
    pub(crate) fn release_character(&mut self) {
        // godot_print!("销毁主题");
        self.character.queue_free();
    }

    #[func]
    pub(crate) fn face_to_target(&mut self) {
        if let Some(target) = self.target.as_ref() {
            let dir = self
                .character
                .get_global_position()
                .direction_to(target.get_global_position());
            if dir.x > 0.0 {
                self.body.set_scale(Vector2::new(1.0, 1.0));
            } else if dir.x < 0.0 {
                self.body.set_scale(Vector2::new(-1.0, 1.0));
            }
        }
    }

    pub(crate) fn get_character_position(&self) -> Vector2 {
        self.character.get_global_position()
    }

    pub(crate) fn get_target(&self) -> Option<Gd<Character>> {
        self.target.clone()
    }

    pub fn get_nearest_attack_marker(&self, pos: Vector2) -> Option<Vector2> {
        let enemy_markers = self
            .base()
            .get_tree()
            .get_nodes_in_group("attack_group_marker");
        let poses = enemy_markers
            .iter_shared()
            .map(|m| m.cast::<Marker2D>().get_global_position())
            .min_by(|a, b| {
                a.distance_squared_to(pos)
                    .total_cmp(&b.distance_squared_to(pos))
            });

        poses
    }

    #[func]
    pub(crate) fn set_character_position(&mut self, v: Vector2) {
        self.character.set_global_position(v);
    }
}

use godot::{
    classes::{AnimationPlayer, IStaticBody2D, StaticBody2D},
    global::remap,
    init::is_editor_hint,
    prelude::*,
};

use crate::entities::{damage_emitter::DamageEmitter, damage_receiver::DamageReceiver};

#[derive(GodotClass)]
#[class(init, base = StaticBody2D)]
pub(crate) struct Barrel {
    base: Base<StaticBody2D>,

    #[init(node = "%DamageReceiver")]
    damage_receiver: OnReady<Gd<DamageReceiver>>,

    #[init(node = "%AnimationPlayer")]
    anim: OnReady<Gd<AnimationPlayer>>,

    #[export]
    #[init(val = 1.0)]
    hp: f32,

    #[export]
    #[init(val = 10.0)]
    speed: f32,

    #[export]
    #[init(val = 10.0)]
    height: f32,

    #[init(val = 0.0)]
    duration: f32,

    #[init(val = 0.0)]
    time: f32,

    #[init(val = 1.0)]
    dir: f32,
    #[init(val = Vector2::ZERO)]
    base_pos: Vector2,
}

#[godot_api]
impl IStaticBody2D for Barrel {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.damage_receiver
            .signals()
            .damage_received()
            .connect_other(&*self, Self::on_damage_received);
        self.base_mut().set_process(false);
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as f32;
        self.time += delta;

        let percent = 1.0 - remap(self.time as f64, 0.0, self.duration as f64, -1.0, 1.0).abs();

        let v_pos = -self.height * percent as f32;
        let h_pos = self.speed * self.time * self.dir;
        // godot_print!("当前：（{}，{}）= {}", h_pos, v_pos, percent);
        let next_pos = Vector2::new(h_pos, v_pos) + self.base_pos;
        self.base_mut().set_global_position(next_pos);

        if self.time > self.duration {
            self.base_mut().set_process(false);
            self.base_mut().queue_free();
        }
    }
}

impl Barrel {
    fn on_damage_received(&mut self, emitter: Gd<DamageEmitter>) {
        // godot_print!("我被打了 {}", self.base().get_name());
        let amount = emitter.bind().get_amount();

        self.hp -= amount;

        if self.hp <= 0.0 {
            self.anim.play_ex().name("destroy").done();
            let hit_dir = emitter
                .get_global_position()
                .direction_to(self.damage_receiver.get_global_position())
                .x;

            self.base_pos = self.base().get_global_position();
            let duration = self.anim.get_current_animation_length() as f32;
            self.duration = 2.0 * duration;
            self.base_mut().set_process(true);
            self.damage_receiver.set_monitorable(false);
            self.base_mut().set_scale(Vector2::new(hit_dir, 1.0));
            self.dir = hit_dir;
        }
    }

    // fn on_animation_finished(&mut self, anim_name: StringName) {
    //     if anim_name.to_string() == "destroy" {
    //         // self.base_mut().queue_free();
    //     }
    // }
}

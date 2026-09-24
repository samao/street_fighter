use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub(crate) struct Health {
    base: Base<Resource>,

    #[export]
    #[init(val = 3.0)]
    #[var(no_set, get)]
    hp: f32,

    #[export]
    #[init(val = 3.0)]
    max_hp: f32,
}

#[godot_api]
impl Health {
    #[signal]
    pub(crate) fn die();
    #[signal]
    pub(crate) fn health_change(hp: f32, max_hp: f32);

    #[func]
    fn get_hp(&self) -> f32 {
        self.hp
    }

    #[func]
    pub(crate) fn heal(&mut self, amount: f32) {
        self.hp += amount;
        let cur_hp = self.hp;
        let max_hp = self.max_hp;
        self.signals().health_change().emit(cur_hp, max_hp)
    }

    #[func]
    pub(crate) fn take_damage(&mut self, damage: f32) {
        let cur_hp = self.hp - damage;
        let max_hp = self.max_hp;
        let cur_hp = cur_hp.clamp(0.0, max_hp);
        self.hp = cur_hp;
        self.signals().health_change().emit(cur_hp, max_hp);
        godot_print!("当前血量: {}/{}", cur_hp, max_hp);
        if self.hp <= 0.0 {
            self.signals().die().emit();
        }
    }
}

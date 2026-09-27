use godot::{classes::Area2D, prelude::*};

use crate::entities::damage_emitter::DamageEmitter;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct DamageReceiver {
    base: Base<Area2D>,
}

#[godot_api]
impl DamageReceiver {
    #[signal]
    pub fn damage_received(from: Gd<DamageEmitter>);
}

use godot::{classes::Area2D, prelude::*};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct DamageReceiver {
    base: Base<Area2D>,
}

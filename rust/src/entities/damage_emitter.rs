use godot::{
    classes::{Area2D, IArea2D},
    init::is_editor_hint,
    prelude::*,
};

use crate::entities::damage_receiver::DamageReceiver;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct DamageEmitter {
    base: Base<Area2D>,
    #[export]
    #[init(val = 1.0)]
    #[var(pub)]
    amount: f32,
}

#[godot_api]
impl IArea2D for DamageEmitter {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.base()
            .signals()
            .area_entered()
            .connect_other(&*self, Self::on_damage_receiver_enter);
    }
}

impl DamageEmitter {
    fn on_damage_receiver_enter(&mut self, area: Gd<Area2D>) {
        if let Ok(mut damage_receiver) = area.try_cast::<DamageReceiver>() {
            //延迟调用，突破借用限制
            damage_receiver.call_deferred(
                "emit_signal",
                &["damage_received".to_variant(), self.to_gd().to_variant()],
            );
        }
    }
}

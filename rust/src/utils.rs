use godot::{classes::Engine, prelude::*};

pub(crate) fn next_frame<T>(id: InstanceId, callback: Box<dyn FnOnce(Gd<T>)>)
where
    T: GodotClass,
{
    godot::task::spawn(async move {
        if let Some(main) = Engine::singleton().get_main_loop()
            && let Ok(tree) = main.try_cast::<SceneTree>()
        {
            tree.signals().process_frame().to_future().await;
        }

        if let Ok(node) = Gd::<T>::try_from_instance_id(id) {
            callback(node);
        }
    });
}

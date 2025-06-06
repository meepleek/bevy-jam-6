use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Last,
        (
            store_initial_state::<Transform>,
            store_initial_state::<GlobalTransform>,
        ),
    );
}

#[derive(Component, Deref, DerefMut)]
pub struct Initial<T: Component + Clone>(T);

fn store_initial_state<T: Component + Clone>(
    initial_state_q: Query<(Entity, &T), Added<T>>,
    mut cmd: Commands,
) {
    for (e, val) in &initial_state_q {
        or_continue!(cmd.get_entity(e)).insert(Initial(val.clone()));
    }
}

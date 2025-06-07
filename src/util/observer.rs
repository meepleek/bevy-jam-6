use crate::prelude::*;

#[derive(Component)]
pub struct RemovableObserver(Entity);

pub fn consume_event<E: Event, B: Bundle>(mut trig: Trigger<E, B>) {
    trig.propagate(false);
}

pub fn insert_default_on_event<E: Event, B: Bundle, C: Component + Default>(
    trig: Trigger<E, B>,
    mut cmd: Commands,
) {
    or_return_quiet!(cmd.get_entity(trig.target())).insert(C::default());
}

pub fn remove_on_event<E: Event, B: Bundle, C: Component>(trig: Trigger<E, B>, mut cmd: Commands) {
    or_return_quiet!(cmd.get_entity(trig.target())).remove::<C>();
}

pub fn trigger_default_on_event<TSourceEv: Event, B: Bundle, TTargetEv: Event + Default>(
    trig: Trigger<TSourceEv, B>,
    mut cmd: Commands,
) {
    or_return_quiet!(cmd.get_entity(trig.target())).trigger(TTargetEv::default());
}

pub fn remove_observers_for_watched_entity(
    commands: &mut Commands,
    observer_q: Query<(Entity, &Observer)>,
    entity: Entity,
) {
    for (observer_e, observer) in observer_q.iter() {
        if observer.descriptor().entities().contains(&entity) {
            or_continue!(commands.get_entity(observer_e)).try_despawn();
        }
    }
}

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
    or_return_quiet!(cmd.get_entity(trig.target())).try_insert(C::default());
}

pub fn remove_on_event<E: Event, B: Bundle, C: Component>(trig: Trigger<E, B>, mut cmd: Commands) {
    or_return_quiet!(cmd.get_entity(trig.target())).try_remove::<C>();
}

pub fn remove_on_add<TAddComponent: Component, TBundleToRemove: Bundle>(
    trig: Trigger<OnAdd, TAddComponent>,
    mut cmd: Commands,
) {
    or_return_quiet!(cmd.get_entity(trig.target())).try_remove::<TBundleToRemove>();
}

pub fn trigger_default_on_event<TSourceEv: Event, B: Bundle, TTargetEv: Event + Default>(
    trig: Trigger<TSourceEv, B>,
    mut cmd: Commands,
) {
    or_return_quiet!(cmd.get_entity(trig.target())).trigger(TTargetEv::default());
}

pub fn ensure_single_on_add<C: Component>(
    trig: Trigger<OnAdd, C>,
    mut cmd: Commands,
    query: Query<Entity, With<C>>,
) {
    let target_e = trig.target();
    for e in query.iter().filter(|e| *e != target_e) {
        or_continue!(cmd.get_entity(e)).try_remove::<C>();
    }
}

pub fn remove_observers_for_watched_entity(
    commands: &mut Commands,
    observer_q: Query<(Entity, &Observer)>,
    entity: Entity,
) {
    for (observer_e, _) in observer_q
        .iter()
        .filter(|(_, observer)| observer.descriptor().entities().contains(&entity))
    {
        or_continue!(commands.get_entity(observer_e)).try_despawn();
    }
}

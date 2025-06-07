use crate::prelude::*;

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

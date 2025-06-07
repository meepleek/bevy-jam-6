use bevy::ecs::relationship::Relationship;
use bevy::ecs::system::IntoObserverSystem;

use crate::prelude::*;

pub trait EntityCommandsObserverExt {
    fn observe_with_relationship<R: Relationship, E: Event, B: Bundle, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self;
}
impl EntityCommandsObserverExt for EntityCommands<'_> {
    fn observe_with_relationship<R: Relationship, E: Event, B: Bundle, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        let e = self.id();
        let observer = Observer::new(observer).with_entity(e);
        self.commands_mut().spawn(observer).add_one_related::<R>(e);
        self
    }
}

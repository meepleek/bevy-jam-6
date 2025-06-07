use bevy::prelude::*;

pub trait RelationshipEntity {
    fn entity(&self) -> Entity;
}

pub trait RelationshipEntities {
    fn entities(&self) -> &[Entity];
}

macro_rules! relationship_1_to_1 {
    ($source:ident, $target:ident) => {
        #[derive(Component, Debug, Reflect)]
        #[reflect(Component)]
        #[relationship(relationship_target = $target)]
        pub struct $source(pub Entity);

        #[allow(dead_code)]
        impl RelationshipEntity for $source {
            fn entity(&self) -> Entity {
                self.0
            }
        }

        #[derive(Component, Debug, Reflect)]
        #[reflect(Component)]
        #[relationship_target(relationship = $source, linked_spawn)]
        pub struct $target(Entity);

        #[allow(dead_code)]
        impl RelationshipEntity for $target {
            fn entity(&self) -> Entity {
                self.0
            }
        }
    };
}

pub(crate) use relationship_1_to_1;

relationship_1_to_1!(ChildTranslation, TranslationRoot);
relationship_1_to_1!(ChildRotation, RotationRoot);

macro_rules! relationship_1_to_n {
    ($source:ident, $target:ident) => {
        #[derive(Component, Debug, Reflect)]
        #[reflect(Component)]
        #[relationship(relationship_target = $target)]
        pub struct $source(pub Entity);

        #[allow(dead_code)]
        impl RelationshipEntity for $source {
            fn entity(&self) -> Entity {
                self.0
            }
        }

        #[derive(Component, Default, Debug, Reflect)]
        #[reflect(Component)]
        #[relationship_target(relationship = $source, linked_spawn)]
        pub struct $target(Vec<Entity>);

        #[allow(dead_code)]
        impl RelationshipEntities for $target {
            fn entities(&self) -> &[Entity] {
                &self.0
            }
        }
    };
}

pub(crate) use relationship_1_to_n;

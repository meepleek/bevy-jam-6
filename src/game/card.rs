use bevy::ecs::relationship::RelatedSpawner;

use crate::prelude::*;

pub fn plugin(_app: &mut App) {}

pub enum CardEffect {
    Move(u8),
}

#[derive(Component)]
#[require(Transform)]
pub struct Card {
    effect: CardEffect,
}

#[derive(Component)]
#[relationship(relationship_target = CardVisuals)]
pub struct CardRootOf(Entity);
#[derive(Component)]
#[relationship_target(relationship = CardRootOf, linked_spawn)]
pub struct CardVisuals(Entity);

pub fn card(effect: CardEffect, position: Vec2, hover_mesh: Handle<Mesh>) -> impl Bundle {
    (
        Name::new("card_root"),
        Transform::from_translation(position.extend(0.)),
        Visibility::default(),
        Mesh2d(hover_mesh),
        Pickable {
            should_block_lower: false,
            is_hoverable: true,
        },
        Patch(|b| {
            b.observe(on_card_pointer_out);
            b.with_children(|b| {
                b.spawn((
                    Name::new("card"),
                    Card { effect },
                    Sprite::from_color(AMBER_100, Vec2::new(160., 240.)),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                    CardRootOf(b.target_entity()),
                ))
                .observe(on_card_pointer_over)
                .observe(on_card_click);
            });
        }),
    )
}

fn on_card_pointer_over(t: Trigger<Pointer<Over>>, mut card_q: Query<&mut Transform, With<Card>>) {
    let mut t = or_return!(card_q.get_mut(t.target()));
    t.translation.y = 150.;
}

fn on_card_pointer_out(
    t: Trigger<Pointer<Out>>,
    card_visuals_q: Query<&CardVisuals>,
    mut card_q: Query<&mut Transform, With<Card>>,
) {
    let card_e = or_return_quiet!(card_visuals_q.get(t.target));
    let mut t = or_return!(card_q.get_mut(card_e.0));
    t.translation = Vec3::ZERO;
}

fn on_card_click(t: Trigger<Pointer<Click>>, mut card_q: Query<&mut Transform, With<Card>>) {
    info!("card click");
}

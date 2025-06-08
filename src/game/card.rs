use bevy::color::palettes::css::BLACK;

use crate::game::card_effect::CardAction;
use crate::prelude::*;
use crate::util;

pub const CARD_BORDER_COL: Srgba = GRAY_950;
pub const CARD_BORDER_COL_FOCUS: Srgba = AMBER_400;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, draw_card_action);
}

#[derive(Component, Debug)]
#[require(Transform)]
pub struct Card {
    pub action: CardAction,
}

#[derive(Event, Default)]
#[event(traversal = &'static ChildOf, auto_propagate)]
pub struct CardPointerOut;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct CardFocused;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct CardSelected;

util::relationship::relationship_1_to_1!(CardContent, CardContentRoot);

pub fn card(
    action: CardAction,
    position: Vec3,
    rotation: Rot2,
    hover_mesh: Handle<Mesh>,
) -> impl Bundle {
    (
        Name::new("card"),
        Card { action },
        Transform::from_translation(position),
        Visibility::default(),
        Patch(move |b| {
            b.with_children(|b| {
                b.spawn((
                    Name::new("card_border"),
                    Sprite::from_color(CARD_BORDER_COL, Vec2::new(160., 240.)),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                    Transform::from_rotation(Quat::from_rotation_z(rotation.as_radians())),
                    ChildRotation(b.target_entity()),
                    Visibility::default(),
                    children![(
                        Name::new("card_content"),
                        CardContent(b.target_entity()),
                        Sprite::from_color(AMBER_100, Vec2::new(150., 230.)),
                        Transform::from_xyz(0., 0., 0.05),
                    )],
                ));

                b.spawn((
                    Name::new("card_hover_area"),
                    Transform::from_xyz(0., -120., 0.),
                    Mesh2d(hover_mesh),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                ))
                .observe(trigger_default_on_event::<Pointer<Out>, (), CardPointerOut>)
                .observe(consume_event::<Pointer<Over>, ()>)
                .observe(consume_event::<Pointer<Click>, ()>);
            });
        }),
    )
}

fn draw_card_action(card_q: Query<(Entity, &Card, &RotationRoot), Added<Card>>, mut cmd: Commands) {
    for (card_e, card, rotation_root) in &card_q {
        or_continue!(cmd.get_entity(rotation_root.entity())).with_children(|b| {
            b.spawn((
                Name::new("card_content"),
                Visibility::default(),
                Transform::from_translation(Vec3::Z * 0.06),
            ))
            .with_children(|b| {
                b.spawn((
                    Name::new("card_title"),
                    Text2d::new(card.action.title()),
                    TextColor::from(BLACK),
                    Transform::from_translation(Vec3::Y * 90.),
                ));

                if let Some(pip_change) = card.action.pip_change() {
                    b.spawn((
                        Name::new("pip_cost"),
                        Text2d::new(pip_change.to_string()),
                        TextColor::from(if pip_change > 0 { GREEN_400 } else { RED_400 }),
                        Transform::from_translation(Vec3::new(50., 90., 0.)),
                    ));
                }

                if let (Some(palette), Some(effect_tiles)) = (
                    card.action.tile_interaction_palette(),
                    card.action.effect_tiles(),
                ) {
                    b.spawn((
                        Name::new("effect_tiles"),
                        Transform::from_translation(Vec3::Y * -15.),
                        Visibility::default(),
                    ))
                    .with_children(|b| {
                        let size = 20f32;
                        // center tile
                        b.spawn((Sprite::from_color(ROSE_300, Vec2::splat(size - 3.)),));
                        for tile in effect_tiles {
                            b.spawn((
                                Sprite::from_color(palette.highlight, Vec2::splat(size - 3.)),
                                Transform::from_translation(tile.as_vec2().extend(0.) * size),
                            ));
                        }
                    });
                }
            });
        });
    }
}

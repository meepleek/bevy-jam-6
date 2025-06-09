use bevy::color::palettes::css::BLACK;

use crate::game::card_effect::CardActionTrigger;
use crate::game::card_effect::PlaySelectedTileCard;
use crate::game::card_effect::TileActionCommon;
use crate::game::card_effect::TileTarget;
use crate::game::pile::HandCard;
use crate::game::tile::TileInteraction;
use crate::prelude::*;
use crate::util;

pub const CARD_BORDER_COL: Srgba = GRAY_950;
pub const CARD_BORDER_COL_FOCUS: Srgba = AMBER_400;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(draw_card_effects)
        .add_observer(process_selected_card);
}

#[derive(Component, Debug, Clone)]
#[require(Transform)]
pub struct Card {
    pub trigger: CardActionTrigger,
}

#[derive(Event, Default)]
#[event(traversal = &'static ChildOf, auto_propagate)]
pub struct CardPointerOut;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct CardFocused;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct SelectedTileTriggerCard;

util::relationship::relationship_1_to_1!(CardContent, CardContentRoot);
util::relationship::relationship_1_to_1!(CardFace, CardFaceRoot);

pub fn card(
    action: CardActionTrigger,
    position: Vec3,
    rotation: Rot2,
    hover_mesh: Handle<Mesh>,
) -> impl Bundle {
    (
        Name::new("card"),
        Card { trigger: action },
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

fn draw_card_effects(
    trig: Trigger<OnAdd, HandCard>,
    card_q: Query<(&Card, &RotationRoot), Without<CardFaceRoot>>,
    mut cmd: Commands,
) {
    let (card, rotation_root) = or_return!(card_q.get(trig.target()));
    or_return!(cmd.get_entity(rotation_root.entity())).with_children(|b| {
        b.spawn((
            Name::new("card_content"),
            Visibility::default(),
            Transform::from_translation(Vec3::Z * 0.06),
            CardFace(trig.target()),
        ))
        .with_children(|b| {
            b.spawn((
                Name::new("card_title"),
                Text2d::new(card.trigger.title()),
                TextColor::from(BLACK),
                Transform::from_translation(Vec3::Y * 90.),
            ));

            if let Some(pip_change) = card.trigger.pip_change() {
                b.spawn((
                    Name::new("pip_cost"),
                    Text2d::new(pip_change.to_string()),
                    TextColor::from(if pip_change > 0 { GREEN_400 } else { RED_400 }),
                    Transform::from_translation(Vec3::new(50., 90., 0.)),
                ));
            }

            match &card.trigger {
                CardActionTrigger::CardSelection(action) => {
                    // todo: smt for card actions
                    //  b.spawn((
                    //             Name::new("immediate_action"),
                    //             Text2d::new("Do the thing"),
                    //             TextColor::from(BLACK),
                    //         ));
                },
                CardActionTrigger::TileSelection(action) => {
                    b.spawn((
                        Name::new("effect_tiles"),
                        Transform::from_translation(Vec3::Y * -15.),
                        Visibility::default(),
                    ))
                    .with_children(|b| {
                        let palette = action.tile_interaction_palette();
                        let size = 20f32;
                        // center tile
                        b.spawn((Sprite::from_color(ROSE_300, Vec2::splat(size - 3.)),));
                        for tile in action.tiles() {
                            b.spawn((
                                Sprite::from_color(palette.highlight, Vec2::splat(size - 3.)),
                                Transform::from_translation(tile.as_vec2().extend(0.) * size),
                            ));
                        }
                    });
                },
            }
        });
    });
}

fn process_selected_card(
    trig: Trigger<OnAdd, SelectedTileTriggerCard>,
    card_q: Query<&Card>,
    mut cmd: Commands,
    grid: Single<&Grid>,
    player: Single<Entity, With<Player>>,
) {
    let card = or_return!(card_q.get(trig.target()));
    let player_tile = or_return!(grid.entity_to_coords(*player));
    match &card.trigger {
        CardActionTrigger::TileSelection(action) => {
            let interaction_palette = action.tile_interaction_palette();
            for (tile, position) in action
                .tiles()
                .into_iter()
                .map(|tile| player_tile + tile)
                .filter_map(|tile| {
                    if matches!(action.tile_target(), TileTarget::EmptyTiles)
                        || grid.contains_die(tile)
                    {
                        grid.tile_to_world(tile).map(|pos| (tile, pos))
                    } else {
                        None
                    }
                })
            {
                let card_e = trig.target();
                cmd.spawn((
                    Transform::from_translation(position.extend(0.)),
                    Sprite::from_color(Color::NONE, Vec2::splat(60.)),
                    tween::get_relative_sprite_color_anim(interaction_palette.highlight, 150, None),
                    tween::get_absolute_scale_anim(Vec3::splat(0.5), Vec2::ONE, 180, None),
                    TileInteraction,
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                ))
                .observe(tween::tween_sprite_color_on_trigger::<Pointer<Over>, ()>(
                    interaction_palette.hover,
                ))
                .observe(tween::tween_sprite_color_on_trigger::<Pointer<Out>, ()>(
                    interaction_palette.highlight,
                ))
                .observe(
                    move |_trig: Trigger<Pointer<Click>>, mut cmd: Commands| {
                        cmd.trigger(PlaySelectedTileCard {
                            card_e,
                            selected_tile: tile,
                        });
                    },
                );
            }
        },
        CardActionTrigger::CardSelection(_) => {
            unreachable!("Card should have been played directly")
        },
    }
}

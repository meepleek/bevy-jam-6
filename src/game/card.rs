use bevy_tweening::Animator;
use bevy_tweening::RepeatStrategy;
use bevy_tweening::Sequence;
use bevy_tweening::Tracks;

use crate::prelude::tween::get_relative_scale_tween;
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
pub struct CardSelected;

#[derive(Component)]
#[relationship(relationship_target = CardVisuals)]
pub struct CardRootOf(Entity);
#[derive(Component)]
#[relationship_target(relationship = CardRootOf, linked_spawn)]
pub struct CardVisuals(Entity);

pub fn card(
    effect: CardEffect,
    position: Vec3,
    rotation: Rot2,
    hover_mesh: Handle<Mesh>,
) -> impl Bundle {
    (
        Name::new("card_root"),
        Transform::from_translation(position)
            .with_rotation(Quat::from_rotation_z(rotation.as_radians())),
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

#[cfg_attr(feature = "native_dev", hot)]
fn on_card_pointer_over(t: Trigger<Pointer<Over>>, mut cmd: Commands) {
    or_return!(cmd.get_entity(t.target())).insert(tween::get_relative_translation_anim(
        Vec3::Y * 150.,
        250,
        Some(EaseFunction::BackOut),
    ));
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_card_pointer_out(
    t: Trigger<Pointer<Out>>,
    mut cmd: Commands,
    card_visuals_q: Query<&CardVisuals>,
    card_selected_q: Query<(), With<CardSelected>>,
) {
    let card_e = or_return_quiet!(card_visuals_q.get(t.target)).0;
    if card_selected_q.contains(card_e) {
        return;
    }
    or_return!(cmd.get_entity(card_e)).insert(Animator::new(Tracks::new([
        tween::get_relative_translation_tween(Vec3::ZERO, 250, Some(EaseFunction::BackOut)),
        // reset size in case
        get_relative_scale_tween(
            Vec2::splat(1.).extend(1.),
            220,
            Some(EaseFunction::QuinticOut),
        ),
    ])));
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_card_click(
    t: Trigger<Pointer<Click>>,
    mut cmd: Commands,
    card_selected_q: Query<(), With<CardSelected>>,
) {
    info!("card click");
    let mut e_cmd = or_return!(cmd.get_entity(t.target()));
    if card_selected_q.contains(t.target()) {
        e_cmd.remove::<CardSelected>();
        e_cmd.insert(Animator::new(Sequence::new([
            get_relative_scale_tween(
                Vec2::splat(0.8).extend(1.),
                80,
                Some(EaseFunction::QuadraticOut),
            ),
            get_relative_scale_tween(
                Vec2::splat(1.).extend(1.),
                260,
                Some(EaseFunction::QuinticOut),
            ),
        ])));
    } else {
        e_cmd.insert((
            CardSelected,
            Animator::new(Sequence::new([
                get_relative_scale_tween(
                    Vec2::splat(1.15).extend(1.),
                    80,
                    Some(EaseFunction::QuadraticOut),
                ),
                get_relative_scale_tween(
                    Vec2::splat(1.).extend(1.),
                    260,
                    Some(EaseFunction::QuinticOut),
                ),
            ])),
        ));
    }
}

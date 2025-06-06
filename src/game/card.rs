use bevy_tweening::Animator;
use bevy_tweening::BoxedTweenable;
use bevy_tweening::Sequence;
use bevy_tweening::Tracks;

use crate::prelude::tween::get_relative_scale_tween;
use crate::prelude::*;

pub fn plugin(app: &mut App) {}

const CARD_BORDER_COL: Srgba = GRAY_950;
const CARD_BORDER_COL_FOCUS: Srgba = AMBER_400;

pub enum CardEffect {
    Move(u8),
}

#[derive(Component)]
#[require(Transform)]
pub struct Card {
    effect: CardEffect,
}

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct CardFocused;

#[derive(Component, Clone, Copy, PartialEq, Default)]
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
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                    CardRootOf(b.target_entity()),
                    Sprite::from_color(CARD_BORDER_COL, Vec2::new(160., 240.)),
                    children![(
                        Sprite::from_color(AMBER_100, Vec2::new(150., 230.)),
                        Transform::from_xyz(0., 0., 0.05)
                    )],
                ))
                .observe(on_card_pointer_over)
                .observe(on_card_pointer_out)
                .observe(on_card_click)
                .observe(move_focused_card)
                .observe(move_unfocused_card)
                .observe(tween::tween_sprite_color_on_trigger::<OnAdd, CardFocused>(
                    CARD_BORDER_COL_FOCUS,
                ))
                .observe(
                    tween::tween_sprite_color_on_trigger::<OnRemove, CardFocused>(CARD_BORDER_COL),
                )
                .observe(move_selected_card)
                .observe(move_deselected_card);
            });
        }),
    )
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_card_pointer_over(trig: Trigger<Pointer<Over>>, mut cmd: Commands) {
    or_return_quiet!(cmd.get_entity(trig.target())).insert(CardFocused);
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_card_pointer_out(
    trig: Trigger<Pointer<Out>>,
    card_visuals_q: Query<&CardVisuals>,
    mut cmd: Commands,
) {
    let card_e = or_return_quiet!(card_visuals_q.get(trig.target)).0;
    or_return_quiet!(cmd.get_entity(card_e)).remove::<CardFocused>();
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_card_click(
    t: Trigger<Pointer<Click>>,
    mut cmd: Commands,
    card_selected_q: Query<(), With<CardSelected>>,
) {
    let mut e_cmd = or_return!(cmd.get_entity(t.target()));
    if card_selected_q.contains(t.target()) {
        // deselect card
        e_cmd
            .try_remove::<CardSelected>()
            .try_remove::<CardFocused>();
    } else {
        // select card
        e_cmd.insert(CardSelected);
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_focused_card(
    trig: Trigger<OnAdd, CardFocused>,
    mut cmd: Commands,
    selected_q: Query<(), With<CardSelected>>,
) {
    let card_e = trig.target();
    if selected_q.contains(card_e) {
        return;
    }
    or_return_quiet!(cmd.get_entity(card_e)).insert(tween::get_relative_translation_anim(
        Vec2::Y * 150.,
        250,
        Some(EaseFunction::BackOut),
    ));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_unfocused_card(
    trig: Trigger<OnRemove, CardFocused>,
    mut cmd: Commands,
    selected_q: Query<(), With<CardSelected>>,
) {
    let card_e = trig.target();
    if selected_q.contains(card_e) {
        return;
    }
    or_return!(cmd.get_entity(card_e)).insert(Animator::new(Tracks::new([
        tween::get_relative_translation_tween(Vec2::ZERO, 250, Some(EaseFunction::BackOut)),
        // reset size in case other interactions overlap
        get_relative_scale_tween(Vec2::splat(1.), 220, Some(EaseFunction::QuinticOut)),
    ])));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_selected_card(trig: Trigger<OnAdd, CardSelected>, mut cmd: Commands) {
    or_return!(cmd.get_entity(trig.target())).insert(Animator::new(
        get_relative_scale_tween(Vec2::splat(1.15), 80, Some(EaseFunction::QuadraticOut)).then(
            get_relative_scale_tween(Vec2::splat(1.), 260, Some(EaseFunction::QuinticOut)),
        ),
    ));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_deselected_card(trig: Trigger<OnRemove, CardSelected>, mut cmd: Commands) {
    or_return!(cmd.get_entity(trig.target())).insert(Animator::new(Tracks::new([
        tween::get_relative_translation_tween(Vec2::ZERO, 250, Some(EaseFunction::BackOut)).into(),
        Box::new(Sequence::new([get_relative_scale_tween(
            Vec2::splat(0.8),
            80,
            Some(EaseFunction::QuadraticOut),
        )
        .then(get_relative_scale_tween(
            Vec2::splat(1.),
            260,
            Some(EaseFunction::QuinticOut),
        ))])) as BoxedTweenable<_>,
    ])));
}

use bevy_tweening::Animator;
use bevy_tweening::BoxedTweenable;
use bevy_tweening::Sequence;
use bevy_tweening::Tracks;

use crate::game::card_effect::CardAction;
use crate::prelude::tween::PriorityTween;
use crate::prelude::tween::get_relative_scale_tween;
use crate::prelude::tween::get_relative_translation_tween;
use crate::prelude::*;
use crate::util;

pub fn plugin(app: &mut App) {}

const CARD_BORDER_COL: Srgba = GRAY_950;
const CARD_BORDER_COL_FOCUS: Srgba = AMBER_400;

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
                    // CardRootOf(b.target_entity()),
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
                        Visibility::default(),
                    )],
                ));

                b.spawn((
                    Name::new("card_hover_area"),
                    Transform::from_xyz(0., -120., 0.),
                    Visibility::default(),
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

            b.observe(insert_default_on_event::<Pointer<Over>, (), CardFocused>)
                .observe(remove_on_event::<CardPointerOut, (), CardFocused>)
                .observe(on_card_click)
                .observe(move_focused_card)
                .observe(rotate_focused_card)
                .observe(rotate_unfocused_card)
                .observe(move_unfocused_card)
                .observe(tween::tween_related_sprite_color_on_trigger::<
                    OnAdd,
                    CardFocused,
                    RotationRoot,
                >(CARD_BORDER_COL_FOCUS))
                .observe(tween::tween_related_sprite_color_on_trigger::<
                    OnRemove,
                    CardFocused,
                    RotationRoot,
                >(CARD_BORDER_COL))
                .observe(move_selected_card)
                .observe(move_deselected_card);
        }),
    )
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_card_click(
    t: Trigger<Pointer<Click>>,
    mut cmd: Commands,
    card_selected_q: Query<(Entity), With<CardSelected>>,
) {
    if card_selected_q.contains(t.target()) {
        // deselect card
        or_return!(cmd.get_entity(t.target()))
            .try_remove::<CardSelected>()
            .try_remove::<CardFocused>();
    } else {
        // deselect other cards
        for e in &card_selected_q {
            or_continue_quiet!(cmd.get_entity(e)).try_remove::<CardSelected>();
        }

        // select card
        or_return!(cmd.get_entity(t.target())).insert(CardSelected);
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_focused_card(
    trig: Trigger<OnAdd, CardFocused>,
    mut cmd: Commands,
    card_q: Query<(&Initial<Transform>, Has<CardSelected>), Without<PriorityTween<Transform>>>,
) {
    let card_e = trig.target();
    let (initial_t, is_selected) = or_return!(card_q.get(card_e));
    if is_selected {
        return;
    }
    or_return_quiet!(cmd.get_entity(card_e)).insert(tween::get_relative_translation_anim(
        initial_t.translation.truncate().with_y(-150.),
        250,
        Some(EaseFunction::BackOut),
    ));
}

#[cfg_attr(feature = "native_dev", hot)]
fn rotate_focused_card(
    trig: Trigger<OnAdd, CardFocused>,
    mut cmd: Commands,
    card_q: Query<&RotationRoot, Without<CardSelected>>,
) {
    let rotation_root = or_return_quiet!(card_q.get(trig.target()));
    or_return_quiet!(cmd.get_entity(rotation_root.entity()))
        .insert(tween::get_relative_z_rotation_anim(0., 250, None));
}

#[cfg_attr(feature = "native_dev", hot)]
fn rotate_unfocused_card(
    trig: Trigger<OnRemove, CardFocused>,
    mut cmd: Commands,
    card_q: Query<&RotationRoot, Without<CardSelected>>,
    initial_trans_q: Query<&Initial<Transform>>,
) {
    let rotation_root = or_return_quiet!(card_q.get(trig.target()));
    let initial_t = or_return!(initial_trans_q.get(rotation_root.entity()));
    or_return_quiet!(cmd.get_entity(rotation_root.entity())).insert(
        tween::get_relative_z_rotation_anim(
            initial_t.rotation.to_euler(EulerRot::XYZ).2,
            250,
            None,
        ),
    );
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_unfocused_card(
    trig: Trigger<OnRemove, CardFocused>,
    mut cmd: Commands,
    card_q: Query<(&Initial<Transform>, Has<CardSelected>), Without<PriorityTween<Transform>>>,
) {
    let card_e = trig.target();
    let (initial_t, is_selected) = or_return_quiet!(card_q.get(card_e));
    if is_selected {
        return;
    }
    or_return!(cmd.get_entity(card_e)).insert(Animator::new(Tracks::new([
        tween::get_relative_translation_tween(
            initial_t.translation.truncate(),
            250,
            Some(EaseFunction::BackOut),
        ),
        // reset size in case other interactions overlap
        get_relative_scale_tween(Vec2::splat(1.), 220, Some(EaseFunction::QuinticOut)),
    ])));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_selected_card(trig: Trigger<OnAdd, CardSelected>, mut cmd: Commands) {
    or_return!(cmd.get_entity(trig.target())).insert(Animator::new(Tracks::new([
        Box::new(get_relative_translation_tween(
            Vec2::new(-400., 0.),
            350,
            Some(EaseFunction::BackOut),
        )) as BoxedTweenable<_>,
        get_relative_scale_tween(Vec2::splat(1.15), 80, Some(EaseFunction::QuadraticOut))
            .then(get_relative_scale_tween(
                Vec2::splat(1.),
                260,
                Some(EaseFunction::QuinticOut),
            ))
            .into(),
    ])));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_deselected_card(
    trig: Trigger<OnRemove, CardSelected>,
    mut cmd: Commands,
    initial_trans_q: Query<&Initial<Transform>>,
) {
    let initial_t = or_return!(initial_trans_q.get(trig.target()));
    or_return!(cmd.get_entity(trig.target())).insert((
        Animator::new(Tracks::new([
            tween::get_relative_translation_tween(
                initial_t.translation.truncate(),
                400,
                Some(EaseFunction::BackOut),
            )
            .into(),
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
        ])),
        PriorityTween::<Transform>::default(),
    ));
}

use bevy_tweening::Animator;
use bevy_tweening::BoxedTweenable;
use bevy_tweening::Sequence;
use bevy_tweening::Tracks;

use crate::game::card::CARD_BORDER_COL;
use crate::game::card::CARD_BORDER_COL_FOCUS;
use crate::game::card::CardPointerOut;
use crate::prelude::tween::PriorityTween;
use crate::prelude::*;

relationship_1_to_n!(DrawPileCard, DrawPile);
relationship_1_to_n!(HandCard, CardsInHand);
relationship_1_to_n!(HandCardObserver, CardsInHandObservers);
relationship_1_to_n!(DiscardCard, DiscardPile);

pub(super) fn plugin(app: &mut App) {
    app.add_observer(card_added_to_discard)
        .add_observer(card_added_to_hand);
}

fn card_added_to_hand(trig: Trigger<OnAdd, HandCard>, mut cmd: Commands) {
    debug!("card added to hand");
    or_return!(cmd.get_entity(trig.target()))
        .observe(insert_default_on_event::<Pointer<Over>, (), CardFocused>)
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
}

fn card_added_to_discard(
    trig: Trigger<OnAdd, DiscardCard>,
    mut cmd: Commands,
    observer_q: Query<(Entity, &Observer)>,
    trans_q: Query<&Transform>,
    card_rot_q: Query<&RotationRoot>,
    discard: Single<&DiscardPile>,
) {
    let anim_dur_ms = 300;
    let card_t = or_return!(trans_q.get(trig.target()));
    or_return!(cmd.get_entity(trig.target())).insert(tween::get_absolute_translation_anim(
        card_t
            .translation
            .with_z(0.1 + discard.entities().len() as f32 / 1.),
        Vec2::new(-480., 230.),
        anim_dur_ms,
        None,
    ));
    let rotation_e = or_return!(card_rot_q.get(trig.target())).entity();
    let angle_max = 25f32;
    let angle = -90. + rand::thread_rng().gen_range(-angle_max..angle_max);
    or_return!(cmd.get_entity(rotation_e)).try_insert(tween::get_relative_z_rotation_anim(
        angle.to_radians(),
        anim_dur_ms,
        None,
    ));
    remove_observers_for_watched_entity(&mut cmd, observer_q, trig.target());
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
        tween::get_relative_scale_tween(Vec2::splat(1.), 220, Some(EaseFunction::QuinticOut)),
    ])));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_selected_card(trig: Trigger<OnAdd, CardSelected>, mut cmd: Commands) {
    or_return!(cmd.get_entity(trig.target())).insert(Animator::new(Tracks::new([
        Box::new(tween::get_relative_translation_tween(
            Vec2::new(-400., 0.),
            350,
            Some(EaseFunction::BackOut),
        )) as BoxedTweenable<_>,
        tween::get_relative_scale_tween(Vec2::splat(1.15), 80, Some(EaseFunction::QuadraticOut))
            .then(tween::get_relative_scale_tween(
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
            Box::new(Sequence::new([tween::get_relative_scale_tween(
                Vec2::splat(0.8),
                80,
                Some(EaseFunction::QuadraticOut),
            )
            .then(tween::get_relative_scale_tween(
                Vec2::splat(1.),
                260,
                Some(EaseFunction::QuinticOut),
            ))])) as BoxedTweenable<_>,
        ])),
        PriorityTween::<Transform>::default(),
    ));
}

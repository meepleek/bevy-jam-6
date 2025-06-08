use bevy::time::common_conditions::repeating_after_delay;
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
// relationship_1_to_n!(HandCardObserver, CardsInHandObservers);
relationship_1_to_n!(DiscardPileCard, DiscardPile);

pub const START_HAND_SIZE: u8 = 3;

const FOCUSED_CARD_Y: f32 = -220.;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(card_added_to_draw)
        .add_observer(card_added_to_hand)
        .add_observer(card_added_to_discard)
        .add_observer(restore_empty_piles::<DrawPile>)
        .add_observer(restore_empty_piles::<CardsInHand>)
        .add_observer(restore_empty_piles::<DiscardPile>)
        .add_observer(ensure_single_on_add::<CardSelected>)
        .add_observer(ensure_single_on_add::<CardFocused>);
    app.add_systems(
        Update,
        check_hand_size.run_if(repeating_after_delay(Duration::from_millis(1500))),
    );
    app.register_type::<DrawPile>()
        .register_type::<CardsInHand>()
        .register_type::<DiscardPile>();
}

#[derive(Component)]
#[require(DrawPile, CardsInHand, DiscardPile)]
pub struct Piles;

fn pile_card_pos_rot(
    rng: &mut ThreadRng,
    y_sign: f32,
    card_pile_order: i16,
    translation_max_offset: f32,
    rotation_max_degree: f32,
) -> (Vec3, f32) {
    let angle = -90. + rng.gen_range(-rotation_max_degree..rotation_max_degree);
    let offset_range = -translation_max_offset..translation_max_offset;
    (
        Vec3::new(
            -480. + rng.gen_range(offset_range.clone()),
            -230. * y_sign + rng.gen_range(offset_range),
            card_pile_order as f32 * 0.1,
        ),
        angle.to_radians(),
    )
}

pub fn draw_pile_card_pos_rot(rng: &mut ThreadRng, card_pile_order: i16) -> (Vec3, f32) {
    pile_card_pos_rot(rng, 1., card_pile_order, 10., 8.)
}

fn discard_pile_card_pos_rot(rng: &mut ThreadRng, card_pile_order: i16) -> (Vec3, f32) {
    pile_card_pos_rot(rng, -1., card_pile_order, 20., 25.)
}

fn card_added_to_draw(
    trig: Trigger<OnAdd, DrawPileCard>,
    mut cmd: Commands,
    card_rot_q: Query<&RotationRoot>,
    draw_pile: Single<&DrawPile>,
) {
    let mut rng = thread_rng();
    let anim_dur_ms = 300;
    let (new_pos, new_angle) = draw_pile_card_pos_rot(&mut rng, draw_pile.len() as i16);

    or_return!(cmd.get_entity(trig.target())).insert(tween::get_relative_translation_3d_anim(
        new_pos,
        anim_dur_ms,
        None,
    ));
    let rotation_e = or_return!(card_rot_q.get(trig.target())).entity();
    or_return!(cmd.get_entity(rotation_e)).try_insert(tween::get_relative_z_rotation_anim(
        new_angle,
        anim_dur_ms,
        None,
    ));
}

fn card_index_mult(card_index: usize, pile_size: usize) -> f32 {
    card_index as f32 - (pile_size / 2) as f32
}

fn card_index_from_slice(entities: &[Entity], entity: Entity) -> usize {
    entities.iter().position(|e| *e == entity).unwrap_or(0)
}

fn hand_card_pos(card_index: usize, pile_size: usize) -> Vec3 {
    let pos_mult = card_index_mult(card_index, pile_size);
    Vec3::new(
        pos_mult * 150.,
        -290. - pos_mult.abs() * 25.,
        pos_mult / 10. + 1.,
    )
}

fn hand_card_pos_with_offset(card_index: usize, pile_size: usize, rng: &mut ThreadRng) -> Vec3 {
    let max_offset = 10f32;
    hand_card_pos(card_index, pile_size)
        + Vec3::new(
            rng.gen_range(-max_offset..max_offset),
            rng.gen_range(-max_offset..max_offset),
            0.,
        )
}

fn hand_card_rot(card_index: usize, pile_size: usize) -> f32 {
    let i = card_index_mult(card_index, pile_size);
    (-10. * i).to_radians()
}

fn hand_card_rot_with_offset(card_index: usize, pile_size: usize, rng: &mut ThreadRng) -> f32 {
    let max_rot_offset = 5f32;
    hand_card_rot(card_index, pile_size)
        + rng.gen_range(-max_rot_offset..max_rot_offset).to_radians()
}

fn card_added_to_hand(
    trig: Trigger<OnAdd, HandCard>,
    mut cmd: Commands,
    hand: Single<&CardsInHand>,
    rotation_q: Query<&RotationRoot>,
) {
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

    let animation_duration = 300;
    let mut rng = thread_rng();
    let mut whole_hand = hand.entities().to_vec();
    // new card is not in the target pile yet
    whole_hand.push(trig.target());
    for (i, e) in whole_hand.iter().enumerate() {
        let pos = hand_card_pos_with_offset(i, whole_hand.len(), &mut rng);
        let rot = hand_card_rot_with_offset(i, whole_hand.len(), &mut rng);
        or_continue!(cmd.get_entity(*e)).try_insert(tween::get_relative_translation_3d_anim(
            pos,
            animation_duration,
            Some(EaseFunction::BackOut),
        ));
        let rot_e = or_continue!(rotation_q.get(*e)).entity();
        or_continue!(cmd.get_entity(rot_e)).try_insert(tween::get_relative_z_rotation_anim(
            rot,
            animation_duration,
            None,
        ));
    }
}

fn restore_empty_piles<T: RelationshipTarget>(trig: Trigger<OnRemove, T>, mut cmd: Commands) {
    // reinsert piles to retrigger adding missing empty piles
    or_return!(cmd.get_entity(trig.target())).insert(Piles);
}

fn check_hand_size(
    piles_q: Query<(Entity, &DrawPile, &CardsInHand, &DiscardPile), Changed<CardsInHand>>,
    mut cmd: Commands,
    rotation_q: Query<&RotationRoot, Without<CardSelected>>,
) {
    let (piles_e, draw, hand, discard) = or_return_quiet!(piles_q.single());
    info!("thingy!");
    if hand.is_empty() {
        info!("draw new cards pls!");
        let mut to_draw = Vec::new();
        for e in draw.0.iter().rev().take(3) {
            or_continue!(cmd.get_entity(*e)).try_remove::<DrawPileCard>();
            to_draw.push(*e);
        }
        let hand_size = START_HAND_SIZE as usize;
        if to_draw.len() < hand_size {
            // not enough cards in the draw pile
            // shuffle the discard, take the rest from there, then move to rest to the draw pile
            let mut new_draw = discard.0.clone();
            let mut rng = thread_rng();
            new_draw.shuffle(&mut rng);
            for e in new_draw {
                or_continue!(cmd.get_entity(e)).try_remove::<DiscardPileCard>();
                if to_draw.len() < hand_size {
                    to_draw.push(e);
                } else {
                    or_continue!(cmd.get_entity(e)).try_insert(DrawPileCard(piles_e));
                }
            }
        }
        for e in to_draw {
            or_continue!(cmd.get_entity(e)).try_insert(HandCard(piles_e));
        }
    } else {
        // hand cards have changed => just tween their positions
        let anim_dur_ms = 200;
        let mut rng = thread_rng();
        for (i, e) in hand.entities().iter().enumerate() {
            let pos = hand_card_pos_with_offset(i, hand.len(), &mut rng);
            let rot = hand_card_rot_with_offset(i, hand.len(), &mut rng);
            or_return_quiet!(cmd.get_entity(*e)).insert(tween::get_relative_translation_anim(
                pos.truncate(),
                anim_dur_ms,
                None,
            ));
            let rotation_root = or_return_quiet!(rotation_q.get(*e));
            or_return_quiet!(cmd.get_entity(rotation_root.entity()))
                .insert(tween::get_relative_z_rotation_anim(rot, anim_dur_ms, None));
        }
    }
}

fn card_added_to_discard(
    trig: Trigger<OnAdd, DiscardPileCard>,
    mut cmd: Commands,
    observer_q: Query<(Entity, &Observer)>,
    card_rot_q: Query<&RotationRoot>,
    discard: Single<&DiscardPile>,
) {
    let anim_dur_ms = 300;
    let mut rng = thread_rng();
    let (new_pos, new_rot) = discard_pile_card_pos_rot(&mut rng, discard.len() as i16);
    or_return!(cmd.get_entity(trig.target())).insert(tween::get_relative_translation_3d_anim(
        new_pos,
        anim_dur_ms,
        None,
    ));
    let rotation_e = or_return!(card_rot_q.get(trig.target())).entity();
    or_return!(cmd.get_entity(rotation_e)).try_insert(tween::get_relative_z_rotation_anim(
        new_rot,
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
    card_q: Query<Has<CardSelected>, Without<PriorityTween<Transform>>>,
    hand: Single<&CardsInHand>,
) {
    let card_e = trig.target();
    let is_selected = or_return!(card_q.get(card_e));
    if is_selected {
        return;
    }
    let i = card_index_from_slice(hand.entities(), trig.target());
    let pos = hand_card_pos(i, hand.len());

    or_return_quiet!(cmd.get_entity(card_e)).insert(tween::get_relative_translation_anim(
        pos.truncate().with_y(FOCUSED_CARD_Y),
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
    hand: Single<&CardsInHand>,
) {
    let rotation_root = or_return_quiet!(card_q.get(trig.target()));
    let i = card_index_from_slice(hand.entities(), trig.target());
    let rot = hand_card_rot_with_offset(i, hand.len(), &mut thread_rng());

    or_return_quiet!(cmd.get_entity(rotation_root.entity()))
        .insert(tween::get_relative_z_rotation_anim(rot, 250, None));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_unfocused_card(
    trig: Trigger<OnRemove, CardFocused>,
    mut cmd: Commands,
    card_q: Query<Has<CardSelected>, Without<PriorityTween<Transform>>>,
    hand: Single<&CardsInHand>,
) {
    let card_e = trig.target();
    let is_selected = or_return_quiet!(card_q.get(card_e));
    if is_selected {
        return;
    }
    let i = card_index_from_slice(hand.entities(), trig.target());
    let pos = hand_card_pos_with_offset(i, hand.len(), &mut thread_rng());

    or_return!(cmd.get_entity(card_e)).insert(Animator::new(Tracks::new([
        tween::get_relative_translation_tween(pos.truncate(), 250, Some(EaseFunction::BackOut)),
        // reset size in case other interactions overlap
        tween::get_relative_scale_tween(Vec2::splat(1.), 220, Some(EaseFunction::QuinticOut)),
    ])));
}

#[cfg_attr(feature = "native_dev", hot)]
fn move_selected_card(trig: Trigger<OnAdd, CardSelected>, mut cmd: Commands) {
    or_return!(cmd.get_entity(trig.target())).insert(Animator::new(Tracks::new([
        Box::new(tween::get_relative_translation_3d_tween(
            Vec3::new(-470., 0., 5.),
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
    hand: Single<&CardsInHand>,
) {
    let i = card_index_from_slice(hand.entities(), trig.target());
    let pos = hand_card_pos(i, hand.len());

    or_return!(cmd.get_entity(trig.target())).insert((
        Animator::new(Tracks::new([
            tween::get_relative_translation_tween(pos.truncate(), 400, Some(EaseFunction::BackOut))
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

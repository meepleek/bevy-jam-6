use crate::prelude::tween::get_absolute_translation_anim;
use crate::prelude::tween::get_relative_translation_anim;
use crate::prelude::tween::get_relative_z_rotation_anim;
use crate::prelude::*;

relationship_1_to_n!(DrawPileCard, DrawPile);
relationship_1_to_n!(HandCard, CardsInHand);
relationship_1_to_n!(HandCardObserver, CardsInHandObservers);
relationship_1_to_n!(DiscardCard, DiscardPile);

pub(super) fn plugin(app: &mut App) {
    app.add_observer(card_added_to_discard);
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
    or_return!(cmd.get_entity(trig.target())).insert(get_absolute_translation_anim(
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
    or_return!(cmd.get_entity(rotation_e)).try_insert(get_relative_z_rotation_anim(
        angle.to_radians(),
        anim_dur_ms,
        None,
    ));
    remove_observers_for_watched_entity(&mut cmd, observer_q, trig.target());
}

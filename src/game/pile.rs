use crate::prelude::tween::get_relative_translation_anim;
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
) {
    or_return!(cmd.get_entity(trig.target())).insert(get_relative_translation_anim(
        Vec2::new(-510., 200.),
        300,
        Some(EaseFunction::BackOut),
    ));
    remove_observers_for_watched_entity(&mut cmd, observer_q, trig.target());
}

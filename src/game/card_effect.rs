use std::ops::RangeInclusive;

use crate::game::action::*;
use crate::game::pile::DiscardPile;
use crate::game::pile::DiscardPileCard;
use crate::game::pile::HandCard;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(play_selected_tile_card)
        .add_observer(play_card);
}

// #[derive(Debug)]
// pub struct CardEffect {
//     pub action: CardAction,
//     pub conditions: Vec<CardActionCondition>,
//     // todo: trash effect?
// }
// impl CardEffect {
//     pub fn new(action: CardAction) -> Self {
//         Self {
//             action,
//             conditions: Vec::default(),
//         }
//     }
// }

pub enum ActionTrigger {
    CardSelection,
    TileSelection(Vec<Coords>),
}

#[derive(Debug)]
pub enum CardAction {
    Move {
        reach: EffectReach,
        direction: EffectDirection,
        pip_cost: u8,
    },
    Attack {
        reach: EffectReach,
        direction: EffectDirection,
        attack: u8,
        pip_cost: u8,
        poison: bool,
    },
    HealSelf(u8),
    Junk,
}
impl CardAction {
    pub fn trigger(&self) -> Option<ActionTrigger> {
        match self {
            CardAction::Move {
                reach, direction, ..
            }
            | CardAction::Attack {
                reach, direction, ..
            } => {
                let range = match *reach {
                    EffectReach::Exact(val) => val as i16..=val as i16,
                    EffectReach::Range(max) => 1..=max as i16,
                };
                Some(ActionTrigger::TileSelection(match direction {
                    EffectDirection::Area => match *reach {
                        EffectReach::Exact(val) => {
                            let val = val as i16;
                            let range = -val..=val;
                            let mut res = HashSet::with_capacity(val as usize * 2 * 5);
                            res.extend(
                                range
                                    .clone()
                                    .flat_map(|x| [-val, val].map(|y| Coords::new(x, y))),
                            );
                            res.extend(range.flat_map(|y| [-val, val].map(|x| Coords::new(x, y))));
                            res.into_iter().collect()
                        },
                        EffectReach::Range(max) => {
                            let max = max as i16;
                            let range = -max..=max;
                            range
                                .clone()
                                .flat_map(|y| range.clone().map(move |x| (x, y).into()))
                                .filter(|tile| tile != &Coords::ZERO)
                                .collect()
                        },
                    },
                    EffectDirection::Orthogonal => range
                        .flat_map(|i| {
                            [(0, -1), (0, 1), (-1, 0), (1, 0)]
                                .map(|(sign_x, sign_y)| Coords::new(sign_x, sign_y) * i)
                        })
                        .collect(),
                    EffectDirection::Diagonal => range
                        .flat_map(|i| {
                            [(-1, -1), (-1, 1), (1, -1), (1, 1)]
                                .map(|(sign_x, sign_y)| Coords::new(sign_x, sign_y) * i)
                        })
                        .collect(),
                }))
            },
            CardAction::HealSelf(_) => Some(ActionTrigger::CardSelection),
            CardAction::Junk => None,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            CardAction::Move { .. } => "Move",
            CardAction::Attack { poison: true, .. } => "Poison",
            CardAction::Attack { .. } => "Attack",
            CardAction::HealSelf(_) => "Heal self",
            CardAction::Junk => "Junk",
        }
    }

    // todo: kind
    // like action, passive, timed passive?

    pub fn pip_change(&self) -> Option<i8> {
        match self {
            CardAction::Move { pip_cost, .. } | CardAction::Attack { pip_cost, .. } => {
                Some(-(*pip_cost as i8))
            },
            CardAction::HealSelf(heal) => Some(*heal as i8),
            CardAction::Junk => None,
        }
    }

    pub fn tile_interaction_palette(&self) -> Option<TileInteractionPalette> {
        match self {
            CardAction::Move { .. } => Some(TileInteractionPalette::new(LIME_400, GREEN_800)),
            CardAction::Attack { poison: true, .. } => {
                Some(TileInteractionPalette::new(PURPLE_500, PURPLE_900))
            },
            CardAction::Attack { .. } => Some(TileInteractionPalette::new(ROSE_300, RED_400)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TileInteractionPalette {
    pub highlight: Color,
    pub hover: Color,
}
impl TileInteractionPalette {
    pub fn new(highlight: impl Into<Color>, hover: impl Into<Color>) -> Self {
        Self {
            highlight: highlight.into(),
            hover: hover.into(),
        }
    }
}

#[derive(Debug)]
pub enum EffectDirection {
    Area,
    Orthogonal,
    Diagonal,
}

#[derive(Debug)]
pub enum EffectReach {
    Exact(u8),
    Range(u8),
}

#[derive(Debug)]
pub enum CardActionCondition {
    PipCount(RangeInclusive<u8>),
}

// pub enum CardActionKind {
//     Play,
//     Discard,
//     Trash,
//     // HeldInHand,
//     // InDiscard,
// }

#[derive(Event)]
pub struct PlayCard(pub Entity);

fn play_card(
    trig: Trigger<PlayCard>,
    card_q: Query<&Card>,
    selected_cards: Query<Entity, With<SelectedTileTriggerCard>>,
    player: Single<Entity, With<Player>>,
    discard_pile: Single<Entity, With<DiscardPile>>,
    mut cmd: Commands,
) {
    let card = or_return!(card_q.get(trig.0));
    match &card.action {
        CardAction::HealSelf(heal) => cmd.trigger(PipChange {
            agent_e: *player,
            change: PipChangeKind::Offset(*heal as i8),
        }),
        _ => {
            error!(?card, "Card should not have been played on selection");
            unreachable!();
        },
    }
    or_return!(cmd.get_entity(trig.0))
        .try_remove::<HandCard>()
        .try_insert(DiscardPileCard(discard_pile.into_inner()));
    // deselect any (other) selected tile cards on play
    for selected_card_e in &selected_cards {
        or_return!(cmd.get_entity(selected_card_e)).try_remove::<SelectedTileTriggerCard>();
    }
}

#[derive(Event)]
pub struct PlaySelectedTileCard(pub Coords);

fn play_selected_tile_card(
    trig: Trigger<PlaySelectedTileCard>,
    selected_card: Single<(Entity, &Card), With<SelectedTileTriggerCard>>,
    player: Single<Entity, With<Player>>,
    discard_pile: Single<Entity, With<DiscardPile>>,
    mut cmd: Commands,
) {
    let (card_e, card) = selected_card.into_inner();
    match &card.action {
        CardAction::Move { pip_cost, .. } => cmd.trigger(MoveAction {
            agent_e: *player,
            target_tile: trig.0,
            pip_cost: *pip_cost,
        }),
        CardAction::Attack {
            reach,
            direction,
            attack,
            pip_cost,
            poison,
        } => todo!(),
        _ => {
            error!(?card, "Card should not have been played on selection");
            unreachable!();
        },
    }

    or_return!(cmd.get_entity(card_e))
        .try_remove::<SelectedTileTriggerCard>()
        .try_remove::<HandCard>()
        .try_insert(DiscardPileCard(discard_pile.into_inner()));
}

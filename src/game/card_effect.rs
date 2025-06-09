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

pub enum TileTarget {
    EmptyTiles,
    Dice,
}

#[derive(Debug, Clone)]
pub enum CardActionTrigger {
    CardSelection(CardAction),
    TileSelection(TileCardAction),
}
impl TileActionCommon for CardActionTrigger {
    fn title(&self) -> &str {
        match self {
            CardActionTrigger::CardSelection(action) => action.title(),
            CardActionTrigger::TileSelection(action) => action.title(),
        }
    }

    fn pip_change(&self) -> Option<i8> {
        match self {
            CardActionTrigger::CardSelection(action) => action.pip_change(),
            CardActionTrigger::TileSelection(action) => action.pip_change(),
        }
    }
}

pub trait TileActionCommon {
    fn title(&self) -> &str;
    fn pip_change(&self) -> Option<i8>;
    // todo: kind
    // like action, passive, timed passive?
}

#[derive(Debug, Clone)]
pub enum TileCardAction {
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
    Heal {
        reach: EffectReach,
        direction: EffectDirection,
        heal: u8,
    },
    // Reroll {
    //     reach: EffectReach,
    //     direction: EffectDirection,
    //     pip_cost: u8,
    // },
}
impl TileActionCommon for TileCardAction {
    fn title(&self) -> &str {
        use TileCardAction::*;
        match self {
            Move { .. } => "Move",
            Attack { poison: true, .. } => "Poison",
            Attack { .. } => "Attack",
            Heal { .. } => "Heal",
        }
    }

    fn pip_change(&self) -> Option<i8> {
        use TileCardAction::*;
        match self {
            Move { pip_cost, .. } | Attack { pip_cost, .. } => Some(-(*pip_cost as i8)),
            Heal { heal, .. } => Some(*heal as i8),
        }
    }
}
impl TileCardAction {
    pub fn tile_target(&self) -> TileTarget {
        use TileCardAction::*;

        match self {
            Move { .. } => TileTarget::EmptyTiles,
            Attack { .. } | Heal { .. } => TileTarget::Dice,
        }
    }

    pub fn tiles(&self) -> Vec<Coords> {
        use TileCardAction::*;
        match self {
            Move {
                reach, direction, ..
            }
            | Attack {
                reach, direction, ..
            }
            | Heal {
                reach, direction, ..
            } => {
                let range = match *reach {
                    EffectReach::Exact(val) => val as i16..=val as i16,
                    EffectReach::Range(max) => 1..=max as i16,
                };
                let mut tiles: Vec<_> = match direction {
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
                };
                if matches!(self.tile_target(), TileTarget::Dice) {
                    tiles.push(Coords::ZERO);
                }
                tiles
            },
        }
    }

    pub fn tile_interaction_palette(&self) -> TileInteractionPalette {
        use TileCardAction::*;
        match self {
            Move { .. } => TileInteractionPalette::new(INDIGO_400, INDIGO_800),
            Attack { poison: true, .. } => TileInteractionPalette::new(PURPLE_500, PURPLE_900),
            Heal { .. } => TileInteractionPalette::new(LIME_400, GREEN_800),
            Attack { .. } => TileInteractionPalette::new(ROSE_300, RED_400),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardAction {
    HealSelf(u8),
    RerollSelf,
    // Junk,
}
impl TileActionCommon for CardAction {
    fn title(&self) -> &str {
        use CardAction::*;
        match self {
            HealSelf(_) => "Heal self",
            RerollSelf => "Reroll self",
        }
    }

    fn pip_change(&self) -> Option<i8> {
        use CardAction::*;
        match self {
            HealSelf(heal) => Some(*heal as i8),
            RerollSelf => None,
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

#[derive(Debug, Clone)]
pub enum EffectDirection {
    Area,
    Orthogonal,
    #[allow(dead_code)]
    Diagonal,
}

#[derive(Debug, Clone)]
pub enum EffectReach {
    Exact(u8),
    Range(u8),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
    selected_cards: Query<Entity, With<SelectedTileTriggerCard>>,
    player: Single<Entity, With<Player>>,
    discard_pile: Single<Entity, With<DiscardPile>>,
    card_q: Query<&Card>,
    mut cmd: Commands,
) {
    use CardAction::*;
    let card = or_return!(card_q.get(trig.0));
    match &card.trigger {
        CardActionTrigger::CardSelection(action) => match action {
            HealSelf(heal) => cmd.trigger(PipChangeAction {
                agent_e: *player,
                change: PipChangeKind::Offset(*heal as i8),
            }),
            RerollSelf => cmd.trigger(PipChangeAction {
                agent_e: *player,
                change: PipChangeKind::Reroll,
            }),
        },
        CardActionTrigger::TileSelection(_) => {
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
pub struct PlaySelectedTileCard {
    pub card_e: Entity,
    pub selected_tile: Coords,
}

fn play_selected_tile_card(
    trig: Trigger<PlaySelectedTileCard>,
    player: Single<Entity, With<Player>>,
    discard_pile: Single<Entity, With<DiscardPile>>,
    grid: Single<&Grid>,
    card_q: Query<&Card>,
    mut cmd: Commands,
) {
    use TileCardAction::*;
    let card = or_return!(card_q.get(trig.card_e));
    match &card.trigger {
        CardActionTrigger::TileSelection(tile_card_action) => {
            match tile_card_action {
                Move { pip_cost, .. } => cmd.trigger(MoveAction {
                    agent_e: *player,
                    to: trig.selected_tile,
                    pip_cost: *pip_cost,
                }),
                Attack {
                    attack,
                    pip_cost,
                    // poison,
                    ..
                } => {
                    cmd.trigger(PipChangeAction {
                        change: PipChangeKind::Offset(-(*attack as i8)),
                        agent_e: or_return!(grid.coords_to_tile_entity(trig.selected_tile)).entity,
                    });
                    cmd.trigger(PipChangeAction {
                        change: PipChangeKind::Offset(-(*pip_cost as i8)),
                        agent_e: *player,
                    });
                },
                Heal { heal, .. } => {
                    cmd.trigger(PipChangeAction {
                        change: PipChangeKind::Offset(*heal as i8),
                        agent_e: or_return!(grid.coords_to_tile_entity(trig.selected_tile)).entity,
                    });
                },
            }
        },
        CardActionTrigger::CardSelection(_) => {
            error!(?card, "Card should not have been played on tile selection");
            unreachable!();
        },
    }
    or_return!(cmd.get_entity(trig.card_e))
        .try_remove::<SelectedTileTriggerCard>()
        .try_remove::<HandCard>()
        .try_insert(DiscardPileCard(discard_pile.into_inner()));
}

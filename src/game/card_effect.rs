use std::ops::RangeInclusive;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {}

pub struct CardEffect {
    pub action: CardAction,
    pub conditions: Vec<CardActionCondition>,
    // todo: trash effect?
}
impl CardEffect {
    pub fn new(action: CardAction) -> Self {
        Self {
            action,
            conditions: Vec::default(),
        }
    }
}

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
    Junk,
}
impl CardAction {
    pub fn effect_tiles(&self) -> Vec<Coords> {
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
                match direction {
                    EffectDirection::Area => range
                        .clone()
                        .flat_map(|y| range.clone().map(move |x| (x, y).into()))
                        .collect(),
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
                }
            },
            _ => Vec::default(),
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

pub enum EffectDirection {
    Area,
    Orthogonal,
    Diagonal,
}

pub enum EffectReach {
    Exact(u8),
    Range(u8),
}

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

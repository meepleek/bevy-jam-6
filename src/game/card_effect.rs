use std::ops::RangeInclusive;

use bevy::math::I16Vec2;

use crate::game::card::Card;
use crate::game::card::CardSelected;
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
    pub fn effect_tiles(&self) -> Vec<I16Vec2> {
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
                                .map(|(sign_x, sign_y)| I16Vec2::new(sign_x, sign_y) * i)
                        })
                        .collect(),
                    EffectDirection::Diagonal => range
                        .flat_map(|i| {
                            [(-1, -1), (-1, 1), (1, -1), (1, 1)]
                                .map(|(sign_x, sign_y)| I16Vec2::new(sign_x, sign_y) * i)
                        })
                        .collect(),
                }
            },
            _ => Vec::default(),
        }
    }

    pub fn grid_interaction_palette(&self) -> Option<GridInteractionPalette> {
        match self {
            CardAction::Move { .. } => Some(GridInteractionPalette::new(LIME_400, GREEN_800)),
            CardAction::Attack { poison: true, .. } => {
                Some(GridInteractionPalette::new(PURPLE_500, PURPLE_900))
            },
            CardAction::Attack { .. } => Some(GridInteractionPalette::new(ROSE_300, RED_400)),
            _ => None,
        }
    }
}

pub struct GridInteractionPalette {
    highlight: Color,
    hover: Color,
}
impl GridInteractionPalette {
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

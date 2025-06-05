use crate::prelude::*;

pub fn plugin(_app: &mut App) {}

pub enum CardEffect {
    Move(u8),
}

#[derive(Component)]
#[require(Transform)]
pub struct Card {
    effect: CardEffect,
}

pub fn card(effect: CardEffect, position: Vec2) -> impl Bundle {
    (
        Name::new("card_root"),
        Transform::from_translation(position.extend(0.)),
        Visibility::default(),
        children![(
            Name::new("card"),
            Card { effect },
            Sprite::from_color(AMBER_100, Vec2::new(160., 240.)),
            Pickable::default(),
            Patch(|b| {
                b.observe(on_card_pointer_over)
                    .observe(on_card_click)
                    .observe(on_card_pointer_out);
            }),
        )],
    )
}

fn on_card_pointer_over(t: Trigger<Pointer<Over>>, mut card_q: Query<&mut Transform, With<Card>>) {
    info!("point over card");
    let mut t = or_return!(card_q.get_mut(t.target()));
    t.translation += Vec3::Y * 50.;
}

fn on_card_pointer_out(t: Trigger<Pointer<Out>>, mut card_q: Query<&mut Transform, With<Card>>) {
    info!("point over card");
    let mut t = or_return!(card_q.get_mut(t.target()));
    t.translation = Vec3::ZERO;
}

fn on_card_click(t: Trigger<Pointer<Click>>, mut card_q: Query<&mut Transform, With<Card>>) {
    info!("card click");
}

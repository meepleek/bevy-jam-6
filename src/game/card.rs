use crate::game::card_effect::CardAction;
use crate::prelude::*;
use crate::util;

pub const CARD_BORDER_COL: Srgba = GRAY_950;
pub const CARD_BORDER_COL_FOCUS: Srgba = AMBER_400;

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
        }),
    )
}

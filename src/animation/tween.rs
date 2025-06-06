#![allow(dead_code)]

use std::marker::PhantomData;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::*;

#[derive(Component)]
pub enum DespawnOnTweenCompleted {
    Itself,
    Entity(Entity),
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct TweenFactor<T: Send + Sync + 'static> {
    timer: Timer,
    delay: Option<Timer>,
    ease: EaseFunction,
    _phantom: PhantomData<T>,
}

impl<T: Send + Sync> TweenFactor<T> {
    pub fn new(duration_ms: u64, ease: EaseFunction) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(duration_ms), TimerMode::Once),
            delay: None,
            ease,
            _phantom: default(),
        }
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay = Some(Timer::new(Duration::from_millis(delay_ms), TimerMode::Once));
        self
    }

    pub fn factor(&self) -> f32 {
        self.timer.fraction()
    }
}

pub fn tween_factor<T: Send + Sync>(mut factor_q: Query<&mut TweenFactor<T>>, time: Res<Time>) {
    for mut factor in &mut factor_q {
        if let Some(delay) = factor.delay.as_mut() {
            delay.tick(time.delta());
            if delay.just_finished() {
                factor.delay.take();
            }
        } else if !factor.timer.finished() {
            factor.timer.tick(time.delta());
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TweeningPlugin).add_systems(
        Update,
        (
            component_animator_system::<BackgroundColor>,
            despawn_after_tween,
            process_priority_tween::<Transform>,
            process_priority_tween::<()>,
        ),
    );
}

fn despawn_after_tween(
    mut cmd: Commands,
    mut ev_r: EventReader<TweenCompleted>,
    despawn_q: Query<&DespawnOnTweenCompleted>,
) {
    for ev in ev_r.read() {
        if let Ok(despawn) = despawn_q.get(ev.entity) {
            let e = match despawn {
                DespawnOnTweenCompleted::Itself => ev.entity,
                DespawnOnTweenCompleted::Entity(e) => *e,
            };
            cmd.entity(e).despawn();
        }
    }
}

#[derive(Component, Default)]
pub struct PriorityTween<T = ()>(PhantomData<T>);

fn process_priority_tween<T: Send + Sync + 'static>(
    mut cmd: Commands,
    mut ev_r: EventReader<TweenCompleted>,
    despawn_q: Query<(), With<PriorityTween<T>>>,
) {
    for ev in ev_r.read() {
        if despawn_q.contains(ev.entity) {
            cmd.entity(ev.entity).remove::<PriorityTween<T>>();
        }
    }
}

pub fn delay_tween<T: 'static>(tween: Tween<T>, delay_ms: u64) -> Sequence<T> {
    if delay_ms > 0 {
        Delay::new(Duration::from_millis(delay_ms)).then(tween)
    } else {
        Sequence::new([tween])
    }
}

relative_tween_fns!(
    translation,
    Animator,
    Transform,
    TransformRelativePositionLens,
    Vec3,
    Vec2
);

relative_tween_fns!(
    scale,
    Animator,
    Transform,
    TransformRelativeScaleLens,
    Vec3,
    Vec2
);

relative_tween_fns!(
    z_rotation,
    Animator,
    Transform,
    TransformRelativeZRotationLens,
    Quat,
    f32
);

relative_tween_fns!(text_color, Animator, TextColor, TextColorLens, Color, Color);

relative_tween_fns!(
    ui_bg_color,
    Animator,
    BackgroundColor,
    BackgroundColorLens,
    Color,
    Color
);

relative_tween_fns!(
    ui_image_color,
    Animator,
    ImageNode,
    ImageNodeColorLens,
    Color,
    Color
);

relative_tween_fns!(
    color_material_color,
    AssetAnimator,
    ColorMaterial,
    ColorMaterialRelativeColorLens,
    Color,
    Color
);

relative_lens_struct!(TransformRelativeScaleLens, Vec3, Vec2);
relative_lens_struct!(TransformRelativePositionLens, Vec3, Vec2);
relative_lens_struct!(TransformRelativeZRotationLens, Quat, f32);

impl Lens<Transform> for TransformRelativeScaleLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let start = self.start.unwrap();
        let start_2d = start.truncate();
        let value = start_2d + (self.end - start_2d) * ratio;
        target.scale = value.extend(start.z);
    }

    fn update_on_tween_start(
        &mut self,
        target: &mut dyn Targetable<Transform>,
        _direction: TweeningDirection,
        _times_completed: i32,
    ) {
        self.start.get_or_insert_with(|| target.scale);
    }
}

impl Lens<Transform> for TransformRelativePositionLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let start = self.start.unwrap();
        let start_2d = start.truncate();
        let value = start_2d + (self.end - start_2d) * ratio;
        target.translation = value.extend(start.z);
    }

    fn update_on_tween_start(
        &mut self,
        target: &mut dyn Targetable<Transform>,
        _direction: TweeningDirection,
        _times_completed: i32,
    ) {
        self.start.get_or_insert_with(|| target.translation);
    }
}

impl Lens<Transform> for TransformRelativeZRotationLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let start = self.start.unwrap();
        let z = start.to_euler(EulerRot::XYZ).2;
        let tweened_z = z + (self.end - z) * ratio;
        target.rotation = Quat::from_rotation_z(tweened_z);
    }

    fn update_on_tween_start(
        &mut self,
        target: &mut dyn Targetable<Transform>,
        _direction: TweeningDirection,
        _times_completed: i32,
    ) {
        self.start.get_or_insert_with(|| target.rotation);
    }
}

#[derive(Default)]
pub struct TransformRelativeByPositionLens {
    start: Vec3,
    end: Vec3,
    move_by: Vec3,
}

impl TransformRelativeByPositionLens {
    #[allow(dead_code)]
    pub fn new(move_by: Vec3) -> Self {
        Self {
            move_by,
            start: Vec3::ZERO,
            end: Vec3::ZERO,
        }
    }
}

impl Lens<Transform> for TransformRelativeByPositionLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.translation = value;
    }

    fn update_on_tween_start(
        &mut self,
        target: &mut dyn Targetable<Transform>,
        _direction: TweeningDirection,
        _times_completed: i32,
    ) {
        self.start = target.translation;
        self.end = target.translation + self.move_by;
    }
}

color_lens!(Sprite, SpriteRelativeColorLens, color);
relative_tween_fns!(
    sprite_color,
    Animator,
    Sprite,
    SpriteRelativeColorLens,
    Color,
    Color
);
color_lens!(TextColor, TextColorLens, 0);
color_lens!(BackgroundColor, BackgroundColorLens, 0);
color_lens!(ImageNode, ImageNodeColorLens, color);
color_lens!(ColorMaterial, ColorMaterialRelativeColorLens, color);

pub fn lerp_color(from: Color, to: Color, ratio: f32) -> Color {
    let start = from.to_linear().to_vec4();
    let end: Vec4 = to.to_linear().to_vec4();
    let lerped = start.lerp(end, ratio);
    Color::LinearRgba(LinearRgba::from_vec4(lerped))
}

macro_rules! relative_lens_struct {
    ($lens:ident, $value:ty) => {
        relative_lens_struct!($lens, $value, $value);
    };
    ($lens:ident, $initial_value:ty, $value:ty) => {
        #[derive(Default)]
        pub struct $lens {
            pub(super) start: Option<$initial_value>,
            pub(super) end: $value,
        }

        impl $lens {
            #[allow(dead_code)]
            pub fn relative(end: $value) -> Self {
                Self { start: None, end }
            }

            #[allow(dead_code)]
            pub fn new(start: $initial_value, end: $value) -> Self {
                Self {
                    start: Some(start),
                    end,
                }
            }
        }
    };
}

pub(super) use relative_lens_struct;

macro_rules! color_lens {
    ($component:ty, $lens:ident, $field:tt) => {
        relative_lens_struct!($lens, Color);

        impl Lens<$component> for $lens {
            fn lerp(&mut self, target: &mut dyn Targetable<$component>, ratio: f32) {
                target.$field = lerp_color(
                    self.start
                        .expect("Lerping has started so initial values should have been set"),
                    self.end,
                    ratio,
                );
            }

            fn update_on_tween_start(
                &mut self,
                target: &mut dyn Targetable<$component>,
                _direction: TweeningDirection,
                _times_completed: i32,
            ) {
                self.start.get_or_insert_with(|| target.$field);
            }
        }
    };
}

pub(super) use color_lens;

macro_rules! relative_lens {
    ($component:ty, $value:ty, $lens:ident, $field:tt) => {
        relative_lens_struct!($lens, $value);

        impl Lens<$component> for $lens {
            fn lerp(&mut self, target: &mut dyn Targetable<$component>, ratio: f32) {
                let start = self.start.unwrap();
                let value = start + (self.end - start) * ratio;
                target.$field = value;
            }

            fn update_on_tween_start(
                &mut self,
                target: &mut dyn Targetable<$component>,
                _direction: TweeningDirection,
                _times_completed: i32,
            ) {
                self.start.get_or_insert_with(|| target.$field);
            }
        }
    };
}

pub(super) use relative_lens;

macro_rules! relative_tween_fns {
    ($name:ident, $animator: ty, $component:ty, $lens:ty, $value_start:ty, $value_end:ty) => {
        paste::paste! {
            pub fn [<get_absolute_ $name _tween>](
                start: $value_start,
                end: $value_end,
                duration_ms: u64,
                ease: Option<EaseFunction>,
            ) -> Tween<$component> {
                [<get_ $name _tween>](
                    Some(start),
                    end,
                    duration_ms,
                    ease.unwrap_or(EaseFunction::QuadraticInOut),
                )
            }

            pub fn [<get_relative_ $name _tween>](
                end: $value_end,
                duration_ms: u64,
                ease: Option<EaseFunction>,
            ) -> Tween<$component> {
                [<get_ $name _tween>](
                    None,
                    end,
                    duration_ms,
                    ease.unwrap_or(EaseFunction::QuadraticInOut),
                )
            }

            pub fn [<get_absolute_ $name _anim>](
                start: $value_start,
                end: $value_end,
                duration_ms: u64,
                ease: Option<EaseFunction>,
            ) -> $animator<$component> {
                $animator::new([<get_absolute_ $name _tween>](
                    start,
                    end,
                    duration_ms,
                    ease,
                ))
            }

            pub fn [<get_relative_ $name _anim>](
                end: $value_end,
                duration_ms: u64,
                ease: Option<EaseFunction>,
            ) -> $animator<$component> {
                $animator::new([<get_relative_ $name _tween>](
                    end,
                    duration_ms,
                    ease,
                ))
            }

            pub fn [<get_ $name _anim>](
                start: Option<$value_start>,
                end: $value_end,
                duration_ms: u64,
                ease: EaseFunction,
            ) -> $animator<$component> {
                $animator::new([<get_ $name _tween>](
                    start,
                    end,
                    duration_ms,
                    ease,
                ))
            }

            pub fn [<get_ $name _tween>](
                start: Option<$value_start>,
                end: $value_end,
                duration_ms: u64,
                ease: EaseFunction,
            ) -> Tween<$component> {
                Tween::new(
                    ease,
                    Duration::from_millis(duration_ms),
                    $lens {
                        start,
                        end,
                    },
                ).with_completed_event(0)
            }
        }
    };
}

pub(super) use relative_tween_fns;

use crate::prelude::RelationshipEntity;

pub fn tween_sprite_color_on_trigger<E: Event, B: Bundle>(
    color: impl Into<Color>,
) -> impl FnMut(Trigger<E, B>, Commands) {
    let color = color.into();
    move |trig, mut cmd| {
        let card_e = trig.target();
        tiny_bail::or_return_quiet!(cmd.get_entity(card_e))
            .insert(get_relative_sprite_color_anim(color, 200, None));
    }
}

pub fn tween_related_sprite_color_on_trigger<
    E: Event,
    B: Bundle,
    TSpriteRelationship: Component + RelationshipEntity,
>(
    color: impl Into<Color>,
) -> impl FnMut(Trigger<E, B>, Commands, Query<&TSpriteRelationship>) {
    let color = color.into();
    move |trig, mut cmd, entity_q| {
        let e = tiny_bail::or_return!(entity_q.get(trig.target())).entity();
        tiny_bail::or_return_quiet!(cmd.get_entity(e))
            .insert(get_relative_sprite_color_anim(color, 200, None));
    }
}

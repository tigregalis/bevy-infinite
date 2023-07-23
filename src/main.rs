#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::{
    math::I64Vec2,
    prelude::*,
    render::camera::RenderTarget,
    utils::HashMap,
    window::{PrimaryWindow, WindowRef},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldCursor>()
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, updates_world_cursor)
        .add_systems(Update, updates_transforms_from_positions)
        .add_systems(Update, leader_tracks_cursor)
        .add_systems(Update, tails_follow_heads)
        .add_systems(Update, moves_camera)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), WorldPosition::new(0, 0)));

    // this won't move
    spawn_tail(
        &mut commands,
        Entity::PLACEHOLDER,
        Color::WHITE,
        Vec2::splat(100.0),
        -450,
        0,
        0.5,
        false,
    );

    // this won't move
    spawn_tail(
        &mut commands,
        Entity::PLACEHOLDER,
        Color::GRAY,
        Vec2::splat(100.0),
        -150,
        0,
        3.5,
        false,
    );

    // this won't move
    spawn_tail(
        &mut commands,
        Entity::PLACEHOLDER,
        Color::DARK_GRAY,
        Vec2::splat(100.0),
        150,
        0,
        6.5,
        false,
    );

    // this won't move
    spawn_tail(
        &mut commands,
        Entity::PLACEHOLDER,
        Color::BLACK,
        Vec2::splat(100.0),
        450,
        0,
        9.5,
        false,
    );

    // this is an empty entity that just tracks the cursor
    let root_head = commands
        .spawn((Leader, Head, WorldPosition::new(0, 0)))
        .id();
    let tail = spawn_tail(
        &mut commands,
        root_head,
        Color::RED,
        Vec2::splat(100.0),
        0,
        0,
        1.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::ORANGE_RED,
        Vec2::splat(95.0),
        10000,
        0,
        2.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::ORANGE,
        Vec2::splat(90.0),
        10000,
        10000,
        3.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::YELLOW,
        Vec2::splat(85.0),
        0,
        10000,
        4.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::YELLOW_GREEN,
        Vec2::splat(80.0),
        -10000,
        10000,
        5.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::GREEN,
        Vec2::splat(75.0),
        -10000,
        0,
        6.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::AQUAMARINE,
        Vec2::splat(70.0),
        -10000,
        -10000,
        7.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::BLUE,
        Vec2::splat(65.0),
        0,
        -10000,
        8.0,
        true,
    );
    let tail = spawn_tail(
        &mut commands,
        tail,
        Color::INDIGO,
        Vec2::splat(60.0),
        10000,
        -10000,
        9.0,
        true,
    );
    let _tail = spawn_tail(
        &mut commands,
        tail,
        Color::PURPLE,
        Vec2::splat(55.0),
        20000,
        -10000,
        10.0,
        false,
    );
}


9_223_372_036_854_775_807
7_500_000_000_000_000_000

/// 10 world_position per 1 bevy translation unit
const WORLD_SCALE: i64 = 10;

fn spawn_tail(
    commands: &mut Commands,
    head: Entity,
    color: Color,
    size: Vec2,
    x: i64,
    y: i64,
    depth: f32,
    is_head: bool,
) -> Entity {
    let mut spawner = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, depth),
            ..default()
        },
        WorldPosition::new(x, y),
        Tailing(head),
    ));
    if is_head {
        spawner.insert(Head);
    }
    spawner.id()
}

#[derive(Resource, Default)]
struct WorldCursor(I64Vec2);

/// The basic idea is that the camera is always at (0, 0)
/// We store the world position, which is an I64Vec2
#[derive(Deref, DerefMut, Default, Debug, Component, Clone, Copy)]
struct WorldPosition(I64Vec2);

impl WorldPosition {
    fn new(x: i64, y: i64) -> Self {
        Self(I64Vec2::new(x, y))
    }

    fn to_vec3(
        self,
        // may not be relevant, as the camera should always be at (0, 0)
        self_transform: &Transform,
        camera_transform: &GlobalTransform,
        camera_position: &WorldPosition,
        scale: i64,
    ) -> Vec3 {
        let relative_position = self.0 - camera_position.0;
        let new_translation =
            camera_transform.translation().truncate() + relative_position.as_vec2() / scale as f32;
        new_translation.extend(self_transform.translation.z)
    }

    fn from_vec2(translation: &Vec2, camera_position: &WorldPosition, scale: i64) -> Self {
        let relative_world_position =
            I64Vec2::new(translation.x as i64, translation.y as i64) * scale;
        let world_position = camera_position.0 + relative_world_position;
        Self(world_position)
    }
}

fn updates_world_cursor(
    mut cursor: ResMut<WorldCursor>,
    // need to get window dimensions
    windows: Query<&Window>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &WorldPosition, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    for (camera, camera_position, camera_transform) in camera_q.iter() {
        // get the window that the camera is displaying to (or the primary window)
        let window = if let RenderTarget::Window(WindowRef::Entity(id)) = camera.target {
            windows.get(id).unwrap()
        } else {
            primary_window.single()
        };

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(cursor_translation) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            cursor.0 =
                WorldPosition::from_vec2(&cursor_translation, camera_position, WORLD_SCALE).0;
        }
    }
}

/// This is a dumb approach which moves everything relative to the camera.
/// But you could determine what's in the camera's viewport and update that only.
fn updates_transforms_from_positions(
    mut query: Query<(&mut Transform, &WorldPosition), (Without<Camera>, Without<Parent>)>,
    camera: Query<(&GlobalTransform, &WorldPosition), With<Camera>>,
) {
    let Ok((camera_transform, camera_position)) = camera.get_single() else {
        return;
    };
    for (mut transform, world_position) in query.iter_mut() {
        transform.translation =
            world_position.to_vec3(&transform, camera_transform, camera_position, WORLD_SCALE);
    }
}

#[derive(Component)]
struct Head;

#[derive(Component)]
struct Tailing(Entity);

#[derive(Component)]
struct Leader;

// the leading head just tracks the cursor
fn leader_tracks_cursor(
    mut query: Query<&mut WorldPosition, (With<Leader>, With<Head>, Without<Tailing>)>,
    cursor: Res<WorldCursor>,
) {
    for mut world_position in query.iter_mut() {
        world_position.0 = cursor.0;
    }
}

// tail moves relative to head
fn tails_follow_heads(
    mut params: ParamSet<(
        Query<(&mut WorldPosition, &Tailing)>,
        Query<(Entity, &WorldPosition), With<Head>>,
    )>,
    time: Res<Time>,
) {
    const CATCH_UP_SPEED: i64 = 10;
    const SLACK: i64 = 10;
    let dt = time.delta().as_millis() as i64;
    let heads = params.p1();
    let heads = heads
        .iter()
        .map(|(entity, world_position)| (entity, *world_position))
        .collect::<HashMap<Entity, WorldPosition>>();
    let mut tails = params.p0();
    for (mut tail_position, tailing) in tails.iter_mut() {
        if let Some(head_position) = heads.get(&tailing.0) {
            let dv = head_position.0 - tail_position.0;
            if dv.length_squared() >= SLACK * SLACK {
                // the larger the distance, the faster the tail moves towards the head
                // division by 1000 because dt is in milliseconds
                tail_position.0 += CATCH_UP_SPEED * dv * dt / 1000;
            }
        }
    }
}

fn moves_camera(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut camera: Query<&mut WorldPosition, With<Camera>>,
) {
    let Ok(mut position) = camera.get_single_mut() else {
        return;
    };

    const CAMERA_SPEED: i64 = 2000; // world scale units

    let dt = time.delta().as_millis() as i64;

    for input in input.get_pressed() {
        match input {
            KeyCode::W => {
                position.0.y += CAMERA_SPEED * dt / 1000;
            }
            KeyCode::S => {
                position.0.y -= CAMERA_SPEED * dt / 1000;
            }
            KeyCode::D => {
                position.0.x += CAMERA_SPEED * dt / 1000;
            }
            KeyCode::A => {
                position.0.x -= CAMERA_SPEED * dt / 1000;
            }
            _ => {}
        }
    }
}

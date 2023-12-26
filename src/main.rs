#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_editor_pls::prelude::*;
// use rand::{thread_rng, Rng};

const GRID_CELL: f32 = 60.0;
const GRID_SIZE: u32 = 13;
const GRID_CENTER: u32 = GRID_SIZE / 2;

const WINDOW_SIZE: f32 = GRID_CELL * GRID_SIZE as f32;
const LEFT_WINDOW_BORDER: f32 = -WINDOW_SIZE / 2.;
// const RIGHT_WINDOW_BORDER: f32 = WINDOW_SIZE / 2.;
const TOP_WINDOW_BORDER: f32 = WINDOW_SIZE / 2.;
// const BOTTOM_WINDOW_BORDER: f32 = -WINDOW_SIZE / 2.;

const BACKGROUND_COLOR: Color = Color::rgb(0.24, 0.25, 0.24);

const FOOD_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
const HEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TAIL_COLOR: Color = Color::rgb(0.15, 0.79, 0.58);

const GAME_SPEED: Duration = Duration::from_millis(500);

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".into(),
                resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .register_type::<Cell>()
        .register_type::<Head>()
        .register_type::<Tail>()
        .register_type::<Snake>()
        .insert_resource(Snake::default())
        .add_event::<EatEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                set_snake_direction,
                move_tail.run_if(on_timer(GAME_SPEED)),
                move_head.run_if(on_timer(GAME_SPEED)),
                eat,
                on_eat,
                update_cells_positions.run_if(on_timer(GAME_SPEED)),
            )
                .chain(),
        );

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

#[derive(Component)]
struct Food;

#[derive(Default, Reflect, InspectorOptions, PartialEq)]
#[reflect(InspectorOptions)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match &self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
struct Head {
    direction: Direction,
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
struct Tail;

#[derive(Component, Clone, Copy, Default, PartialEq, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
struct Cell {
    x: u32,
    y: u32,
}

#[derive(Bundle)]
struct CellBundle {
    cell: Cell,
    sprite: SpriteBundle,
}

impl CellBundle {
    fn new(cell: Cell, color: Color) -> Self {
        Self::new_with_z(cell, color, 0.)
    }

    fn new_with_z(cell: Cell, color: Color, z: f32) -> Self {
        let pos = grid_to_screen(cell.x, cell.y);

        Self {
            cell: cell,
            sprite: SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform {
                    translation: Vec3 {
                        x: pos.x,
                        y: pos.y,
                        z,
                    },
                    scale: Vec3 {
                        x: GRID_CELL,
                        y: GRID_CELL,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
struct HeadBundle {
    head: Head,
    cell: CellBundle,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Snake {
    parts: Vec<Entity>,
}

impl Snake {
    fn new(commands: &mut Commands, snake: &mut ResMut<Snake>) -> Entity {
        let cell = Cell {
            x: GRID_CENTER,
            y: GRID_CENTER,
        };

        let id = commands
            .spawn(CellBundle::new_with_z(cell, HEAD_COLOR, 1.))
            .insert(Head {
                direction: Direction::Up,
            })
            .id();
        snake.parts.push(id);
        id
    }

    fn new_tail(
        commands: &mut Commands,
        query: &Query<&Cell, Without<Food>>,
        snake: &mut ResMut<Snake>,
    ) -> Entity {
        let last_id = snake.parts.last().unwrap();
        let last_cell = query.get(*last_id).unwrap();

        let id = commands
            .spawn(CellBundle::new(*last_cell, TAIL_COLOR))
            .insert(Tail)
            .id();
        snake.parts.push(id);
        id
    }
}

#[derive(Event)]
struct EatEvent;

#[derive(Bundle)]
struct FoodBundle {
    food: Food,
    cell: CellBundle,
}

impl FoodBundle {
    fn new(grid_x: u32, grid_y: u32) -> Self {
        let cell = Cell {
            x: grid_x,
            y: grid_y,
        };

        Self {
            food: Food {},
            cell: CellBundle::new(cell, FOOD_COLOR),
        }
    }
}

fn setup(mut commands: Commands, mut snake: ResMut<Snake>) {
    commands.spawn(FoodBundle::new(4, 4));
    commands.spawn(FoodBundle::new(5, 5));
    commands.spawn(FoodBundle::new(8, 8));

    let _: Entity = Snake::new(&mut commands, &mut snake);
    // let id = Snake::new_tail(&mut commands, &mut snake);

    commands.spawn(Camera2dBundle::default());
}

fn set_snake_direction(
    gamepads: Res<Gamepads>,
    keyboard_input: Res<Input<KeyCode>>,
    button_inputs: Res<Input<GamepadButton>>,
    mut query: Query<&mut Head>,
) {
    let gamepad = gamepads.iter().last();

    let gamepad_up: bool = if let Some(gamepad) = gamepad {
        button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp))
    } else {
        false
    };
    let keyboard_up = keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W);

    let gamepad_down: bool = if let Some(gamepad) = gamepad {
        button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown))
    } else {
        false
    };
    let keyboard_down = keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S);

    let gamepad_left: bool = if let Some(gamepad) = gamepad {
        button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft))
    } else {
        false
    };
    let keyboard_left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);

    let gamepad_right: bool = if let Some(gamepad) = gamepad {
        button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight))
    } else {
        false
    };
    let keyboard_right =
        keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

    if let Ok(mut head) = query.get_single_mut() {
        if (keyboard_up || gamepad_up) && head.direction != Direction::Up.opposite() {
            head.direction = Direction::Up;
        }

        if (keyboard_down || gamepad_down) && head.direction != Direction::Down.opposite() {
            head.direction = Direction::Down;
        }

        if (keyboard_left || gamepad_left) && head.direction != Direction::Left.opposite() {
            head.direction = Direction::Left;
        }

        if (keyboard_right || gamepad_right) && head.direction != Direction::Right.opposite() {
            head.direction = Direction::Right;
        }
    }
}

fn update_cells_positions(mut query: Query<(&Cell, &mut Transform), Without<Food>>) {
    for (cell, mut transform) in query.iter_mut() {
        let new_pos = grid_to_screen(cell.x, cell.y);

        transform.translation = Vec3 {
            x: new_pos.x,
            y: new_pos.y,
            z: transform.translation.z,
        };
    }
}

fn move_head(mut query: Query<(&mut Cell, &Head)>) {
    if let Ok(mut q) = query.get_single_mut() {
        match q.1.direction {
            Direction::Up => {
                if q.0.y == 0 {
                    q.0.y = GRID_SIZE - 1;
                } else {
                    q.0.y -= 1;
                }
            }
            Direction::Down => {
                if q.0.y == GRID_SIZE - 1 {
                    q.0.y = 0;
                } else {
                    q.0.y += 1;
                }
            }
            Direction::Left => {
                if q.0.x == 0 {
                    q.0.x = GRID_SIZE - 1;
                } else {
                    q.0.x -= 1;
                }
            }
            Direction::Right => {
                if q.0.x == GRID_SIZE - 1 {
                    q.0.x = 0;
                } else {
                    q.0.x += 1;
                }
            }
        }
    }
}

fn move_tail(mut query: Query<(Entity, &mut Cell), Without<Food>>, snake: Res<Snake>) {
    let mut current_snake_parts: Vec<(Entity, Cell)> = vec![];

    for part in snake.parts.iter() {
        if let Ok(e) = query.get(*part) {
            current_snake_parts.push((e.0, *e.1));
        }
    }

    for (i, tail_id) in snake.parts.iter().enumerate().skip(1) {
        if let Ok(mut world_tail) = query.get_mut(*tail_id) {
            let previous_part = current_snake_parts.get(i - 1).unwrap();
            *world_tail.1 = previous_part.1;
        }
    }
}

fn eat(
    mut commands: Commands,
    head: Query<&Cell, With<Head>>,
    foods: Query<(Entity, &Cell), With<Food>>,
    mut ev_eat: EventWriter<EatEvent>,
) {
    if let Ok(head) = head.get_single() {
        for (id, food) in foods.iter() {
            if *head == *food {
                ev_eat.send(EatEvent);
                commands.entity(id).despawn();
            }
        }
    }
}

fn on_eat(
    mut ev_eat: EventReader<EatEvent>,
    mut commands: Commands,
    query: Query<&Cell, Without<Food>>,
    mut snake: ResMut<Snake>,
) {
    for _ in ev_eat.read() {
        Snake::new_tail(&mut commands, &query, &mut snake);
    }
}

fn grid_to_screen(grid_x: u32, grid_y: u32) -> Vec2 {
    Vec2 {
        x: LEFT_WINDOW_BORDER + GRID_CELL / 2. + GRID_CELL * grid_x as f32,
        y: TOP_WINDOW_BORDER - GRID_CELL / 2. - GRID_CELL * grid_y as f32,
    }
}

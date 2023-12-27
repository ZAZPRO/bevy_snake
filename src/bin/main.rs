#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use snake::libs::{
    cell::Cell,
    food::{Food, FoodBundle},
    globals::{BACKGROUND_COLOR, GAME_SPEED, GRID_SIZE, WINDOW_SIZE},
    input::get_user_input,
    snake::{Direction, Head, Snake, Tail},
    utils::grid_to_screen,
};

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

#[derive(Event)]
struct EatEvent;

fn setup(mut commands: Commands, mut snake: ResMut<Snake>) {
    commands.spawn(FoodBundle::new(4, 4));
    commands.spawn(FoodBundle::new(5, 5));
    commands.spawn(FoodBundle::new(8, 8));

    Snake::new(&mut commands, &mut snake);

    commands.spawn(Camera2dBundle::default());
}

fn set_snake_direction(
    gamepads: Res<Gamepads>,
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    mut query: Query<&mut Head>,
) {
    let user_input_state = get_user_input(gamepads, keyboard_input, button_input);

    if let Ok(mut head) = query.get_single_mut() {
        let direction: Direction = if user_input_state.input_up {
            Direction::Up
        } else if user_input_state.input_down {
            Direction::Down
        } else if user_input_state.input_left {
            Direction::Left
        } else if user_input_state.input_right {
            Direction::Right
        } else {
            head.direction
        };

        if direction != head.direction.opposite() {
            head.direction = direction;
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

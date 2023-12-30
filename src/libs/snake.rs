use std::collections::VecDeque;

use super::{
    audio::AudioAssets,
    cell::{Cell, CellBundle},
    events::EatEvent,
    food::Food,
    game_configuration::GameConfiguration,
    game_states::GameState,
    globals::{GRID_CENTER, GRID_SIZE, HEAD_COLOR, TAIL_COLOR},
    input::{action_events::ActionMoveEvent, direction::Direction},
    schedule::InGameSet,
};
use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};

#[derive(Component, Reflect, Debug)]
pub struct Head {
    pub planned_direction: VecDeque<Direction>,
    pub direction: Direction,
}

#[derive(Component, Reflect)]
pub struct Tail;

#[derive(Bundle)]
pub struct HeadBundle {
    head: Head,
    cell: CellBundle,
}

#[derive(Resource, Default, Reflect)]
pub struct Snake {
    pub parts: Vec<Entity>,
}

impl Snake {
    pub fn create(commands: &mut Commands, snake: &mut ResMut<Snake>) {
        let cell = Cell {
            x: GRID_CENTER,
            y: GRID_CENTER,
        };

        let id = commands
            .spawn(CellBundle::new_with_z(cell, HEAD_COLOR, 1.))
            .insert(Head {
                planned_direction: VecDeque::new(),
                direction: Direction::Up,
            })
            .id();
        snake.parts.push(id);
    }

    pub fn new_tail(
        commands: &mut Commands,
        query: &Query<&Cell, Without<Food>>,
        snake: &mut ResMut<Snake>,
    ) {
        let last_id = snake.parts.last().unwrap();
        let last_cell = query.get(*last_id).unwrap();

        let id = commands
            .spawn(CellBundle::new(*last_cell, TAIL_COLOR))
            .insert(Tail)
            .id();
        snake.parts.push(id);
    }
}

fn set_snake_direction(mut ev_move: EventReader<ActionMoveEvent>, mut query: Query<&mut Head>) {
    for evt in ev_move.read() {
        if let Ok(mut head) = query.get_single_mut() {
            if let Some(last_dir) = head.planned_direction.back() {
                if *last_dir != evt.0 {
                    head.planned_direction.push_back(evt.0);
                }
            } else {
                head.planned_direction.push_back(evt.0);
            }
        }
    }
}

fn move_head(
    mut commands: Commands,
    mut query: Query<(&mut Cell, &mut Head)>,
    audio: Res<AudioAssets>,
    game_configration: Res<GameConfiguration>,
) {
    if game_configration.tick_timer.just_finished() {
        if let Ok((mut cell, mut head)) = query.get_single_mut() {
            let planned_direction = head.planned_direction.pop_front();

            if let Some(plan_dir) = planned_direction {
                if head.direction != plan_dir.opposite() {
                    head.direction = plan_dir;
                } else {
                    head.planned_direction.clear();
                }
            }

            match head.direction {
                Direction::Up => {
                    if cell.y == 0 {
                        cell.y = GRID_SIZE - 1;
                    } else {
                        cell.y -= 1;
                    }
                }
                Direction::Down => {
                    if cell.y == GRID_SIZE - 1 {
                        cell.y = 0;
                    } else {
                        cell.y += 1;
                    }
                }
                Direction::Left => {
                    if cell.x == 0 {
                        cell.x = GRID_SIZE - 1;
                    } else {
                        cell.x -= 1;
                    }
                }
                Direction::Right => {
                    if cell.x == GRID_SIZE - 1 {
                        cell.x = 0;
                    } else {
                        cell.x += 1;
                    }
                }
            }
        }

        commands.spawn(AudioBundle {
            source: audio.snake_movement_sound.clone(),
            settings: PlaybackSettings {
                volume: Volume::Relative(VolumeLevel::new(0.3)),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

fn move_tail(
    mut query: Query<(Entity, &mut Cell), Without<Food>>,
    snake: Res<Snake>,
    game_configration: Res<GameConfiguration>,
) {
    if game_configration.tick_timer.just_finished() {
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
}

fn grow_snake_on_eat(
    mut ev_eat: EventReader<EatEvent>,
    mut commands: Commands,
    query: Query<&Cell, Without<Food>>,
    mut snake: ResMut<Snake>,
    audio: Res<AudioAssets>,
) {
    for _ in ev_eat.read() {
        Snake::new_tail(&mut commands, &query, &mut snake);

        commands.spawn(AudioBundle {
            source: audio.eat_sound.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
            ..default()
        });
    }
}

fn spawn_snake(mut commands: Commands, mut snake: ResMut<Snake>) {
    Snake::create(&mut commands, &mut snake);
}

fn destroy_snake(mut commands: Commands, mut snake: ResMut<Snake>) {
    for part in snake.parts.iter() {
        commands.entity(*part).despawn();
    }

    snake.parts.clear();
}

fn snake_self_collision(
    head: Query<&Cell, With<Head>>,
    tails: Query<&Cell, With<Tail>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Len check is a dirty hack, but it works.
    if tails.iter().len() > 1 {
        if let Ok(head) = head.get_single() {
            for tails in tails.iter() {
                if *head == *tails {
                    next_state.set(GameState::FinishMenu);
                }
            }
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Head>()
            .register_type::<Tail>()
            .register_type::<Snake>()
            .insert_resource(Snake::default())
            .add_systems(OnEnter(GameState::InGame), spawn_snake)
            .add_systems(
                Update,
                (set_snake_direction, move_tail, move_head)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            )
            .add_systems(Update, grow_snake_on_eat.in_set(InGameSet::SpawnEntities))
            .add_systems(
                OnExit(GameState::InGame),
                destroy_snake.in_set(InGameSet::DespawnEntities),
            )
            .add_systems(
                Update,
                snake_self_collision.in_set(InGameSet::CollisionDetection),
            );
    }
}

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use super::{
    cell::{Cell, CellBundle},
    events::EatEvent,
    food::Food,
    game_states::GameState,
    globals::{GAME_SPEED, GRID_CENTER, GRID_SIZE, HEAD_COLOR, TAIL_COLOR},
    input::get_user_input,
    schedule::InGameSet,
};

#[derive(Default, Reflect, InspectorOptions, PartialEq, Clone, Copy)]
#[reflect(InspectorOptions)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
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
pub struct Head {
    pub direction: Direction,
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Tail;

#[derive(Bundle)]
pub struct HeadBundle {
    head: Head,
    cell: CellBundle,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Snake {
    pub parts: Vec<Entity>,
}

impl Snake {
    pub fn new(commands: &mut Commands, snake: &mut ResMut<Snake>) {
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

fn move_head(mut query: Query<(&mut Cell, &Head)>) {
    if let Ok((mut cell, head)) = query.get_single_mut() {
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

fn grow_snake_on_eat(
    mut ev_eat: EventReader<EatEvent>,
    mut commands: Commands,
    query: Query<&Cell, Without<Food>>,
    mut snake: ResMut<Snake>,
) {
    for _ in ev_eat.read() {
        Snake::new_tail(&mut commands, &query, &mut snake);
    }
}

fn spawn_snake(mut commands: Commands, mut snake: ResMut<Snake>) {
    Snake::new(&mut commands, &mut snake);
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
                (
                    set_snake_direction,
                    move_tail.run_if(on_timer(GAME_SPEED)),
                    move_head.run_if(on_timer(GAME_SPEED)),
                )
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

use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Debug, Clone, Copy)]
pub enum Powerup {
    Normal,
    Slowdown,
    Shorten,
    Feast,
    //TODO
    //Ghost,
}

impl Powerup {
    pub fn get_color(&self) -> Color {
        match self {
            Powerup::Normal => Color::rgb(0.9, 0.1, 0.1),
            Powerup::Slowdown => Color::rgb(0.0, 0.0, 0.9),
            Powerup::Shorten => Color::rgb(0.9, 0.9, 0.0),
            Powerup::Feast => Color::rgb(0.0, 0.9, 0.0),
        }
    }

    fn get_chance(&self) -> f32 {
        match self {
            Powerup::Normal => 0.80,
            Powerup::Shorten => 0.1,
            Powerup::Feast => 0.05,
            Powerup::Slowdown => 0.05,
        }
    }

    fn chance_to_powerup(random_number: f32) -> Powerup {
        if random_number < Powerup::Feast.get_chance() {
            Powerup::Feast
        } else if random_number < Powerup::Feast.get_chance() + Powerup::Shorten.get_chance() {
            Powerup::Shorten
        } else if random_number
            < Powerup::Feast.get_chance()
                + Powerup::Shorten.get_chance()
                + Powerup::Slowdown.get_chance()
        {
            Powerup::Slowdown
        } else {
            Powerup::Normal
        }
    }

    pub fn get_random_powerup() -> Powerup {
        let random_number = rand::thread_rng().gen_range(0.0..1.0);
        let powerup = Powerup::chance_to_powerup(random_number);
        powerup
    }

    fn get_speed(&self) -> f32 {
        match self {
            Powerup::Normal => 1.0,
            Powerup::Slowdown => 0.5,
            Powerup::Shorten => 1.0,
            Powerup::Feast => 1.0,
        }
    }

    fn get_duration(&self) -> u32 {
        match self {
            Powerup::Normal => 0,
            Powerup::Slowdown => 5,
            Powerup::Shorten => 5,
            Powerup::Feast => 0,
        }
    }
}

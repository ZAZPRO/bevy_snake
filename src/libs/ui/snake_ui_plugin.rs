use bevy::app::{PluginGroup, PluginGroupBuilder};

use super::{
    finish_menu::FinishMenuPlugin, pause_ui::PauseUiPlugin, score_ui::ScoreUiPlugin,
    start_menu::StartMenuPlugin,
};

pub struct SnakeUiPlugins;

impl PluginGroup for SnakeUiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StartMenuPlugin)
            .add(ScoreUiPlugin)
            .add(PauseUiPlugin)
            .add(FinishMenuPlugin)
    }
}

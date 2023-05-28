use bevy::prelude::*;

pub mod layout;

use crate::game::enemy::resources::Enemies;
use crate::game::{score::resources::Score, ui::hud::components::ScoreText};

use super::components::EnemiesText;

pub fn update_score_in_hud_menu(
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    if let Ok(mut score_text) = score_text_query.get_single_mut() {
        score_text.sections[1].value = score.value.to_string();
    }
}

pub fn update_enemies_in_hud_menu(
    mut enemies_text_query: Query<&mut Text, With<EnemiesText>>,
    enemies: Res<Enemies>,
) {
    if let Ok(mut enemies_text) = enemies_text_query.get_single_mut() {
        enemies_text.sections[1].value = enemies.value.to_string();
    }
}

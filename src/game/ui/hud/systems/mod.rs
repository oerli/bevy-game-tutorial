use bevy::prelude::*;

pub mod interactions;
pub mod layout;

use crate::game::{score::resources::Score, ui::hud::components::ScoreText};

pub fn update_score_in_hud_menu(
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    if let Ok(mut score_text) = score_text_query.get_single_mut() {
        score_text.sections[1].value = score.value.to_string();
        }
}

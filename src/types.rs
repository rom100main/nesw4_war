use crate::{COLOR_GRID_LINE, COLOR_NEUTRAL, COLOR_PLAYER1, COLOR_PLAYER2};
use eframe::egui;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    Neutral,
    Player1,
    Player2,
}

impl CellState {
    pub fn show(self, painter: &egui::Painter, rect: egui::Rect) {
        match self {
            CellState::Neutral => {
                painter.rect_filled(rect, 0.0, COLOR_NEUTRAL);
            }
            CellState::Player1 => {
                painter.rect_filled(rect, 0.0, COLOR_PLAYER1);
            }
            CellState::Player2 => {
                painter.rect_filled(rect, 0.0, COLOR_PLAYER2);
            }
        }

        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(0.5, COLOR_GRID_LINE),
            egui::StrokeKind::Inside,
        );
    }
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Neutral => write!(f, "Neutral"),
            CellState::Player1 => write!(f, "P1"),
            CellState::Player2 => write!(f, "P2"),
        }
    }
}

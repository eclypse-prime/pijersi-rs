use pijersi_rs::logic::Cells;

mod actions;
mod movegen;
mod perft;
mod rules;
mod translate;

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW S- R- SP
///  R- P- S- R- P- .
const TEST_CELLS: Cells = [
    12, 13, 14, 12, 0, 14, 13, 14, 12, 255, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 0, 0,
    0, 0, 0, 0, 0, 9, 8, 10, 187, 8, 10, 152, 10, 9, 8, 10, 9, 0,
];

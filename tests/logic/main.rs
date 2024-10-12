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
const TEST_CELLS: [u8; 45] = [
    3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 0, 0, 0, 0,
    0, 0, 0, 5, 1, 9, 221, 1, 9, 81, 9, 5, 1, 9, 5, 0,
];



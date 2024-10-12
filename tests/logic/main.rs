mod actions;
mod movegen;
mod perft;
mod translate;

/// Cells state for testing
///  s- p- r- s- p- r-
/// p- r- s- ww r- s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  .
/// P- S- R- WW S- R- SP
///  R- P- S- R- P- .
const TEST_CELLS: [u8; 45] = [
    3, 7, 11, 3, 7, 11, 7, 11, 3, 255, 11, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 5, 1, 9, 221, 1, 9, 81, 9, 5, 1, 9, 5, 0,
];
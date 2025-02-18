use pijersi_rs::logic::Cells;

mod position;

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW S- R- SP
///  R- P- S- R- P- .
const TEST_CELLS: Cells = [
    3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 0, 0, 0, 0,
    0, 0, 0, 5, 1, 9, 221, 1, 9, 81, 9, 5, 1, 9, 5, 0,
];

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  pr
/// P- S- R- WW S- RP SP
///  R- P- S- R- .  .
const TEST_CELLS2: Cells = [
    3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 183, 5, 1, 9, 221, 1, 89, 81, 9, 5, 1, 9, 0, 0,
];

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
    12, 13, 14, 12, 0, 14, 13, 14, 12, 255, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 0, 0,
    0, 0, 0, 0, 0, 9, 8, 10, 187, 8, 10, 152, 10, 9, 8, 10, 9, 0,
];

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  pr
/// P- S- R- WW S- RP SP
///  R- P- S- R- .  .
const TEST_CELLS2: Cells = [12, 13, 14, 12, 0, 14, 13, 14, 12, 255, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 9, 8, 10, 187, 8, 154, 152, 10, 9, 8, 10, 0, 0];

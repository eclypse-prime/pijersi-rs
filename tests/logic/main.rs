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
const TEST_BOARD_STR: &str =
    "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..............P-S-R-WWS-R-SPR-P-S-R-P-..";

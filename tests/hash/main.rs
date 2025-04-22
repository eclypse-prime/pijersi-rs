mod position;

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

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  pr
/// P- S- R- WW S- RP SP
///  R- P- S- R- .  .
const TEST_BOARD_STR2: &str =
    "s-p-r-s-..r-p-r-s-ww..s-p-....................................prP-S-R-WWS-RPSPR-P-S-R-....";

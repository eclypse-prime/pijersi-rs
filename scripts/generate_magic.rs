fn possible_actions(mask_subset: u64, index: usize) -> u64 {
    let mut search_index = 0;
    let mut result = 0;
    let mut neighbours2 = NEIGHBOURS2[index];
    while neighbours2 != 0 {
        let shift = neighbours2.trailing_zeros();
        search_index += shift;
        if search_index >= 64 {
            break;
        }
        neighbours2 >>= shift + 1;
        if mask_subset.get((search_index as usize + index) / 2) {
            result |= 1 << search_index
        }
        search_index += 1;
    }
    result
}

fn try_magic(mask: u64, magic: u64, index: usize) -> Option<[u64; 64]> {
    let mut table: [u64; 64] = [u64::MAX; 64];
    let index_max = 1 << mask.count_ones();
    let mut subset: u64 = 0;
    for _ in 0..index_max {
        subset = subset.wrapping_sub(mask) & mask;
        let magic_hash = subset.wrapping_mul(magic);
        let magic_index = (magic_hash >> (64 - 6)) as usize;

        let actions = possible_actions(subset, index);

        let table_entry = table[magic_index];
        if table_entry == u64::MAX {
            table[magic_index] = actions;
        } else if table_entry != actions {
            return None;
        }
    }
    Some(table)
}

fn find_magic(index: usize) -> (u64, [u64; 64]) {
    println!("Starting search on index {index}");
    let mask = MOVE2_MASKS[index];
    loop {
        let magic = random::<u64>() & random::<u64>() & random::<u64>();
        if let Some(table) = try_magic(mask, magic, index) {
            println!("Step {index} completed");
            return (magic, table);
        }
    }
}

fn test_magic(index: usize, magic: u64, table: [u64; 64]) {
    let block_mask = MOVE2_MASKS[index];
    let index_max = 1 << block_mask.count_ones();
    let mut subset: u64 = 0;
    for _ in 0..index_max {
        subset = subset.wrapping_sub(block_mask) & block_mask;
        let magic_hash = subset.wrapping_mul(magic);
        let magic_index = (magic_hash >> (64 - 6)) as usize;

        assert_eq!(possible_actions(subset, index), table[magic_index]);
    }
}

fn main() {
    let magics: Vec<(u64, [u64; 64])> = (0..45).map(find_magic).collect();
    
    for i in 0..45 {
        test_magic(i, magics[i].0, magics[i].1);
    }
    
    println!("{magics:?}");
}
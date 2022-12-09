fn is_tree_visible(trees: &[Vec<u8>], row: usize, col: usize) -> bool {
    let height = trees[row][col];
    let mut is_visible = trees[row][..col].iter().all(|tree| *tree < height);
    is_visible = is_visible || trees[row][(col + 1)..].iter().all(|tree| *tree < height);
    is_visible = is_visible
        || trees
            .iter()
            .map(move |row| row.get(col))
            .take(row)
            .flatten()
            .all(|tree| *tree < height);
    is_visible = is_visible
        || trees
            .iter()
            .map(move |row| row.get(col))
            .skip(row + 1)
            .flatten()
            .all(|tree| *tree < height);
    is_visible
}

fn get_scenic_score(trees: &[Vec<u8>], row: usize, col: usize) -> u32 {
    let height = trees[row][col];
    let mut score = 1u32;
    score *= trees[row][..col]
        .iter()
        .rev()
        .enumerate()
        .find(|(_, tree)| **tree >= height)
        .map(|(i, _)| i as u32 + 1)
        .unwrap_or(col as u32);
    score *= trees[row][(col + 1)..]
        .iter()
        .enumerate()
        .find(|(_, tree)| **tree >= height)
        .map(|(i, _)| i as u32 + 1)
        .unwrap_or((trees[row].len() - col) as u32 - 1);
    score *= trees
        .iter()
        .map(move |row| row.get(col))
        .take(row)
        .flatten()
        .rev()
        .enumerate()
        .find(|(_, tree)| **tree >= height)
        .map(|(i, _)| i as u32 + 1)
        .unwrap_or(row as u32);
    score *= trees
        .iter()
        .map(move |row| row.get(col))
        .skip(row + 1)
        .flatten()
        .enumerate()
        .find(|(_, tree)| **tree >= height)
        .map(|(i, _)| i as u32 + 1)
        .unwrap_or((trees.len() - row) as u32 - 1);
    score
}

fn main() {
    let input_lines = util::get_input_lines();

    let grid = input_lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|tree| tree as u8 - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible = vec![vec![true; grid[0].len()]; grid.len()];
    for row in 1..(visible.len() - 1) {
        for col in 1..(visible[row].len() - 1) {
            visible[row][col] = is_tree_visible(&grid, row, col);
        }
    }
    println!(
        "Total trees visible: {}",
        visible
            .into_iter()
            .map(|row| row.into_iter().filter(|v| *v).count() as u32)
            .sum::<u32>()
    );

    let mut scores = vec![vec![0u32; grid[0].len()]; grid.len()];
    for row in 1..(scores.len() - 1) {
        for col in 1..(scores[row].len() - 1) {
            scores[row][col] = get_scenic_score(&grid, row, col);
        }
    }
    println!(
        "Highest scenic score: {}",
        scores
            .into_iter()
            .map(|row| row.into_iter().max().unwrap())
            .max()
            .unwrap()
    );
}

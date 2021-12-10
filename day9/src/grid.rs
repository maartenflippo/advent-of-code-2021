pub struct Grid {
    heightmap: Vec<Vec<u32>>,
}

impl Grid {
    pub fn new(lines: Vec<String>) -> Self {
        let heightmap = lines.into_iter()
            .map(|l| l.chars()
                .map(|c| c.to_digit(10).expect("Failed to get digit."))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { heightmap }
    }

    pub fn size_x(&self) -> usize {
        self.heightmap[0].len()
    }

    pub fn size_z(&self) -> usize {
        self.heightmap.len()
    }

    pub fn height_at(&self, row: usize, col: usize) -> u32 {
        if !self.in_grid(row as i32, col as i32) {
            return 9;
        }

        self.heightmap[row][col]
    }

    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        [(-1i32, 0i32), (1i32, 0i32), (0i32, -1i32), (0i32, 1i32)]
            .into_iter()
            .map(|(dr, dc)| (row as i32 + dr, col as i32 + dc))
            .filter(|&(row, col)| self.in_grid(row, col))
            .map(|(r, c)| (r as usize, c as usize))
            .collect::<Vec<_>>()
    }

    pub fn cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.size_z())
            .flat_map(|row| (0..self.size_x()).map(move |col| (row, col)))
    }

    fn in_grid(&self, row: i32, col: i32) -> bool {
        (0..self.size_x() as i32).contains(&col) &&
        (0..self.size_z() as i32).contains(&row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let grid = Grid::new(vec![
            "2199943210".to_owned(),
            "3987894921".to_owned(),
            "9856789892".to_owned(),
            "8767896789".to_owned(),
            "9899965678".to_owned(),
        ]);

        let (row, col) = (0, 0);
        let neighbors = grid.neighbors(row, col);

        assert_eq!(2, neighbors.len());
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(1, 0)));
    }
}
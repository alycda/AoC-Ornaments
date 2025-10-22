//! Spiral Matrix
//! 
//! https://exercism.org/tracks/rust/exercises/spiral-matrix
//! Reddit: r/dailyprogrammer [2017-06-19] Challenge #320 [Easy] Spiral Ascension

use aoc_ornaments::spatial::{Grid, Position};

fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let grid = Grid::square(size as usize, 0);
    let start = Position::ZERO;

    dbg!(&grid);

    for i in 0..size as usize {
        // starting at position step right until edge OR non-zero; then turn right
        
    };

    grid.to_vec()
}

fn main() -> miette::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, "empty spiral", vec![])]
    #[case(1, "trivial spiral", vec![vec![1]])]
    #[case(2, "2x2 spiral", vec![vec![1, 2], vec![4, 3]])]
    #[trace] //This attribute enable tracing
    fn test_spirals(#[case] input: u32, #[case] name: &str, #[case] expected: Vec<Vec<u32>>) {
        assert_eq!(spiral_matrix(input), expected);
    }

    #[test]
    #[ignore]
    fn empty_spiral() {
        let input = 0;
        let output = spiral_matrix(input);
        let expected: [[u32; 0]; 0] = [];
        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn trivial_spiral() {
        let input = 1;
        let output = spiral_matrix(input);
        let expected: [[u32; 1]; 1] = [[1]];
        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn spiral_of_size_2() {
        let input = 2;
        let output = spiral_matrix(input);
        let expected: [[u32; 2]; 2] = [[1, 2], [4, 3]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_3() {
        let input = 3;
        let output = spiral_matrix(input);
        let expected: [[u32; 3]; 3] = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_4() {
        let input = 4;
        let output = spiral_matrix(input);
        let expected: [[u32; 4]; 4] = [
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7],
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_5() {
        let input = 5;
        let output = spiral_matrix(input);
        let expected: [[u32; 5]; 5] = [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ];
        assert_eq!(output, expected);
    }

}
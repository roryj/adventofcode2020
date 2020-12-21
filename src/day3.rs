/// With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.

// Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:

// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#

// These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:

// ..##.........##.........##.........##.........##.........##.......  --->
// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........#.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...##....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

// You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

// The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:

// From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.

// The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:

// ..##.........##.........##.........##.........##.........##.......  --->
// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........X.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...#X....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

// In this example, traversing the map using this slope would cause you to encounter 7 trees.

// Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?

pub fn part1(input: &[String]) -> u32 {
    // first we need to parse the input into an array of arrays
    let tree_map: Vec<Vec<bool>> = input.iter().map(|line| parse_line(line)).collect();

    // println!("map");
    // let mut index: usize = 0;
    // for line in input.iter() {
    //     println!("{:?} | {:?}", line, tree_map[index]);
    //     index += 1;
    // }

    calculate_trees_hit(&tree_map, 3, 1)
}

pub fn part2(input: &[String]) -> u32 {
    // first we need to parse the input into an array of arrays
    let tree_map: Vec<Vec<bool>> = input.iter().map(|line| parse_line(line)).collect();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|(x, y)| calculate_trees_hit(&tree_map, x.clone() as usize, y.clone() as usize))
        .product()
}

fn calculate_trees_hit(grid: &Vec<Vec<bool>>, x_slope: usize, y_slope: usize) -> u32 {
    assert!(grid.len() >= 1);
    let height = grid.len();
    let width = grid[0].len();
    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;

    while y < height {
        // check to see if we are at a tree
        if grid[y][x] {
            tree_count += 1;
        }

        // now lets get the new slope

        x = (x + x_slope) % width;
        y += y_slope;
    }

    tree_count
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("illegale input"),
        })
        .collect()
}

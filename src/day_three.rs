pub fn run(input: String) {
    let mut field: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for character in line.chars() {
            row.push(character);
        }
        field.push(row);
    }
    let field = field;

    part1(&field);
    part2(&field);
}

struct Position {
    x: usize,
    y: usize,
}

struct Pattern {
    right: usize,
    down: usize,
}

fn part1(field: &Vec<Vec<char>>) {
    let row_length = field[0].len();
    let mut tree_count = 0;
    let mut position = Position { x: 0, y: 0 };

    while position.y < field.len() {
        if field[position.y][position.x % row_length] == '#' {
            tree_count += 1;
        }
        position.x += 3;
        position.y += 1;
    }

    println!("Day 3 Part 1: {}", tree_count);
}

fn part2(field: &Vec<Vec<char>>) {
    let row_length = field[0].len();
    let mut multiplier = 1;
    let patterns: [Pattern; 5] = [
        Pattern { right: 1, down: 1 },
        Pattern { right: 3, down: 1 },
        Pattern { right: 5, down: 1 },
        Pattern { right: 7, down: 1 },
        Pattern { right: 1, down: 2 },
    ];

    for pattern in patterns.iter() {
        let mut tree_count: usize = 0;
        let mut position = Position { x: 0, y: 0 };

        while position.y < field.len() {
            if field[position.y][position.x % row_length] == '#' {
                tree_count += 1;
            }
            position.x += pattern.right;
            position.y += pattern.down;
        }
        multiplier *= tree_count;
    }

    println!("Day 3 Part 2: {}", multiplier);
}

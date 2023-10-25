pub trait VecExt<T>: AsMut<Vec<T>> {
    fn prepend_vec(&mut self, other: &mut Vec<T>) {
        self.as_mut().splice(0..0, other.drain(..));
    }

    fn preppend(&mut self, other: T) {
        self.as_mut().splice(0..0, vec![other]);
    }

    fn insert_after(&mut self, index: usize, other: &mut Vec<T>) {
        self.as_mut().splice(index..index, other.drain(..));
    }

    fn insert_from_slice(&mut self, index: usize, other: &[T])
    where
        T: Clone,
    {
        self.as_mut()
            .splice(index..index, other.into_iter().cloned());
    }
}

impl<T> VecExt<T> for Vec<T> {}

fn move_up(grid: &mut Vec<Vec<char>>, pos: &mut (i32, i32)) {
    // if there is no space above position
    if pos.1 as usize == grid.len() - 1 {
        // creation of new row with only ' '
        // map has no index so i can't replace ' ' to '*' at position index
        let mut new_row = grid
            .clone()
            .into_iter()
            .nth(0)
            .unwrap()
            .into_iter()
            .map(|mut x| {
                x = ' ';
                return x;
            })
            .collect::<Vec<char>>();

        // replace new row index at pos.0 to '*' and assign gotten new valut to nothing
        let _ = std::mem::replace(&mut new_row[pos.0 as usize], '*');

        grid.preppend(new_row);
    } else {
        // if there is space above just replace element above position
        let length = grid.len();
        let _ = std::mem::replace(&mut grid[length - pos.1 as usize - 2][pos.0 as usize], '*');
    }

    pos.1 += 1;
}

fn move_down(grid: &mut Vec<Vec<char>>, pos: &mut (i32, i32)) {
    // if there is no space below position
    if pos.1 as usize == 0 {
        // creation of new row with only ' '
        // map has no index so i can't replace ' ' to '*' at position index
        let mut new_row = grid
            .clone()
            .into_iter()
            .nth(0)
            .unwrap()
            .into_iter()
            .map(|mut x| {
                x = ' ';
                return x;
            })
            .collect::<Vec<char>>();

        // replace new row index at pos.0 to '*' and assign gotten new valut to nothing
        let _ = std::mem::replace(&mut new_row[pos.0 as usize], '*');

        grid.push(new_row);
    } else {
        // if there is space below just replace element above position
        let length = grid.len();
        let _ = std::mem::replace(&mut grid[length - pos.1 as usize][pos.0 as usize], '*');
        pos.1 -= 1;
    }
}

fn move_right(grid: &mut Vec<Vec<char>>, pos: &mut (i32, i32)) {
    // if position at x is on right with no space to right append to all vectors ' '
    // only at position add '*'
    if pos.0 as usize == grid.iter().nth(0).unwrap().len() - 1 {
        for (index, row) in grid.into_iter().rev().enumerate() {
            if pos.1 as usize == index {
                row.push('*');
            } else {
                row.push(' ');
            }
        }
    } else {
        let length = grid.len();
        let _ = std::mem::replace(
            &mut grid[length - pos.1 as usize - 1][pos.0 as usize + 1],
            '*',
        );
    }
    pos.0 += 1
}

fn print_grid(grid: &mut Vec<Vec<char>>) {
    for row in grid.iter() {
        println!("{row:?}");
    }
}
fn move_left(grid: &mut Vec<Vec<char>>, pos: &mut (i32, i32)) {
    if pos.0 == 0 {
        for (i, row) in grid.into_iter().rev().enumerate() {
            if i == pos.1 as usize {
                row.preppend('*');
            } else {
                row.preppend(' ');
            }
        }
    } else {
        let length = grid.len();
        let _ = std::mem::replace(
            &mut grid[length - pos.1 as usize - 1][pos.0 as usize - 1],
            '*',
        );
        pos.0 -= 1;
    }
}

fn handle_movement(
    letter: &char,
    direction: &mut char,
    grid: &mut Vec<Vec<char>>,
    pos: &mut (i32, i32),
) {
    match letter {
        'F' => match direction {
            'U' => move_up(grid, pos),
            'D' => move_down(grid, pos),
            'R' => move_right(grid, pos),
            'L' => move_left(grid, pos),
            _ => {
                println!("direction messed up");
            }
        },
        'L' => match direction {
            'U' => {
                *direction = 'L';
            }
            'D' => {
                *direction = 'R';
            }
            'R' => {
                *direction = 'U';
            }
            'L' => {
                *direction = 'D';
            }
            _ => {
                println!("direction messed up");
            }
        },
        'R' => match direction {
            'U' => {
                *direction = 'R';
            }
            'D' => {
                *direction = 'L';
            }
            'R' => {
                *direction = 'D';
            }
            'L' => {
                *direction = 'U';
            }
            _ => {
                println!("direction messed up");
            }
        },
        sth => {
            println!("character no supported {sth}");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    println!("{command}");
    let mut grid: Vec<Vec<char>> = vec![vec!['*']];
    let mut pos = (0, 0);
    let mut direction = 'R';

    let re = regex::Regex::new(r"([A-Z]\d*)").unwrap();

    let result: Vec<&str> = re.find_iter(command).map(|mat| mat.as_str()).collect();

    for window in result {
        let (com, num) = window.split_at(1);
        println!("{}{}", com, num);
        if com.parse::<usize>().is_err() {
            if let Ok(repeat) = num.parse::<usize>() {
                for _ in 0..repeat {
                    handle_movement(
                        &com.chars().next().unwrap(),
                        &mut direction,
                        &mut grid,
                        &mut pos,
                    );
                }
            } else {
                handle_movement(
                    &com.chars().next().unwrap(),
                    &mut direction,
                    &mut grid,
                    &mut pos,
                );
            }
        }
        // handle_movement(&letter, &mut direction, &mut grid, &mut pos);
        print_grid(&mut grid);
    }
    let mut finnal_str = grid
        .into_iter()
        .map(|x| format!["{}{}{}", x.into_iter().collect::<String>(), '\r', '\n'])
        .collect::<Vec<String>>()
        .join("");
    finnal_str.pop();
    finnal_str.pop();
    let finnal_str = finnal_str.as_str();
    println!("{finnal_str:?}");
}

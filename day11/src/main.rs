use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn in_bounds(matrix: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    return y >= 0 && y < matrix.len() && x >= 0 && x < matrix[y].len();
}

fn add_then_check_flash(
    matrix: &mut Vec<Vec<usize>>,
    to_flash: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
) {
    if in_bounds(&matrix, x, y) && matrix[y][x] != 0 {
        matrix[y][x] += 1;
        if matrix[y][x] > 9 && !to_flash.contains(&(x, y)) {
            to_flash.push((x, y));
        }
    }
}

fn all_flashed(matrix: &Vec<Vec<usize>>) -> bool {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] != 0 {
                return false;
            }
        }
    }
    return true;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut game: Vec<Vec<usize>> = Vec::new();

    for line in lines_vec {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            let num = c.to_string().parse::<usize>().unwrap();
            row.push(num);
        }
        game.push(row);
    }

    let mut flashes = 0;
    let mut count = 0;
    loop {
        let mut to_flash: Vec<(usize, usize)> = Vec::new();

        // increment
        for y in 0..game.len() {
            for x in 0..game[y].len() {
                game[y][x] += 1;

                if game[y][x] > 9 && !to_flash.contains(&(x, y)) {
                    to_flash.push((x, y));
                }
            }
        }

        // calculate flashes
        while !to_flash.is_empty() {
            let (x, y) = to_flash.pop().unwrap();

            if game[y][x] != 0 {
                game[y][x] = 0;
                flashes += 1;

                if x > 0 {
                    add_then_check_flash(&mut game, &mut to_flash, x - 1, y);
                    add_then_check_flash(&mut game, &mut to_flash, x - 1, y + 1);
                }

                if y > 0 {
                    add_then_check_flash(&mut game, &mut to_flash, x, y - 1);
                    add_then_check_flash(&mut game, &mut to_flash, x + 1, y - 1);
                }

                if x > 0 && y > 0 {
                    add_then_check_flash(&mut game, &mut to_flash, x - 1, y - 1);
                }

                add_then_check_flash(&mut game, &mut to_flash, x + 1, y);
                add_then_check_flash(&mut game, &mut to_flash, x, y + 1);
                add_then_check_flash(&mut game, &mut to_flash, x + 1, y + 1);
            }
        }

        count += 1;

        if count == 100 {
            println!("flashes @ 100: {}", flashes);
        }

        if all_flashed(&game) {
            println!("all flashed at: {}", count);
            break;
        }
    }
    Ok(())
}

use std::{error::Error, fs::File, io::BufRead, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/3")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for result in reader.lines() {
        let line = result?;
        grid.push(line.chars().map(|c| c == '#').collect());
    }

    let num_trees = count_trees(&grid, 1, 1)
        * count_trees(&grid, 3, 1)
        * count_trees(&grid, 5, 1)
        * count_trees(&grid, 7, 1)
        * count_trees(&grid, 1, 2);

    println!("{}", num_trees);

    Ok(())
}

fn count_trees(grid: &Vec<Vec<bool>>, x_step: usize, y_step: usize) -> usize {
    let mut x = 0 as usize;
    let mut y = 0 as usize;
    let mut num_trees = 0;

    loop {
        x = (x + x_step) % grid.first().unwrap().len();
        y += y_step;

        if y >= grid.len() {
            break;
        }

        if *grid.get(y).unwrap().get(x).unwrap() {
            num_trees += 1;
        }
    }

    num_trees
}

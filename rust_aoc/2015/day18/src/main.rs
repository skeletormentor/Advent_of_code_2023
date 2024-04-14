#[derive(Debug, Copy, Clone)]
struct Light(bool);

#[derive(Debug, Clone)]
struct Grid {
    lightmap: Vec<Vec<Light>>,
    rows: usize,
    cols: usize,
}
#[derive(Debug)]
struct GridErr;

impl Grid {
    fn count_surroundings(&self, row: usize, col: usize) -> usize {
        let mut count: usize = 0;
        let row = row as isize;
        let col = col as isize;
        let surr: Vec<(isize, isize)> = vec![
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ];
        for (r, c) in surr {
            if let Some(is_on) = self.get(r, c) {
                if is_on {
                    count += 1;
                }
            }
        }
        count
    }

    fn get(&self, r: isize, c: isize) -> Option<bool> {
        if r < 0 || r >= self.rows as isize || c < 0 || c >= self.cols as isize {
            None
        } else {
            Some(self.lightmap[r as usize][c as usize].0)
        }
    }

    fn run(&mut self) {
        let mut lightmap = vec![];
        for r in 0..self.rows {
            let mut lights = vec![];
            for c in 0..self.cols {
                let surr_count = self.count_surroundings(r, c);
                let current = self.lightmap[r][c];
                if current.0 {
                    match surr_count {
                        2 | 3 => lights.push(Light(true)),
                        _ => lights.push(Light(false)),
                    }
                } else {
                    match surr_count {
                        3 => lights.push(Light(true)),
                        _ => lights.push(current),
                    }
                }
            }
            lightmap.push(lights);
        }
        *self = Self {
            lightmap,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn run_n(&mut self, n: usize, part2: bool) -> usize {
        if part2 {
            self.put_corners_on();
        }
        for _ in 0..n {
            self.run();
            if part2 {
                self.put_corners_on();
            }
        }
        self.count_lights()
    }

    fn count_lights(&self) -> usize {
        self.lightmap
            .iter()
            .flatten()
            .filter(|light| light.0)
            .count()
    }

    fn put_corners_on(&mut self) {
        self.lightmap[0][0] = Light(true);
        self.lightmap[0][self.cols - 1] = Light(true);
        self.lightmap[self.rows - 1][0] = Light(true);
        self.lightmap[self.rows - 1][self.cols - 1] = Light(true);
    }
}

impl std::str::FromStr for Grid {
    type Err = GridErr;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().len();
        let mut lightmap = vec![];
        for line in input.lines() {
            let mut lights = vec![];
            for char in line.chars() {
                let light = Light(char == '#');
                lights.push(light);
            }
            lightmap.push(lights);
        }
        Ok(Self {
            lightmap,
            rows,
            cols,
        })
    }
}

fn main() {
    let mut grid = include_str!("../src/input.txt").parse::<Grid>().unwrap();
    let mut grid2 = grid.clone();
    println!("{}", grid.run_n(100, false));
    println!("{}", grid2.run_n(100, true));
}

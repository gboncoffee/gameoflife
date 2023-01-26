use ncurses as n;

#[derive(Clone)]
struct Universe {
    cells: Vec<Vec<bool>>,
}

impl Universe {
    fn new(y: usize, x: usize) -> Universe {
        let mut cells = Vec::with_capacity(y);
        for _ in 0..x {
            cells.push(vec![false; x]);
        }

        Universe { cells }
    }

    fn draw(&self) {
        for (y, line) in self.cells.iter().enumerate() {
            n::mv(y as i32, 0);
            n::clrtoeol();
            for (x, cell) in line.iter().enumerate() {
                if *cell {
                    n::addch('#' as n::chtype);
                } else {
                    n::mv(y as i32, x as i32 + 1);
                }
            }
            n::mv(y as i32 + 1, 0);
        }

        n::refresh();
    }
}

fn get_game_input(win: n::WINDOW) -> Universe {
    let mut universe = Universe::new(n::getmaxy(win) as usize, n::getmaxx(win) as usize);

    loop {
        let s = n::keyname(n::getch()).unwrap();

        if s == "^M" {
            break;
        } else if s == " " {
            let x = n::getcurx(win) as usize;
            let y = n::getcury(win) as usize;
            universe.cells[y][x] = !universe.cells[y][x];
            if universe.cells[y][x] {
                n::addch('#' as n::chtype);
            } else {
                n::addch(' ' as n::chtype);
            }
            n::mv(y as i32, x as i32);
        } else if s == "H" || s == "h" || s == "KEY_LEFT" {
            n::mv(n::getcury(win), n::getcurx(win) - 1);
        } else if s == "J" || s == "j" || s == "KEY_DOWN" {
            n::mv(n::getcury(win) + 1, n::getcurx(win));
        } else if s == "K" || s == "k" || s == "KEY_UP" {
            n::mv(n::getcury(win) - 1, n::getcurx(win));
        } else if s == "L" || s == "l" || s == "KEY_RIGHT" {
            n::mv(n::getcury(win), n::getcurx(win) + 1);
        }
    }

    universe
}

fn get_neighbours(universe: &Universe, y: isize, x: isize) -> Vec<bool> {
    // get neighbours
    let mut neighbours = Vec::new();
    let neigh_coordinates = vec![
        // line before
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        // same line
        (y, x - 1),
        (y, x + 1),
        // next line
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ];
    for (a, b) in neigh_coordinates {
        if a >= 0 && b >= 0 {
            if let Some(line) = universe.cells.get(a as usize) {
                if let Some(neighbour) = line.get(b as usize) {
                    neighbours.push(*neighbour);
                }
            }
        }
    }

    neighbours
}

fn game_of_life(input: Universe) -> Universe {
    let mut universe = input.clone();

    for (y, line) in input.cells.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let neighbours = get_neighbours(&input, y as isize, x as isize);

            // calculate what to do
            let mut n_neighs_live = 0;
            for neighbour in neighbours {
                if neighbour {
                    n_neighs_live += 1;
                }
            }
            if *cell {
                if n_neighs_live != 2 && n_neighs_live != 3 {
                    universe.cells[y][x] = false;
                }
            } else {
                if n_neighs_live == 3 {
                    universe.cells[y][x] = true;
                }
            }
        }
    }

    universe
}

fn main() {
    let win = n::initscr();
    n::noecho();
    n::nonl();
    n::intrflush(win, false);
    n::keypad(win, true);

    let mut universe = get_game_input(win);

    n::curs_set(n::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    n::timeout(500);

    loop {
        let new_game = game_of_life(universe);
        universe = new_game;

        universe.draw();

        if n::getch() == 'q' as i32 {
            break;
        }
    }

    n::endwin();
}

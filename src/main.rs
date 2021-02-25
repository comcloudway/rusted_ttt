use rand::Rng;
use std::io;

// APP RELATED
const NAME: &str = "Rusted Tick-Tack-Toe"; // Software name

// GAME RELATED
const BOARD_SIZE: usize = 3; // Board Dimensions -> square
const IOR: usize = 3; // items in row required to win match
const GAME_MODE_COMPUTER: bool = true; // false for local multiplayer
const MODE_COMPUTER_RANDOM: bool = false; // false for intelligent computer
const MODE_COMPUTER_PASSIVE: bool = true; // prevent opponents from getting IOR in row = true || false = try to get IOR in row

// UI RELATED
const COLUMN_IDS: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
const ROW_IDS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const PLAYER_HEAD: &[char; 4] = &['.', 'x', 'o', '#'];
const CURSOR: &[char; 2] = &['[', ']'];

enum BoxState {
    Unflagged,
    Flagged(usize),
    Unobtainable
}
impl BoxState {
    fn to_char(&self) -> char {
        let res: char = match self {
            BoxState::Unflagged => PLAYER_HEAD[0],
            BoxState::Flagged(i) => PLAYER_HEAD[if i == &1 { 1 } else { 2 }],
            BoxState::Unobtainable => PLAYER_HEAD[3]
        };
        return res;
    }
}

enum FSICA {
    NotFound,
    Index(usize),
}
impl FSICA {
    fn from(array: &[char], c: &str) -> FSICA {
        let mut spot: FSICA = FSICA::NotFound;
        for (index, &item) in array.iter().enumerate() {
            if item.to_string() == c.to_string() {
                spot = FSICA::Index(index);
            }
        }
        return spot;
    }
}

pub fn main() {
    let mut board = [[&BoxState::Unflagged; BOARD_SIZE]; BOARD_SIZE];
    let result = game(&mut board);
    if result == 10 {
        println!("PLAYER {} WON", result / 10);
    } else if result == 20 {
        println!("PLAYER {} WON", result / 10);
    } else if result == 30 {
        println!("NO WINNER");
    }
}

fn game(board: &mut [[&BoxState; BOARD_SIZE]; BOARD_SIZE]) -> usize {
    let mut pointer: [usize; 2] = [0; 2];
    let mut player: usize = 1;

    let result: usize = loop {
        // clear screen
        print!("{}[2J", 27 as char);

        let mut ui = String::from("\n     ");
        ui.push_str(NAME);
        ui.push_str("\nPLAYER: ");
        ui.push_str(&player.to_string());
        ui.push_str("\n\n\n");

        // COLUMN NAMES
        ui.push_str(&String::from("      "));
        for c in 0..BOARD_SIZE {
            ui = ui + &String::from("   ") + &COLUMN_IDS[c].to_string()
        }
        ui.push_str(&String::from("\n\n"));

        // ROWS
        for (r, &row) in board.iter().enumerate() {
            let mut line = String::from("\n   ");
            // ROW NAMES
            line = line + &ROW_IDS[r].to_string() + &"   ".to_string();

            // ROW ITEMS
            for (i, &item) in row.iter().enumerate() {
                let end_tag = String::from(if pointer[0] == r && pointer[1] == i {
                    CURSOR[1]
                } else {
                    ' '
                });
                let start_tag = String::from(if pointer[0] == r && pointer[1] == i {
                    CURSOR[0]
                } else {
                    ' '
                });

                line =
                    line + &" ".to_string() + &start_tag + &item.to_char().to_string() + &end_tag;
            }
            line.push_str(&String::from("   \n"));
            ui.push_str(&line);
        }
        ui.push_str(&String::from("\n\n"));
        ui.push_str("Enter Command [e.g. q, s, c, C1, B6]");

        // PRINT UI
        println!("{}", ui);

        // GET USER INPUT
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        command = command.trim().to_string();

        if command.len() == 2 {
            // NAVIGATE
            let col = &command[..1];
            let row = &command[1..];

            let row_index = match FSICA::from(&COLUMN_IDS, &col) {
                FSICA::NotFound => continue,
                FSICA::Index(i) => i,
            };
            let column_index = match FSICA::from(&ROW_IDS, &row) {
                FSICA::NotFound => continue,
                FSICA::Index(i) => i,
            };

            // TODO Pointer x & y seem to be swapped
            pointer = [column_index, row_index];
        } else if command == "q" {
            // QUIT
            break 0;
        } else if command == "c" {
            // TOGGLE CHECK SQUARE
            match board[pointer[0]][pointer[1]] {
                BoxState::Unflagged => (),
                BoxState::Unobtainable => continue,
                BoxState::Flagged(_) => continue,
            };

            if player == 1 {
                board[pointer[0]][pointer[1]] = match board[pointer[0]][pointer[1]] {
                    BoxState::Unflagged => &BoxState::Flagged(1),
                    BoxState::Flagged(_i) => &BoxState::Unflagged,
                    BoxState::Unobtainable => &BoxState::Unobtainable,
                }
            } else {
                board[pointer[0]][pointer[1]] = match board[pointer[0]][pointer[1]] {
                    BoxState::Unflagged => &BoxState::Flagged(2),
                    BoxState::Flagged(_i) => &BoxState::Unflagged,
                    BoxState::Unobtainable => &BoxState::Unobtainable,
                }
            };
            //} else if command == "n" {
            // NEXT
            fn check_win(board: [[&BoxState; BOARD_SIZE]; BOARD_SIZE], player: usize) -> usize {
                let mut empty = 0;
                let mut flagged_p1: [[&BoxState; BOARD_SIZE]; BOARD_SIZE] =
                    [[&BoxState::Unflagged; BOARD_SIZE]; BOARD_SIZE];
                let mut flagged_p2: [[&BoxState; BOARD_SIZE]; BOARD_SIZE] =
                    [[&BoxState::Unflagged; BOARD_SIZE]; BOARD_SIZE];

                // Seperate board
                for (r, &row) in board.iter().enumerate() {
                    for (c, &column) in row.iter().enumerate() {
                        match column {
                            BoxState::Unflagged => empty = empty + 1,
                            BoxState::Unobtainable => {
                                flagged_p1[r][c] = &BoxState::Unobtainable;
                                flagged_p2[r][c] = &BoxState::Unobtainable;
                            },
                            BoxState::Flagged(i) => {
                                if i == &1 {
                                    flagged_p1[r][c] = &BoxState::Flagged(1);
                                } else {
                                    flagged_p2[r][c] = &BoxState::Flagged(2);
                                }
                            }
                        };
                    }
                }

                fn find_arrangement(cb: [[&BoxState; BOARD_SIZE]; BOARD_SIZE]) -> usize {
                    fn scan(
                        mb: [[&BoxState; BOARD_SIZE]; BOARD_SIZE],
                        coords: &[[usize; 2]],
                    ) -> usize {
                        let mut chain: usize = 0;
                        for (_c, &coord) in coords.iter().enumerate() {
                            if coord[0] == 0 || coord[1] == 0 {
                                continue;
                            }
                            match mb[coord[0] - 1][coord[1] - 1] {
                                BoxState::Unflagged => {chain=0; continue},
                                BoxState::Flagged(_) => chain = chain + 1,
                                BoxState::Unobtainable => {chain=0; continue}
                            };
                        }
                        return chain;
                    }

                    let mut longest: usize = 0;

                    for (r, &row) in cb.iter().enumerate() {
                        for (c, _column) in row.iter().enumerate() {
                            match cb[r][c] {
                                BoxState::Unflagged => (),
                                BoxState::Unobtainable => (),
                                BoxState::Flagged(_) => {
                                    let mut dir: [usize; 4] = [0; 4];

                                    if c > 0 {
                                        // left r->l
                                        let mut builder: [[usize; 2]; BOARD_SIZE] =
                                            [[0; 2]; BOARD_SIZE];
                                        for col in 0..c+1 {
                                            builder[col] = [r + 1, col + 1];
                                        }
                                        dir[0] = scan(cb, &builder);
                                    };
                                    if r > 0 && c > 0 {
                                        // top left br->tl
                                        let mut builder: [[usize; 2]; BOARD_SIZE] =
                                            [[0; 2]; BOARD_SIZE];
                                        let d = if r < c { c } else { r };
                                        for li in 0..d + 1 {
                                            if r >= li && c >= li {
                                                builder[li] = [r - li + 1, c - li + 1];
                                            };
                                        }
                                        dir[1] = scan(cb, &builder);
                                    };
                                    if r < BOARD_SIZE - 1 {
                                        // bottom top->bottom
                                        let mut builder: [[usize; 2]; BOARD_SIZE] =
                                            [[0; 2]; BOARD_SIZE];
                                        for ro in r..BOARD_SIZE {
                                            builder[ro] = [ro + 1, c + 1];
                                        }
                                        dir[2] = scan(cb, &builder);
                                    };
                                    if r < BOARD_SIZE - 1 && c > 0 {
                                        // bottom left tr->bl
                                        let mut builder: [[usize; 2]; BOARD_SIZE] =
                                            [[0; 2]; BOARD_SIZE];
                                        let d = if r < c { c } else { r };
                                        for li in 0..d + 1 {
                                            if r + li < BOARD_SIZE && c >= li {
                                                builder[li] = [r + li + 1, c - li + 1];
                                            };
                                        }
                                        dir[3] = scan(cb, &builder);
                                    };

                                    // find biggest chain
                                    for (_c, &chain) in dir.iter().enumerate() {
                                        if chain > longest {
                                            longest = chain;
                                        };
                                    }
                                }
                            };
                        }
                    }

                    return longest;
                }

                let stats: usize =
                    find_arrangement(if player == 1 { flagged_p1 } else { flagged_p2 });
                if stats >= IOR {
                    if player == 1 {
                        return 1;
                    } else {
                        return 2;
                    }
                }

                // if board ist filled but nobpdy won
                if empty == 0 {
                    return 3;
                }

                // nobody won, but game hasent ended yet
                return 0;
            }
            // CHECK FOR WINNER
            let wn = check_win(*board, player);
            if wn != 0 {
                break wn * 10;
            }

            player = if player == 1 { 2 } else { 1 };
            // GENERATE COMPUTER MOVE / ALLOW SECOND PLAYER TO MOVE
            if GAME_MODE_COMPUTER == true {
                if MODE_COMPUTER_RANDOM == true {
                    // COMPUTER GENERATES MOVES RANDOMLY
                    // GENERATE
                    fn random_field(
                        board: [[&BoxState; BOARD_SIZE]; BOARD_SIZE],
                        counter: &mut usize,
                    ) -> (usize, usize) {
                        if counter <= &mut 1 {
                            return (0, 0);
                        }
                        let row = rand::thread_rng().gen_range(0, BOARD_SIZE);
                        let column = rand::thread_rng().gen_range(0, BOARD_SIZE);
                        let res = match board[row][column] {
                            BoxState::Unflagged => (row, column),
                            BoxState::Unobtainable => (row, column),
                            BoxState::Flagged(_) => random_field(board, counter),
                        };
                        *counter = *counter - 1;
                        return res;
                    }

                    let mut count: usize = BOARD_SIZE * BOARD_SIZE;
                    let (row, column) = random_field(*board, &mut count);
                    board[row][column] = &BoxState::Flagged(2);
                    println!(
                        "Player {} selected Field {}{}",
                        player, COLUMN_IDS[column], ROW_IDS[row]
                    );
                    // CHECK FOR WINNER
                    let wn = check_win(*board, player);
                    if wn != 0 {
                        break wn * 10;
                    }
                    // next player
                    player = 1;
                } else {
                    // COMPUTER TRIES TO FIND GOOD SPOTS
                    let mut priority_high: Vec<[usize; 2]> = Vec::new();
                    let mut priority_medium: Vec<[usize; 2]> = Vec::new();
                    let mut priority_low: Vec<[usize; 2]> = Vec::new();

                    // GENERATE PRORITY VECTORS

                    let mut flagged_p1: [[&BoxState; BOARD_SIZE]; BOARD_SIZE] =
                        [[&BoxState::Unflagged; BOARD_SIZE]; BOARD_SIZE];
                    let mut flagged_p2: [[&BoxState; BOARD_SIZE]; BOARD_SIZE] =
                        [[&BoxState::Unflagged; BOARD_SIZE]; BOARD_SIZE];

                    // Seperate board
                    for (r, &row) in board.iter().enumerate() {
                        for (c, &column) in row.iter().enumerate() {
                            match column {
                                BoxState::Unflagged => (),
                                BoxState::Flagged(i) => {
                                    if i == &1 {
                                        flagged_p1[r][c] = &BoxState::Flagged(1);
                                        flagged_p2[r][c] = &BoxState::Unobtainable;
                                    } else {
                                        flagged_p2[r][c] = &BoxState::Flagged(2);
                                        flagged_p1[r][c] = &BoxState::Unobtainable;
                                    }
                                },
                                BoxState::Unobtainable => ()
                            };
                        }
                    }

                    fn find_arrangement(
                        cb: [[&BoxState; BOARD_SIZE]; BOARD_SIZE],
                        high: &mut Vec<[usize; 2]>,
                        med: &mut Vec<[usize; 2]>,
                        low: &mut Vec<[usize; 2]>,
                    ) {

                        for (r, &row) in cb.iter().enumerate() {
                            for (c, _column) in row.iter().enumerate() {
                                match cb[r][c] {
                                    BoxState::Unflagged => (),
                                    BoxState::Unobtainable => (),
                                    BoxState::Flagged(_) => {
                                        // RIGHT -> LEFT
                                        if c > 0 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            for col in 0..c + 1 {
                                                coords[col] = [r + 1, col + 1];
                                            }
                                            // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
                                         // RIGHT <- LEFT
                                        if c > 0 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            for col in c..BOARD_SIZE {
                                                coords[col] = [r + 1, col + 1];
                                            }
                                            // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
                                         // BOTTOM RIGHT <- TOP LEFT
                                        if r > 0 && c > 0 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            let d = if r < c { c } else { r };
                                            for li in d..BOARD_SIZE {
                                                if r >= li && c >= li {
                                                    coords[li] = [r - li + 1, c - li + 1];
                                                };
                                            }
                                                // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
                                        // BOTTOM RIGHT -> TOP LEFT
                                        if r > 0 && c > 0 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            let d = if r < c { c } else { r };
                                            for li in 0..d + 1 {
                                                if r >= li && c >= li {
                                                    coords[li] = [r - li + 1, c - li + 1];
                                                };
                                            }
                                                // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
// TOP -> BOTTOM
                                        if r < BOARD_SIZE - 1 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            for ro in r..0 {
                                            coords[r] = [ro + 1, c + 1];
                                            }
    // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
                                        // TOP <- BOTTOM
                                        if r < BOARD_SIZE - 1 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            for ro in r..BOARD_SIZE {
                                            coords[r] = [ro + 1, c + 1];
                                            }
    // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
                                        // TOP RIGHT <- BOTTOM LEFT
                                        if r < BOARD_SIZE - 1 && c > 0 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            let d = if r < c { c } else { r };
                                            for li in d..BOARD_SIZE {
                                                if r + li < BOARD_SIZE && c >= li {
                                                    coords[li] = [r + li + 1, c - li + 1];
                                                };
                                            }
                                                // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
                                        // TOP RIGHT -> BOTTOM LEFT
                                        if r < BOARD_SIZE - 1 && c > 0 {
                                            let mut coords: [[usize; 2]; BOARD_SIZE] =
                                                [[0; 2]; BOARD_SIZE];
                                            let d = if r < c { c } else { r };
                                            for li in 0..d + 1 {
                                                if r + li < BOARD_SIZE && c >= li {
                                                    coords[li] = [r + li + 1, c - li + 1];
                                                };
                                            }
                                                // do the checking
                                            let mut chain: usize = 0;
                                            for (_c, &coord) in coords.iter().enumerate() {
                                                if coord[0] == 0 || coord[1] == 0 {
                                                    continue;
                                                }
                                                match cb[coord[0] - 1][coord[1] - 1] {
                                                    BoxState::Unflagged => {
                                                        if chain >= IOR - 1 {
                                                            // hp
                                                            high.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 2 {
                                                            // mp
                                                            med.push([coord[0] - 1, coord[1] - 1]);
                                                        } else if chain >= IOR - 3 {
                                                            // lp
                                                            low.push([coord[0] - 1, coord[1] - 1]);
                                                        }
                                                    }
                                                    BoxState::Flagged(_) => chain = chain + 1,
                                                    BoxState::Unobtainable => {chain=0; continue}
                                                };
                                            }
                                        };
                                    }
                                };
                            }
                        }
                    }

                    find_arrangement(
                        if MODE_COMPUTER_PASSIVE == true {
                            flagged_p1
                        } else {
                            flagged_p2
                        },
                        &mut priority_high,
                        &mut priority_medium,
                        &mut priority_low,
                    );
                      find_arrangement(
                          if MODE_COMPUTER_PASSIVE == true {
                              flagged_p2
                          } else {
                              flagged_p1
                          },
                          &mut priority_high,
                          &mut priority_medium,
                          &mut priority_low,
                      );

                    // MATCH VALUES
                    let pos_high = rand::thread_rng().gen_range(0, priority_high.len()+1);
                    let pos_med = rand::thread_rng().gen_range(0, priority_medium.len()+1);
                    let pos_low = rand::thread_rng().gen_range(0, priority_low.len()+1);

                    let coords: [usize; 2] = match priority_high.get(pos_high) {
                        Some(v) => *v,
                        None => match priority_medium.get(pos_med) {
                            Some(v) => *v,
                            None => match priority_low.get(pos_low) {
                                Some(v) => *v,
                                None => loop {
                                    // fallback to random
                                    fn random_field(
                                        board: [[&BoxState; BOARD_SIZE]; BOARD_SIZE],
                                        counter: &mut usize,
                                    ) -> (usize, usize) {
                                        if counter <= &mut 1 {
                                            return (0, 0);
                                        }
                                        let row = rand::thread_rng().gen_range(0, BOARD_SIZE);
                                        let column = rand::thread_rng().gen_range(0, BOARD_SIZE);
                                        let res = match board[row][column] {
                                            BoxState::Unflagged => (row, column),
                                            BoxState::Flagged(_) => random_field(board, counter),
                                            BoxState::Unobtainable => random_field(board, counter),
                                        };
                                        *counter = *counter - 1;
                                        return res;
                                    }

                                    let mut count: usize = BOARD_SIZE * BOARD_SIZE;
                                    let (row, column) = random_field(*board, &mut count);
                                    let co: [usize; 2] = [row, column];
                                    break co;
                                },
                            },
                        },
                    };
                    // APPLY
                    board[coords[0]][coords[1]] = &BoxState::Flagged(2);
                    println!(
                        "Player {} selected Field {}{}",
                        player, COLUMN_IDS[coords[1]], ROW_IDS[coords[0]]
                    );
                    // CHECK FOR WINNER
                    let wn = check_win(*board, player);
                    if wn != 0 {
                        break wn * 10;
                    }
                    // next player
                    player = 1;
                }
            }
        } else {
            // UNKNOWN COMAND
        }
    };
    return result;
}

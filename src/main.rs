use text_io::scan;

// TODO enum BoardSymbol

#[derive(PartialEq, Copy, Clone)]
enum BoardPositionState {
    Clear,
    Cross,
    Circle,
}

impl BoardPositionState {
    fn print(&self) {
        match self {
            BoardPositionState::Clear => print!("   "),
            BoardPositionState::Cross => print!(" X "),
            BoardPositionState::Circle => print!(" O "),
        }
    }
}

struct GameState {
    board_positions_states: Vec<Vec<BoardPositionState>>,
    machine_symbol: BoardPositionState,
}

impl GameState {

    fn clone(&self) -> GameState {
        GameState {
            board_positions_states: self.board_positions_states.clone(),
            machine_symbol: self.machine_symbol.clone()
        }
    }

    fn print(&self) {
        println!("-------------");
        for row in &self.board_positions_states {
            for board_position in row {
                print!("|");
                board_position.print();
            }
            println!("|");
            println!("-------------");
        }
    }

    fn is_finished(&self) -> bool {
        let bps = &self.board_positions_states;
        
        let is_finished = 
        (bps[0][0] != BoardPositionState::Clear && bps[0][0] == bps[0][1] && bps[0][1] == bps[0][2]) ||
        (bps[1][0] != BoardPositionState::Clear && bps[1][0] == bps[1][1] && bps[1][1] == bps[1][2]) ||
        (bps[2][0] != BoardPositionState::Clear && bps[2][0] == bps[2][1] && bps[2][1] == bps[2][2]) ||
        (bps[0][0] != BoardPositionState::Clear && bps[0][0] == bps[1][0] && bps[0][0] == bps[2][0]) ||
        (bps[0][1] != BoardPositionState::Clear && bps[0][1] == bps[1][1] && bps[0][1] == bps[2][1]) ||
        (bps[0][2] != BoardPositionState::Clear && bps[0][2] == bps[1][2] && bps[0][2] == bps[2][2]) ||
        (bps[0][0] != BoardPositionState::Clear && bps[0][0] == bps[1][1] && bps[0][0] == bps[2][2]) ||
        (bps[0][2] != BoardPositionState::Clear && bps[0][2] == bps[1][1] && bps[0][2] == bps[2][0]) ||
        (
            bps[0][0] != BoardPositionState::Clear && bps[0][1] != BoardPositionState::Clear && bps[0][2] != BoardPositionState::Clear &&
            bps[1][0] != BoardPositionState::Clear && bps[1][1] != BoardPositionState::Clear && bps[1][2] != BoardPositionState::Clear &&
            bps[2][2] != BoardPositionState::Clear && bps[2][1] != BoardPositionState::Clear && bps[2][2] != BoardPositionState::Clear);
        
        return is_finished;
    }

    fn is_machine_winner(&self) -> bool {
        let bps = &self.board_positions_states;

        return
        (bps[0][0] == self.machine_symbol && bps[0][0] == bps[0][1] && bps[0][1] == bps[0][2]) ||
        (bps[1][0] == self.machine_symbol && bps[1][0] == bps[1][1] && bps[1][1] == bps[1][2]) ||
        (bps[2][0] == self.machine_symbol && bps[2][0] == bps[2][1] && bps[2][1] == bps[2][2]) ||
        (bps[0][0] == self.machine_symbol && bps[0][0] == bps[1][0] && bps[0][0] == bps[2][0]) ||
        (bps[0][1] == self.machine_symbol && bps[0][1] == bps[1][1] && bps[0][1] == bps[2][1]) ||
        (bps[0][2] == self.machine_symbol && bps[0][2] == bps[1][2] && bps[0][2] == bps[2][2]) ||
        (bps[0][0] == self.machine_symbol && bps[0][0] == bps[1][1] && bps[0][0] == bps[2][2]) ||
        (bps[0][2] == self.machine_symbol && bps[0][2] == bps[1][1] && bps[0][2] == bps[2][0]);
    }

    fn is_draw(&self) -> bool {
        let bps = &self.board_positions_states;

        return
        (bps[0][0] != bps[0][1] || bps[0][1] != bps[0][2]) &&
        (bps[1][0] != bps[1][1] || bps[1][1] != bps[1][2]) &&
        (bps[2][0] != bps[2][1] || bps[2][1] != bps[2][2]) &&
        (bps[0][0] != bps[1][0] || bps[0][0] != bps[2][0]) &&
        (bps[0][1] != bps[1][1] || bps[0][1] != bps[2][1]) &&
        (bps[0][2] != bps[1][2] || bps[0][2] != bps[2][2]) &&
        (bps[0][0] != bps[1][1] || bps[0][0] != bps[2][2]) &&
        (bps[0][2] != bps[1][1] || bps[0][2] != bps[2][0]) &&
        (
            bps[0][0] != BoardPositionState::Clear && bps[0][1] != BoardPositionState::Clear && bps[0][2] != BoardPositionState::Clear &&
            bps[1][0] != BoardPositionState::Clear && bps[1][1] != BoardPositionState::Clear && bps[1][2] != BoardPositionState::Clear &&
            bps[2][2] != BoardPositionState::Clear && bps[2][1] != BoardPositionState::Clear && bps[2][2] != BoardPositionState::Clear);
        
    }

    fn set_board_position(&mut self, row: usize, column: usize, value: BoardPositionState) {
        if self.board_positions_states[row][column] == BoardPositionState::Clear {
            self.board_positions_states[row][column] = value;
        }
    }

    fn get_clear_positions(&self) -> Vec<(usize, usize)>{
        let mut clear_positions:Vec<(usize, usize)>  = Vec::new();

        for i in 0..self.board_positions_states.len() {
            for j in 0..self.board_positions_states[i].len() {
                if self.board_positions_states[i][j] == BoardPositionState::Clear {
                    clear_positions.push((i, j));
                }
            }
        }

        clear_positions
    }
}

fn max_value(game_state: &mut GameState) -> u32 {
    if game_state.is_finished() {
        if game_state.is_draw() {
            return 1;
        }
        else if game_state.is_machine_winner() {
            return 2;
        }
        else {
            return 0;
        }
    }

    let mut max: u32 = 0;
    let mut value: u32;
    let mut new_game_state: GameState;
    for cp in game_state.get_clear_positions() {
        new_game_state = game_state.clone();
        new_game_state.set_board_position(cp.0, cp.1, new_game_state.machine_symbol.clone());
        value = min_value(&mut new_game_state);
        if value > max {
            max = value;
        }
    }

    max
}

fn min_value(game_state: &mut GameState) -> u32 {
    if game_state.is_finished() {
        if game_state.is_draw() {
            return 1;
        }
        else if game_state.is_machine_winner() {
            return 2;
        }
        else {
            return 0;
        }
    }

    let mut min: u32 = 3;
    let mut value: u32;
    let mut new_game_state: GameState;
    for cp in game_state.get_clear_positions() {
        new_game_state = game_state.clone();
        new_game_state.set_board_position(cp.0, cp.1, BoardPositionState::Cross);
        value = max_value(&mut new_game_state);
        if value < min {
            min = value;
        }
    }

    return min;
}

fn minmax(game_state: &GameState) -> (usize, usize) {
    let possible_plays = game_state.get_clear_positions();
    let mut next_play = possible_plays[0];
    let mut max: u32 = 0;
    let mut value: u32;
    let mut new_game_state: GameState;

    for cp in possible_plays {
        new_game_state = game_state.clone();
        new_game_state.set_board_position(cp.0, cp.1, new_game_state.machine_symbol.clone());
        value = min_value(&mut new_game_state);
        if value > max {
            max = value;
            next_play = cp;
        }
    }

    next_play
}

fn machine_play(game_state: &GameState) -> (usize, usize){
    minmax(game_state)
}

fn main() {
    let mut game_state = GameState {
        board_positions_states: vec![
            vec![BoardPositionState::Clear,BoardPositionState::Clear,BoardPositionState::Clear,],
            vec![BoardPositionState::Clear,BoardPositionState::Clear,BoardPositionState::Clear,],
            vec![BoardPositionState::Clear,BoardPositionState::Clear,BoardPositionState::Circle,],],
        machine_symbol: BoardPositionState::Circle
    };

    let mut row: usize;
    let mut column: usize;

    while !game_state.is_finished() {
        game_state.print();

        println!("Play: ");
        scan!("{} {}", row, column);
        game_state.set_board_position(row, column, BoardPositionState::Cross);

        let machine_play = machine_play(&game_state);
        game_state.set_board_position(machine_play.0, machine_play.1, game_state.machine_symbol.clone());
    }

    if game_state.is_draw() {
        println!("Draw!");
    }
    else {
        println!("The winner is: {}", if game_state.is_machine_winner() {"machine"} else {"you"});
    }
    game_state.print();
}

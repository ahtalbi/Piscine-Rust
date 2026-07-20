pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }

    if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    }

    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][1] == player && table[2][2] == player) || (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[i][0] == player && table[i][1] == player && table[i][2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[0][i] == player && table[1][i] == player && table[2][i] == player {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
        );
        // tie
        println!(
            "{}",
            tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
        );
        // player O won
    
        let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];
    
        println!("{}", tic_tac_toe(diag));
        // player X won
    }
}

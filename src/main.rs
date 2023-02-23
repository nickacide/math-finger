use std::ops::Not;

/*
        Indexing
┌───┬───┐
│ 2 │ 3 │  Computer
├───┼───┤
│ 0 │ 1 │  Player
└───┴───┘
*/
#[derive(PartialEq, Eq)]
enum Turn {
    Player,
    Computer,
}
impl Not for Turn {
    type Output = Turn;
    fn not(self) -> Self::Output {
        if self == Turn::Player {
            return Turn::Computer;
        }
        Turn::Player
    }
}

enum Flag {
    Attack,
    Merge,
    Split,
}

fn evaluate(player: &[i32; 2], computer: &[i32; 2]) -> i32 {
    if *player == [0, 0] {
        return -1;
    }
    if *computer == [0, 0] {
        return 1;
    }
    0 // we can't determine the score of a position until we see the future (to find deciding result)
}

fn game_over(player: &[i32; 2], computer: &[i32; 2]) -> bool {
    *player == [0, 0] || *computer == [0, 0]
}

fn moves(player: &[i32; 2], computer: &[i32; 2], turn: &Turn) -> Vec<([usize; 2], Flag)> {
    let mut moves: Vec<([usize; 2], Flag)> = vec![];
    if evaluate(player, computer) != 0 {
        return moves;
    }
    // offense (can't strike a dead target; striker must be alive)
    if turn == &Turn::Player {
        if player[0] != 0 {
            if computer[0] != 0 {
                moves.push(([0, 2], Flag::Attack));
            }
            if computer[1] != 0 {
                moves.push(([0, 3], Flag::Attack));
            }
        }
        if player[1] != 0 {
            if player[0] != 0 {
                moves.push(([1, 2], Flag::Attack));
            }
            if player[1] != 0 {
                moves.push(([1, 3], Flag::Attack));
            }
        }
    } else {
        if computer[0] != 0 {
            if player[0] != 0 {
                moves.push(([2, 0], Flag::Attack));
            }
            if player[1] != 0 {
                moves.push(([2, 1], Flag::Attack));
            }
        }
        if computer[1] != 0 {
            if player[0] != 0 {
                moves.push(([3, 0], Flag::Attack));
            }
            if player[1] != 0 {
                moves.push(([3, 1], Flag::Attack));
            }
        }
    }
    // defense - splitting
    if turn == &Turn::Player {
        match *player {
            [0, n] => {
                if n % 2 == 0 {
                    moves.push(([1, 0], Flag::Split));
                }
            }
            [n, 0] => {
                if n % 2 == 0 {
                    moves.push(([0, 1], Flag::Split));
                }
            }
            _ => {}
        }
    } else {
        match *computer {
            [0, n] => {
                if n % 2 == 0 {
                    moves.push(([3, 2], Flag::Split));
                }
            }
            [n, 0] => {
                if n % 2 == 0 {
                    moves.push(([2, 3], Flag::Split));
                }
            }
            _ => {}
        }
    }
    // defense - merging
    if ((*player)[0] == 1 || player[0] == 2) && player[0] == player[1] && turn == &Turn::Player {
        moves.push(([0, 1], Flag::Merge));
        moves.push(([1, 0], Flag::Merge));
    }
    if (computer[0] == 1 || computer[0] == 2)
        && computer[0] == computer[1]
        && turn == &Turn::Computer
    {
        moves.push(([2, 3], Flag::Merge));
        moves.push(([3, 2], Flag::Merge));
    }
    moves
}

fn apply_move(
    player: &[i32; 2],
    computer: &[i32; 2],
    r#move: &([usize; 2], Flag),
) -> ([i32; 2], [i32; 2]) {
    let mut combined = [player[0], player[1], computer[0], computer[1]];
    match r#move.1 {
        Flag::Attack => {
            combined[r#move.0[1]] = (combined[r#move.0[1]] + combined[r#move.0[0]]) % 5;
        }
        Flag::Merge => {
            combined[r#move.0[0]] = 0;
            combined[r#move.0[1]] *= 2;
        }
        Flag::Split => {
            combined[r#move.0[1]] = combined[r#move.0[0]] / 2;
            combined[r#move.0[0]] /= 2;
        }
    }
    ([combined[0], combined[1]], [combined[2], combined[3]])
}

fn best(
    player: &[i32; 2],
    computer: &[i32; 2],
    depth: u8,
    player_turn: &Turn,
) -> (i32, [usize; 2]) {
    fn minimax(
        player: &[i32; 2],
        computer: &[i32; 2],
        depth: u8,
        player_turn: &Turn,
    ) -> (i32, [usize; 2]) {
        if depth == 0 || game_over(player, computer) {
            return (evaluate(player, computer), [0, 0]);
        }
        if player_turn == &Turn::Player {
            let mut best_score: i32 = -2;
            let mut best_move: [usize; 2] = [0, 0];
            for r#move in moves(player, computer, player_turn) {
                let (new_player, new_computer) = apply_move(player, computer, &r#move);
                let (score, _) = minimax(&new_player, &new_computer, depth - 1, &Turn::Computer);
                if score > best_score {
                    best_score = score;
                    best_move = r#move.0;
                }
            }
            return (best_score, best_move);
        } else {
            let mut best_score: i32 = 2;
            let mut best_move: [usize; 2] = [0, 0];
            for r#move in moves(player, computer, player_turn) {
                let (new_player, new_computer) = apply_move(player, computer, &r#move);
                let (score, _) = minimax(&new_player, &new_computer, depth - 1, &Turn::Player);
                if score < best_score {
                    best_score = score;
                    best_move = r#move.0;
                }
            }
            return (best_score, best_move);
        }
    }
    let (best_score, best_move) = minimax(player, computer, depth, player_turn);
    return (best_score, best_move);
}

fn main() {
    let computer = [1, 0];
    let player = [2, 2];
    let best = best(&player, &computer, 15, &Turn::Computer);
    println!("Best: {:?}", best);
}

const WIN_THRESHOLD: u16 = 1000;

trait Die {
    fn roll(&mut self) -> usize;
    fn count(&self) -> usize;
}
struct CountingDie {
    next_roll: usize,
    roll_count: usize,
    nsides: usize
}

impl CountingDie {
    fn new(nsides: usize) -> Self {
        Self { next_roll: 1, roll_count: 0, nsides }
    }
}

impl Die for CountingDie {
    fn roll(&mut self) -> usize {
        let roll = self.next_roll;
        self.next_roll += 1;
        self.roll_count += 1;
        if self.next_roll > self.nsides { self.next_roll = 1; }
        roll
    }

    fn count(&self) -> usize {
        self.roll_count
    }
}

struct DiracGame {
    die: Box<dyn Die>,
    player_positions: [usize; 2],
    player_scores: [u16; 2],
    current_player: usize
}

impl DiracGame {
    fn new(player_positions: [usize; 2], die: Box<dyn Die>) -> Self {
        DiracGame {
            die,
            player_scores: [0; 2],
            player_positions: player_positions,
            current_player: 0
        }
    }

    fn has_winner(&self) -> Option<usize> {
        self.player_scores.iter().position(|&p| p > WIN_THRESHOLD)
    }

    fn take_turn(&mut self) {
        let sum_rolls : usize = (0..3).map(|_| self.die.roll()).sum();
        // Update the new player location
        self.player_positions[self.current_player] = (sum_rolls +
            self.player_positions[self.current_player]) % 10;
        // Add score for the new space
        self.player_scores[self.current_player] += 1 + self.player_positions[self.current_player] as u16;
        self.current_player = (self.current_player + 1) % 2;
    }

    fn play_until_won(&mut self) -> usize {
        loop {
            if let Some(winner) = self.has_winner() {
                return winner
            }
            self.take_turn();
        }
    }
}

fn main() {
    //// PART 1 ////
    // positions are actually 1&3, but those are in slots 0&2
    let die = Box::new(CountingDie::new(100));
    let mut game = DiracGame::new([0, 2], die);
    let winner = game.play_until_won();

    println!("Winner is player {}.", winner + 1);
    println!("{} die rolls * {} losing score == {}",
        game.die.count(),
        game.player_scores[(winner + 1) % 2],
        game.die.count() * game.player_scores[(winner + 1) % 2] as usize
    );

    //// PART 2 ////
    // I thought I might have fun with traits, but then they went and made
    // this a math problem. boo hoo.
}

use std::io::{self, Write};
use lb10::{Deck, Game, Player};


fn main() {
    //Game Prep
    //initializing a Deck
    let mut deck = Deck::new();
    let player_names: Vec<String> = vec!["Jack".to_string(), "Mark".to_string()];

    //initialzing players
    let mut players: Vec<Player> = Vec::new();
    for name in player_names {
        let player: Player = Player::new(name, &mut deck);
        players.push(player);
    }

    //initializing the dealer
    let mut dealer = Player::new("Dealer".to_string(), &mut deck);

    //initializing game
    let mut game = Game::new(players);
    let mut round_num: u8 = 1;

    //Game Start
    loop {
        Game::play_round(&mut game, &mut dealer, &mut deck, &mut round_num);

        println!("Do you want to play another round? Enter Yes or No");
        let mut input = String::new();
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("yes") {
            //new deck for next game and reseting players
            deck = Deck::new();
            dealer.reset(&mut deck);
            for player in &mut game.players {
                player.reset(&mut deck);
            }
            round_num += 1;
            println!("{}", round_num);
        } else {
            break;
        }
    }
}

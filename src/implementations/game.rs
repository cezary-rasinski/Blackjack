use super::player::Player;
use super::deck::Deck;
use super::action::Action;
pub struct Game {
    pub players: Vec<Player>,
}
impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        return Game { players };
    }
    pub fn show_player_details(&mut self, dealer: &mut Player, round_num: &mut u8) { 
        println!("\nRound {}", round_num);

            //display delaer's first card
            println!("\nDealer's info");
            print!("First card: ");
            dealer.first_card();
            dealer.first_card_val();

            //display players info and make bets
            for player in &mut self.players {
                player.show_info();
                player.bet();
            }

    }
    pub fn handle_player_turns(&mut self, deck: &mut Deck, player_already_won: &mut bool){
            let mut no_actions = false;

            //exit when no-one makes a move
            while !no_actions {

                //check if there is an immediate winner
                for player in &mut self.players {
                    //if there is an immediate winner
                    if player.total_hand_val() == 21 {
                        player.win();
                        *player_already_won = true;
                        return;
                    }

                    //if player hasn't resigned or chose to stand
                    if player.in_play() {
                        no_actions = false;

                        //choosing an action and showing current amount of chips
                        println!("Choose an action {}", player.name());
                        player.see_hand();
                        let action = Action::read_action();

                        //executing an action
                        match action {
                            Action::Hit => {
                                player.draw(deck);
                                player.see_hand();
                            }
                            Action::Stand => {
                                player.stop();
                            }
                            Action::Double => {
                                player.double();
                                player.draw(deck);
                                player.see_hand();
                                player.stop();
                            }
                            Action::Surrender => {
                                player.surrender();
                            }
                        }

                        //seeing if the player has won
                        if player.total_hand_val() == 21 {
                            player.win();
                            return;
                        }

                        //checking if the player has busted
                        if player.total_hand_val() > 21 {
                            println!("Bust! {} has lost!\n\n", player.name());
                            player.stop();
                        }
                    } else {
                        no_actions = true;
                    }
                }
            }
    }
    pub fn handle_dealer_turn(dealer: &mut Player, deck: &mut Deck){
        
        //once all players have finished, show the full dealer's hand
            println!("Dealer's hand: ");
            dealer.see_hand();

        //forcing the dealer to draw while his total_val is less than 17
        while dealer.total_hand_val() < 17 {
            println!("Drawing");
            dealer.draw(deck);
            println!("Dealer's hand: ");
            dealer.see_hand();
        }

        while dealer.total_hand_val() <= 21 && dealer.in_play() == true {
            if dealer.total_hand_val() == 21 {
                println!("The Dealer has won!");
                return;
            } else {
                let d_action = Action::read_action();
                match d_action {
                    Action::Hit => {
                        dealer.draw(deck);
                        println!("Dealer's hand: ");
                        dealer.see_hand();
                    }
                    Action::Stand => {
                        dealer.stop();
                    }
                    Action::Double => {}
                    Action::Surrender => {}
                }
            }
        }
        if dealer.total_hand_val() > 21 {
            println!("Bust! the dealer has lost");
            dealer.stop();
        };
    }
    pub fn play_round(game: &mut Game, dealer: &mut Player, deck: &mut Deck, round_num: &mut u8){
    
        loop {
            let mut player_already_won = false;
            Game::show_player_details(game, dealer, round_num);

            Game::handle_player_turns(game, deck, &mut player_already_won);

            if !player_already_won {
                Game::handle_dealer_turn(dealer, deck);
            }
            

            let mut best_score = 0;
            let mut winners: Vec<&mut Player> = Vec::new();

            for player in &mut game.players {
                let p_score = player.total_hand_val();

                if p_score > 21 {
                    println!("{} busted with {}.", player.name(), p_score);
                    continue;
                }

                if !dealer.in_play() {
                    if p_score > dealer.total_hand_val() {
                        if p_score > best_score {
                            best_score = p_score;
                            winners.clear();
                            winners.push(player);
                        } else if p_score == best_score {
                            winners.push(player);
                        }
                    } else if p_score == dealer.total_hand_val() {
                        println!("{} tied with the dealer at {}.", player.name(), p_score);
                        player.stealmate(); // Refund bet on tie
                    } else {
                        println!(
                            "{} lost to the dealer ({} < {}).",
                            player.name(),
                            p_score,
                            dealer.total_hand_val()
                        );
                    }
                } else {
                    if p_score > best_score {
                        best_score = p_score;
                        winners.clear();
                        winners.push(player);
                    } else if p_score == best_score {
                        winners.push(player);
                    }
                }
            }

            if dealer.in_play() && winners.is_empty() {
                println!("Dealer wins with {}!", dealer.total_hand_val());
                return;
            } else if winners.len() == 1 {
                println!(
                    "{} wins with {}!",
                    winners[0].name(),
                    winners[0].total_hand_val()
                );
                winners[0].win();
                return;
            } else if winners.len() > 1 {
                println!("It's a draw between:");
                for p in &mut winners {
                    println!("  - {} with {}", p.name(), p.total_hand_val());
                    p.stealmate(); // Refund bet on tie
                }
                return;
            }
        }
    }
}
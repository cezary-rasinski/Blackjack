use std::io;
use super::deck::Deck;
use super::hand::Hand;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Player {
    name: String,
    hand: Hand,
    chips: i32,
    betted_chips: i32,
    in_play: bool,
}
impl Player {
    pub fn new(name: String, deck: &mut Deck) -> Self {
        let hand = Hand::new(deck);
        return Player {
            name,
            hand,
            chips: 50,
            betted_chips: 0,
            in_play: true,
        };
    }
    pub fn stop(&mut self) {
        self.in_play = false;
    }
    pub fn reset(&mut self, deck: &mut Deck) {
        self.in_play = true;
        self.hand = Hand::new(deck);
    }
    pub fn show_info(&self) {
        // println!("Player's info");
        // println!("name: {}", self.name);
        // self.hand.see_hand();
        // println!("amount of chips: {}", self.chips);

        let cards = self
        .hand
        .cards
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let line1 = format!("{} ({})", self.name, cards);
    let line2 = format!("Value: {}", self.hand.total_val());
    let line3 = format!("Chips: {}", self.chips);

    // 2. Compute the box width (max line length)
    let width = [line1.len(), line2.len(), line3.len()]
        .iter()
        .cloned()
        .max()
        .unwrap();

    // 3. Print top border
    println!("┌{:─<1$}┐", "", width);

    // 4. Print each content line, left-aligned
    println!("│{:<1$}│", line1, width);
    println!("│{:<1$}│", line2, width);
    println!("│{:<1$}│", line3, width);

    // 5. Print bottom border
    println!("└{:─<1$}┘", "", width);
    }
    pub fn see_hand(&self) {
        // println!("{:?}", self.hand.see_hand());
        // println!("amount of chips: {}", self.chips);
        let w1 = 20;
        let w2 = 12;

        println!("┌{:─<w1$}┬{:─<w2$}┐", "", "");
        let cards = self
            .hand
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let col1 = format!("{} ({})", self.name, cards);
        let col2 = format!("Value: {}", self.hand.total_val());
        println!("│{:<w1$}│{:<w2$}│", col1, col2);
        
        // bottom border
        println!("└{:─<w1$}┴{:─<w2$}┘", "", "");
    }
    pub fn bet(&mut self) {
        let amount = loop {
            println!("Enter how many chips you want to bet: ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<i32>() {
                Ok(num) if num > 0 => break num,
                _ => {
                    println!("Invalid input. Please enter a positive integer.");
                }
            }
        };
        self.chips -= amount;
        self.betted_chips = amount;
    }
    pub fn win(&mut self) {
        println!("{} has won!", self.name);
        self.chips += self.betted_chips * 2;
    }
    pub fn surrender(&mut self) {
        println!("{} has surrendered", self.name);
        self.chips += self.betted_chips / 2;
        self.stop();
    }
    pub fn double(&mut self) {
        self.chips -= self.betted_chips;
        self.betted_chips *= 2;
    }
    pub fn stealmate(&mut self) {
        self.chips += self.betted_chips;
    }
    pub fn in_play(&self) -> bool{
        return self.in_play;
    }
    pub fn name(&self) -> String{
        return self.name.clone();
    }
    pub fn first_card(&self){
        println!("{:?}", self.hand.cards.first());
    }
    pub fn first_card_val(&self){
        println!(
                "total value: {:?}\n",
                self.hand.cards.first().unwrap().card_value()
            );
    }
    pub fn total_hand_val(&self) -> u8 {
        self.hand.total_val()
    }
    pub fn draw(&mut self, deck: &mut Deck){
        self.hand.draw(deck);
    }
}
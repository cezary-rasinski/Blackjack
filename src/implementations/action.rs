use std::io;
pub enum Action {
    Hit,
    Stand,
    Double,
    Surrender,
}

impl Action {
    pub fn print_menu() {
        println!("  1) Hit");
        println!("  2) Stand");
        println!("  3) Double Down");
        println!("  4) Surrender");
    }
    pub fn read_action() -> Action {
        loop {
            Action::print_menu();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "1" => return Action::Hit,
                "2" => return Action::Stand,
                "3" => return Action::Double,
                "4" => return Action::Surrender,
                other => {
                    println!("‘{other}’ is not a valid choice, try again.");
                    continue;
                }
            }
        }
    }
  
}



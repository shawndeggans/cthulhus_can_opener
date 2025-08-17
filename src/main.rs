use cthulhus_can_opener::Game;

fn main() {
    println!("Welcome to Cthulhu's Can Opener!");
    
    let mut game = Game::new("Player".to_string());
    
    for i in 1..=5 {
        println!("\n--- Turn {} ---", i);
        let event = game.random_event();
        println!("{}", event);
        println!("Current score: {}", game.get_score());
    }
    
    println!("\nGame over! Final score: {}", game.get_score());
}

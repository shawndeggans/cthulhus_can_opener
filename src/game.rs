use rand::Rng;

pub struct Game {
    pub player_name: String,
    pub score: u32,
}

impl Game {
    pub fn new(player_name: String) -> Self {
        Self {
            player_name,
            score: 0,
        }
    }

    pub fn random_event(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let event = rng.gen_range(1..=3);
        
        match event {
            1 => {
                self.score += 10;
                "You found Cthulhu's can opener! +10 points".to_string()
            }
            2 => {
                self.score += 5;
                "You discovered an ancient artifact. +5 points".to_string()
            }
            _ => {
                "The cosmic horror grows... but nothing happens.".to_string()
            }
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
}
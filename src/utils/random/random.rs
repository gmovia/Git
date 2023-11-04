use rand::Rng;

pub struct Random;

impl Random{
    pub fn random() -> String{
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(1..=9).to_string();
        id
    }
}
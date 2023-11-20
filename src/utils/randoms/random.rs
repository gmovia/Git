use rand::Rng;

pub struct Random;

impl Random{
    pub fn random() -> String{
        let mut rng = rand::thread_rng();
        rng.gen_range(1000..=9000).to_string()
    }
}
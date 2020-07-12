use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(n).collect()
}
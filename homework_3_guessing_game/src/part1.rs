use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;


pub struct Part1 {}


impl Strategy for Part1 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        for guess in min..max {
            if player.ask_if_equal(guess) {
                return guess;
            }
        }
        unreachable!("Cheater!!! Choose a number in the range.")
    }   
}
pub mod front_of_house;
pub mod back_of_house;

use crate::restaurant::back_of_house::kitchen;
use front_of_house::hosting;

pub fn enter() {
    kitchen::cook_chicken();
    hosting::add_to_waitlist();
} 

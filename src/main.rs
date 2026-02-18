mod pick_color;

use rand::{RngExt};
use crate::pick_color::pick;

fn main() {

    let color: [&str; 3] = ["red", "green", "blue"];

    let mut rng = rand::rng();
    let rand_el = rng.random_range(0..3);

    let el = color[rand_el];

    pick(el, rand_el.to_string().parse().unwrap());

}

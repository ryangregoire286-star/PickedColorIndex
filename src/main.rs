mod pick_color;
mod db;

use rand::{RngExt};
use crate::pick_color::pick;

fn main() {

    
    const LENGTH: i32 = 3usize as i32;
    const X: usize = 0;
    const Y: usize = 3;

    let (x, y) = (X, Y);

    let color: [&str; LENGTH as usize] = ["red", "green", "blue"];

    let mut rng = rand::rng();
    let rand_el = rng.random_range(x..y);

    let el = color[rand_el];

    let _ = db::get_client(el);

    pick(el, rand_el.to_string().parse().unwrap());

}

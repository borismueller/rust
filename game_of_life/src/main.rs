//extern crate rand;

//use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    //world is our field, it holds booleans which say wether the cell is alive or not
    let mut world = init_world(5);
    let episodes = 10;
    for i in 0..episodes {
        println!("Episode: {0}", i);
        print_world(&world);
        get_num_neighbors(&world, 1, 0);
        world = apply_rules(world);
    }
}

fn init_world(size: usize) -> Vec<Vec<bool>> {
    let mut world: Vec<Vec<bool>> = Vec::new();
    for _ in 0..size {
        let mut w: Vec<bool> = Vec::new();
        for _ in 0..size {
            //generate a random bool for each cell
            w.push(random_bool());
        }
        world.push(w);
    }
    world
}

fn print_world(world: &Vec<Vec<bool>>) {
    for w in world {
        print!("\t");
        for cell in w.clone() {
            if cell {
                print!("* ");
            } else {
                print!("- ");
            }
        }
        print!("\n");
    }
}

fn random_bool() -> bool {
    //let's make our own, bad, randomness
    //get current time in ms
    let seed = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let s: usize = seed.subsec_nanos() as usize;

    //get our random number with our seed
    let rand = lcg(134456, 8121, 28411, s);

    //return our random bool based on our number
    return rand % 2 == 0
}

fn lcg(m: usize, a: usize, c: usize, s: usize) -> usize {
    //linear congruential generator
    //m: modulus, a: multiplier, c: increment, s: seed
    (a * s + c) % m
}

fn apply_rules(world: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    //Apply the rules to our world and return the resulting world
    let mut new_world = world.clone(); //new world because the rules don't work out otherwise
    for i in 0..world.len() {
        for j in 0..world[i].len() {
            let num_alive = get_num_neighbors(&world, i, j);
            if world[i][j] {
                //cell is alive
                if num_alive < 2 {
                    new_world[i][j] = false;
                } else if num_alive > 3 {
                    new_world[i][j] = false;
                }
            } else {
                //cell is dead
                if num_alive == 3 {
                    new_world[i][j] = true;
                }
            }
        }
    }
    new_world
}

fn get_num_neighbors(world: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    //return the number of neighbors for the cell at index in world
    let mut num = 0;
    let i = i as i32;
    let j = j as i32;

    for k in -1..2 {
        if !(i+k < 0 || i+k >= world.len() as i32) {
            for l in -1..2{
                if !(j+l < 0 || j+l >= world[0].len() as i32 || (k == 0 && l == 0)) {
                    //make sure we aren't out of bounds, or on the same field
                    if world[(i+k) as usize][(j+l) as usize] {
                        num += 1;
                    }
                }
            }
        }
    }
    num
}

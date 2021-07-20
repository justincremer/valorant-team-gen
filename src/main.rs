use std::env;
use std::error::Error;

use rand::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_print(gen_team()?);
    Ok(())
}

fn pretty_print(i: Vec<u32>) {
    println!(
        "{}",
        i.iter().enumerate().fold(String::new(), |acc, (i, c)| {
            acc + if i % 2 == 0 {
                format!("| {} |", c)
            } else {
                format!(" {} |\n", c)
            }
            .as_str()
        })
    );
}

fn gen_team() -> Result<Vec<u32>, Box<dyn Error>> {
    match args_parse() {
        Err(e) => return Err(e),
        Ok(gamers) => {
            let mut res: Vec<u32> = (1..(gamers + 1)).collect();
            let mut rng = rand::thread_rng();
            res.shuffle(&mut rng);
            Ok(res)
        }
    }
}

fn args_parse<'s>() -> Result<u32, Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    match args.len() {
        1 => Ok(args[0].parse::<u32>()?),
        _ => Err(Box::from("please provide a valid number")),
    }
}

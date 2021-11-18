#![feature(path_file_prefix)]

use clap::{App, Arg};
use image::imageops::overlay;
use image::{ImageBuffer, RgbaImage};
use rand::prelude::SliceRandom;
use rand::{Rng, RngCore};
use rayon::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct Config {
    output: String,
    default_rarity: f64,
    dims: Dims,
    layers: Vec<Layer>,
    rarities: Vec<Rarity>,
}

#[derive(Deserialize)]
struct Layer {
    folder: String,
    prob: f64,
}

#[derive(Deserialize)]
struct Rarity {
    suffix: String,
    factor: f64,
}

#[derive(Deserialize)]
struct Dims {
    height: usize,
    width: usize,
}

fn main() {
    let mut file = File::open("config.toml").expect("Couldn't find the config.toml");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = toml::de::from_str(&content).expect("Couldn't parse the config.toml");

    let matches = App::new("Png merger")
        .author("Samuel Batissou")
        .arg(
            Arg::new("n")
                .short('n')
                .value_name("n")
                .about("Number of png to generate")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let n = matches.value_of_t("n").unwrap();
    let mut stats = (0..n)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let id = rng.next_u64();
            let mut image_buff: RgbaImage =
                ImageBuffer::new(config.dims.width as u32, config.dims.height as u32);
            let mut layers_used = Vec::with_capacity(config.layers.len());
            for l in &config.layers {
                let Layer { folder, prob } = l;
                let needle: f64 = rng.gen();
                if needle < *prob {
                    let objects = WalkDir::new(folder)
                        .into_iter()
                        .filter_map(|r| r.ok().filter(|x| !x.path().is_dir()))
                        .map(|x| {
                            let name = x
                                .path()
                                .file_prefix()
                                .map(|os| os.to_str())
                                .flatten()
                                .unwrap();
                            for rarity in &config.rarities {
                                if name.contains(&format!("_{}",&rarity.suffix)) {
                                    return (x, config.default_rarity / rarity.factor as f64);
                                }
                            }
                            (x, config.default_rarity) // Default rarity
                        })
                        .collect::<Vec<_>>();
                    let (path_chosen, _) = objects.choose_weighted(&mut rng, |x| x.1).unwrap();
                    layers_used.push(path_chosen.path().to_str().unwrap().to_string());
                    let image = image::open(path_chosen.path()).unwrap();
                    overlay(&mut image_buff, &image, 0, 0);
                }
            }
            image_buff
                .save(format!("{}/{}.png", config.output, id))
                .expect("Couldn't save the image");
            println!("{} created. layers used {:?}", id, layers_used);
            layers_used
        })
        .flatten()
        .collect::<Vec<_>>();

    stats.sort();

    let mut count = 1;
    let mut before = &stats[0];
    for s in &stats[1..] {
        if before != s {
            println!("{} attributed     {} times.", before, count);
            count = 1;
            before = s;
        } else {
            count += 1;
        }
    }
    println!("{} attributed     {} times.", before, count);
}

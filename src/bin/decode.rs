extern crate pretty_env_logger;
#[macro_use] extern crate log;

use anyhow::Result;
use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

#[derive(Debug, Deserialize)]
struct Config {
  boolean: bool,
  float: f32,
  map: HashMap<u8, char>,
  nested: Nested,
  tuple: (u32, u32),
}

#[derive(Debug, Deserialize)]
struct Nested {
  a: String,
  b: char,
}

fn get_config() -> Result<Config> {
  let input_path = format!("{}/example.ron", env!("CARGO_MANIFEST_DIR"));
  let f = File::open(&input_path)?;
  let config: Config = from_reader(f)?;

  Ok(config)
}

fn main() -> Result<()>{
  better_panic::install();
  pretty_env_logger::init();

  info!("Config: {:#?}", &get_config());

  Ok(())
}

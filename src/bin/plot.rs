use anyhow::Result;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

#[derive(Debug, Deserialize)]
struct Config {
  plot_color_1: String,
  plot_color_2: String,
  plot_file_name: String,
}

fn plot(config: Config) {
  let data1 = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];

  let s1: Plot =
    Plot::new(data1).point_style(PointStyle::new().marker(PointMarker::Square).colour(config.plot_color_1));

  let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];
  let s2: Plot = Plot::new(data2).point_style(PointStyle::new().colour(config.plot_color_2));

  let v = ContinuousView::new()
    .add(s1)
    .add(s2)
    .x_range(-5., 10.)
    .y_range(-2., 6.)
    .x_label("Some varying variable")
    .y_label("The response of something");

  Page::single(&v).save(config.plot_file_name).unwrap();
}

fn get_config() -> Result<Config> {
  let input_path = format!("{}/plot.ron", env!("CARGO_MANIFEST_DIR"));
  let f = File::open(&input_path)?;
  let config: Config = from_reader(f)?;

  Ok(config)
}

fn main() -> Result<()> {
  better_panic::install();
  plot(get_config()?);

  Ok(())
}

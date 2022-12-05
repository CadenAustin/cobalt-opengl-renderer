use miniquad::*;
use stage::Stage;

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

mod stage;
mod shader;
mod scene;
mod camera;
mod egui_menu;
mod material;

fn main() {
    let conf = conf::Conf {
        window_title: "Cobalt".into(),
        window_height: HEIGHT,
        window_width: WIDTH,
        ..Default::default()
    };
    miniquad::start(conf, |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}
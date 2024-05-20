use iai::black_box;

use raytracer::raytracer;
use raytracer::{read_scene_file, write_image};

fn render_image(file_path_str: &String) {
    let scene = read_scene_file(file_path_str);
    let pixels = raytracer::render(&scene);
    let img_obj = raytracer::build_image((scene.img_width, scene.img_height), &pixels);
    write_image(&"out.png".to_string(), img_obj);
}

fn iai_benchmark_scene00() {
    render_image(black_box(&"test_scenes/scene00.test".to_owned()))
}

fn iai_benchmark_scene01() {
    render_image(black_box(&"test_scenes/scene01.test".to_owned()))
}

fn iai_benchmark_scene02() {
    render_image(black_box(&"test_scenes/scene02.test".to_owned()))
}

fn iai_benchmark_scene03() {
    render_image(black_box(&"test_scenes/scene03.test".to_owned()))
}

fn iai_benchmark_scene04_ambient() {
    render_image(black_box(&"test_scenes/scene04-ambient.test".to_owned()))
}

fn iai_benchmark_scene04_diffuse() {
    render_image(black_box(&"test_scenes/scene04-diffuse.test".to_owned()))
}

fn iai_benchmark_scene04_emission() {
    render_image(black_box(&"test_scenes/scene04-emission.test".to_owned()))
}

fn iai_benchmark_scene04_specular() {
    render_image(black_box(&"test_scenes/scene04-specular.test".to_owned()))
}

fn iai_benchmark_scene05() {
    render_image(black_box(&"test_scenes/scene05.test".to_owned()))
}

fn iai_benchmark_scene06() {
    render_image(black_box(&"test_scenes/scene06.test".to_owned()))
}

fn iai_benchmark_scene07() {
    render_image(black_box(&"test_scenes/scene07.test".to_owned()))
}

iai::main!(
    // iai_benchmark_scene00,
    // iai_benchmark_scene01,
    // iai_benchmark_scene02,
    // iai_benchmark_scene03,
    iai_benchmark_scene04_ambient,
    iai_benchmark_scene04_diffuse,
    iai_benchmark_scene04_emission,
    iai_benchmark_scene04_specular,
    iai_benchmark_scene05,
    iai_benchmark_scene06,
    // iai_benchmark_scene07
);

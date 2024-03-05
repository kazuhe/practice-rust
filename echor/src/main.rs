use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("kazuhe")
        .about("Rust echo command.")
        .get_matches();
    // println!("{:?}", std::env::args());
}

use find_cargo_toml::find;
use std::path::PathBuf;

fn main() {
    for path in find(".", None::<PathBuf>, None) {
        println!("Found: {}", path.display());
    }
}

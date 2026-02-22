# find-cargo-toml

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/find-cargo-toml)
![Crates.io Total Downloads](https://img.shields.io/crates/d/find-cargo-toml)
![docs.rs](https://img.shields.io/docsrs/find-cargo-toml)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/un-rust/find-cargo-toml)
![GitHub Repo stars](https://img.shields.io/github/stars/un-rust/find-cargo-toml)
<!-- /automdrs -->

<!-- automdrs:description -->

Find Cargo.toml by walking up the directory tree

<!-- /automdrs -->

**[Full documentation →](https://docs.rs/find-cargo-toml/)**

## Quick start

<!-- automdrs:cargo-add -->

```sh
cargo add find-cargo-toml
```

<!-- /automdrs -->

## Usage

<!-- automdrs:file src="./src/main.rs" -->
```rust
use find_cargo_toml::find;
use std::path::PathBuf;

fn main() {
    for path in find(".", None::<PathBuf>, None) {
        println!("Found: {}", path.display());
    }
}
```
<!-- /automdrs -->

## License

<!-- automdrs:contributors author="UnRUST" license="Apache-2.0" -->
Published under the [Apache-2.0](./LICENSE) license.
Made by [@UnRUST](https://github.com/un-rust) 💛
<br><br>
<a href="https://github.com/un-rust/find-cargo-toml/graphs/contributors">
<img src="https://contrib.rocks/image?repo=un-rust/find-cargo-toml" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->
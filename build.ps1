cargo build
cargo clippy --release
cargo flux
cargo build --release
Move-Item -Path "./target/release/elden_ring_improved_selectors.dll" -Destination "./OUTPUT/elden_ring_improved_selectors.dll" -Force
cd "./OUTPUT"
7z a -t7z "./release.7z" "./elden_ring_improved_selectors.dll" "./config.toml" -y
cd "../"
Pause
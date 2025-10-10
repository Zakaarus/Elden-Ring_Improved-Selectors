echo "Clearing ./build.log"
"" | Out-File "./build.log"
echo "Cleaning..."
cargo clean *>> ./build.log
echo "Building debug..."
cargo build *>> ./build.log
echo "Checking with clippy..."
cargo clippy --release *>> ./build.log
echo "Checking with flux..."
cargo flux *>> ./build.log
echo "Building release..."
cargo build --release *>> ./build.log
echo "Packing files..."
Move-Item -Path "./target/release/elden_ring_improved_selectors.dll" -Destination "./OUTPUT/er_IS.dll" -Force
cd "./OUTPUT"
7z a -t7z "./er_IS_release.7z" "./er_IS.dll" "./config.toml" -y *>> ../build.log
cd "../"
echo "Done. See ./build.log for details."
Pause
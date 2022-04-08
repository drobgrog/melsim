#!/bin/bash

# Cleanup
rm -rf "build/macos/dmg_staging"
rm melsim.dmg
rm build/macos/src/Applications

set -euxo pipefail
cargo build --release
mkdir -p build/macos/src/Game.app/Contents/MacOS/assets
cp -r assets/ build/macos/src/Game.app/Contents/MacOS/assets
cp -r narrative/ build/macos/src/Game.app/Contents/MacOS/narrative
cp target/release/melsim build/macos/src/Game.app/Contents/MacOS/
strip build/macos/src/Game.app/Contents/MacOS/melsim
mkdir -p build/macos/dmg_staging
cp -r build/macos/src/Game.app "build/macos/dmg_staging/Melbourne Simulator.app"
ln -s /Applications build/macos/dmg_staging/
hdiutil create -fs HFS+ -volname "Melbourne Simulator" -srcfolder build/macos/dmg_staging melsim.dmg
rm -rf build/macos/dmg_staging

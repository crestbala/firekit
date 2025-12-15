#!/bin/bash
set -e

echo "📦 Building frontend..."
cd front
if [ -e "build" ]; then
    rm -rf build
fi
npm install
npm run build

echo "🦀 Building backend..."
cd ../tail
cargo build --release
./target/release/tail

echo -e "\n\n✅ Build complete! Your software executable is in server/target/release/server. Run it with ./server/target/release/server \n\n"

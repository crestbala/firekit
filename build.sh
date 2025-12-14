#!/bin/bash
set -e

echo "📦 Building frontend..."
cd client
if [ -e "build" ]; then
    rm -rf build
fi
# npm install
npm run build

echo "🦀 Building backend..."
cd ../server
cargo run --release

echo -e "\n\n✅ Build complete! Your software executable is in server/target/release/server. Run it with ./server/target/release/server \n\n"

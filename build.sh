#!/bin/bash
set -e

echo "📦 Building frontend..."
cd client
npm install
npm run build

echo "🦀 Building backend..."
cd ../server
cargo build --release

echo -e "\n\n✅ Build complete! Your software executable is in server/target/release/server. Run it with ./server/target/release/server \n\n"

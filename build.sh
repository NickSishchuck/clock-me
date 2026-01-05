#!/bin/bash
set -e

name="clock-me"
version="0.1.0"

build_dir="build"
install_dir="install"

rm -rf "$build_dir" "$install_dir"
mkdir -p "$build_dir/$name-$version"
mkdir -p "$install_dir"

cp -r \
  Cargo.toml \
  Cargo.lock \
  src \
  "$build_dir/$name-$version/"

cd "$build_dir"
tar -czvf "$name-$version.tar.gz" "$name-$version"
mv "$name-$version.tar.gz" "../$install_dir/"
cd ..

cp PKGBUILD "$install_dir/"

rm -rf "$build_dir"

echo "Ready. Run:"
echo "  cd $install_dir && makepkg -si"

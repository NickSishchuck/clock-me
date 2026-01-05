# Maintainer: Mykyta Syshchuk (mikiitasisuk@gmail.com)
pkgname=clock-me
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple CLI time tracker"
arch=('x86_64')
url=""
license=('GPL-3.0-or-later')
depends=()
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}

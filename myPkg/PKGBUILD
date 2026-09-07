# Maintainer: Github repo:
pkgname=multitool
pkgver=0.1.0
pkgrel=1
pkgdesc="Boîte à outils en ligne de commande (contient calc et d'autres modules)"
arch=('x86_64')
url="https://localhost"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')

build() {
  cd "$srcdir/.."
  cargo build --release --locked
}

package() {
  cd "$srcdir/.."
  install -Dm755 "target/release/multitool" "$pkgdir/usr/bin/multitool"
}
# Maintainer: OGabrielPereira ogabrielpereira@pm.me
pkgname=linus-coach
pkgver=1.0.0
pkgrel=1
pkgdesc="Are you sad and unmotivated to code? linus-coach can help you with that."
arch=("x86_64")
url="https://www.github.com/OGabrielPereira/linus-coach"
license=('CC-O-1.0')
makedepends=(rust)
provides=(linus)
source=("git+$url")
md5sums=('SKIP')

build() {
	cd "$pkgname"
	cargo build
}

package() {
	cd "$pkgname"
	install -Dm755 target/debug/linus $pkgdir/usr/bin/linus
	install -Dm644 LICENSE $pkgdir/usr/share/$pkgname/LICENSE
}

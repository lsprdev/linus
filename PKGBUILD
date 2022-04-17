# Maintainer: OGabrielPereira ogabrielpereira@pm.me
pkgname=linus
pkgver=1.0.1
pkgrel=1
pkgdesc="Are you sad and unmotivated to code? linus can help you with that."
arch=("x86_64")
url="https://www.github.com/OGabrielPereira/linus"
license=('GPL-3.0')
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
	install -Dm644 linus.1 $pkgdir/usr/share/man/man1/linus.1
}

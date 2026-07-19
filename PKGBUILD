# Maintainer: Taha Moosavi <taha.moosavi.taha@gmail.com>
pkgname=autopilot-rs
pkgver=0.1.0.beta
pkgrel=1
pkgdesc="AutoPilot-rs runs automation jobs based on conditions like WiFi, Bluetooth, battery, CPU usage, and more."
arch=('x86_64')
url="https://github.com/streamtechteam/autopilot-rs"
license=('MIT')
provides=("$pkgname")
conflicts=("$pkgname")
depends=('systemd-libs' 'libxcb' 'libxrandr' 'libx11' 'libxau')
makedepends=('rust' 'cargo' 'libxcb' 'libxrandr' 'libxau' 'libx11' 'git')
source=("$pkgname-$pkgver.tar.gz::https://github.com/streamtechteam/auto_pilot_rs/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=("SKIP")

build() {
  # Build from the directory where PKGBUILD is located
  cd "$startdir/src/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  # Reference the binary using the absolute start directory
  install -Dm755 "$startdir/src/$pkgname-$pkgver/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  # If you have a license file, it's good practice to include it:
  install -Dm644 "$startdir/src/$pkgname-$pkgver/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

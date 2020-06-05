# gstreamer-rs [![crates.io](https://img.shields.io/crates/v/gstreamer.svg)](https://crates.io/crates/gstreamer) [![pipeline status](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/badges/master/pipeline.svg)](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/commits/master)

[GStreamer](https://gstreamer.freedesktop.org/) bindings for Rust.
Documentation can be found [here](https://slomo.pages.freedesktop.org/rustdocs/gstreamer/gstreamer/).

These bindings are providing a safe API that can be used to interface with
GStreamer, e.g. for writing GStreamer-based applications and GStreamer plugins.

The bindings are mostly autogenerated with [gir](https://github.com/gtk-rs/gir/)
based on the [GObject-Introspection](https://wiki.gnome.org/Projects/GObjectIntrospection/)
API metadata provided by the GStreamer project.

## Table of Contents
1. [Installation](#installation)
   1. [Linux/BSDs](#installation-linux)
   1. [macOS](#installation-macos)
   1. [Windows](#installation-windows)
1. [Getting Started](#getting-started)
1. [License](#license)
1. [Contribution](#contribution)

<a name="installation"/>

## Installation

To build the GStreamer bindings or anything depending on them, you need to
have at least GStreamer 1.8 and gst-plugins-base 1.8 installed. In addition,
some of the examples/tutorials require various GStreamer plugins to be
available, which can be found in gst-plugins-base, gst-plugins-good,
gst-plugins-bad, gst-plugins-ugly and/or gst-libav.

<a name="installation-linux"/>

### Linux/BSDs

You need to install the above mentioned packages with your distributions
package manager, or build them from source.

On Debian/Ubuntu they can be installed with

```
$ apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
      gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav libgstrtspserver-1.0-dev
```

The minimum required version of the above libraries is >= 1.8. If you
build the gstreamer-player sub-crate, or any of the examples that
depend on gstreamer-player, you must ensure that in addition to the
above packages, `libgstreamer-plugins-bad1.0-dev` is installed and
that the version is >= 1.12. See the `Cargo.toml` files for the full
details,

```
# Only if you wish to install gstreamer-player, make sure the version
# of this package is >= 1.12.
$ apt-get install libgstreamer-plugins-bad1.0-dev
```

Package names on other distributions should be similar.
Please submit a pull request with instructions for yours.

<a name="installation-macos"/>

### macOS

You can install GStreamer and the plugins via [Homebrew](https://brew.sh/) or
by installing the [binaries](https://gstreamer.freedesktop.org/data/pkg/osx/)
provided by the GStreamer project.

#### Homebrew

Homebrew only installs various plugins if explicitly enabled, so some extra
`--with-*` flags may be required.

```
$ brew install gstreamer gst-plugins-base gst-plugins-good \
      gst-plugins-bad gst-plugins-ugly gst-libav gst-rtsp-server \
      gst-editing-services --with-orc --with-libogg --with-opus \
      --with-pango --with-theora --with-libvorbis --with-libvpx \
      --enable-gtk3
```

If you wish to install the gstreamer-player sub-crate, make sure the
version of these libraries is >= 1.12. Otherwise, a version >= 1.8 is
sufficient.

#### GStreamer Binaries

You need to download the *two* `.pkg` files from the GStreamer website and
install them, e.g. `gstreamer-1.0-1.12.3-x86_64.pkg` and
`gstreamer-1.0-devel-1.12.3-x86_64.pkg`.

After installation, you also need to install `pkg-config` (e.g. via Homebrew)
and set the `PKG_CONFIG_PATH` environment variable

```
$ export PKG_CONFIG_PATH="/Library/Frameworks/GStreamer.framework/Versions/Current/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
```

<a name="installation-windows"/>

### Windows

You can install GStreamer and the plugins via [MSYS2](http://www.msys2.org/)
with `pacman` or by installing the
[binaries](https://gstreamer.freedesktop.org/data/pkg/windows/) provided by
the GStreamer project.

#### MSYS2 / pacman

```
$ pacman -S pkg-config mingw-w64-x86_64-gstreamer mingw-w64-x86_64-gst-plugins-base \
      mingw-w64-x86_64-gst-plugins-good mingw-w64-x86_64-gst-plugins-bad \
      mingw-w64-x86_64-gst-plugins-ugly mingw-w64-x86_64-gst-libav \
      mingw-w64-x86_64-gst-rtsp-server
```

If you wish to install the gstreamer-player sub-crate, make sure the
version of these libraries is >= 1.12. Otherwise, a version >= 1.8 is
sufficient.

Note that the version of `pkg-config` included in `MSYS2` is
[known to have problems](https://github.com/rust-lang/pkg-config-rs/issues/51#issuecomment-346300858)
compiling GStreamer, so you may need to install another version. One option
would be [`pkg-config-lite`](https://sourceforge.net/projects/pkgconfiglite/).

#### GStreamer Binaries

You need to download the *two* `.msi` files for your platform from the
GStreamer website and install them, e.g. `gstreamer-1.0-x86_64-1.12.3.msi` and
`gstreamer-1.0-devel-x86_64-1.12.3.msi`.

After installation, you also need to install `pkg-config` (e.g. via MSYS2 or
from [here](https://sourceforge.net/projects/pkgconfiglite/))
and set the `PKG_CONFIG_PATH` environment variable

```
$ export PKG_CONFIG_PATH="c:\\gstreamer\\1.0\\x86_64\\lib\\pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
```

<a name="getting-started"/>

## Getting Started

The API reference can be found
[here](https://slomo.pages.freedesktop.org/rustdocs/gstreamer/gstreamer/), however it is
only the Rust API reference and does not explain any of the concepts.

For getting started with GStreamer development, the best would be to follow
the [documentation](https://gstreamer.freedesktop.org/documentation/) on the
GStreamer website, especially the [Application Development
Manual](https://gstreamer.freedesktop.org/documentation/application-development/).
While being C-centric, it explains all the fundamental concepts of GStreamer
and the code examples should be relatively easily translatable to Rust. The
API is basically the same, function/struct names are the same and everything
is only more convenient (hopefully) and safer.

In addition there are
[tutorials](https://gstreamer.freedesktop.org/documentation/tutorials/) on the
GStreamer website. Many of them were ported to Rust already and the code can
be found in the
[tutorials](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/tree/master/tutorials)
directory.

Some further examples for various aspects of GStreamer and how to use it from
Rust can be found in the
[examples](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/tree/master/examples)
directory.

Various GStreamer plugins written in Rust can be found in the
[gst-plugins-rs](https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs)
repository.

<a name="license"/>

## LICENSE

gstreamer-rs and all crates contained in here are licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

GStreamer itself is licensed under the Lesser General Public License version
2.1 or (at your option) any later version:
https://www.gnu.org/licenses/lgpl-2.1.html

<a name="contribution"/>

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in gstreamer-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

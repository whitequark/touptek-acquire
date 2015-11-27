Touptek image acquirer
======================

To build:

    cargo build

To get an image, run:

    ./target/debug/touptek-acquire [camera index] [exposure time (ms)] [exposure gain (%)] [image path]

Width and height as big-endian 32-bit integers followed with the raw pixel data
in RGB0 32-bit format will be saved to `path`.

License
-------

[MIT license](LICENSE.txt)

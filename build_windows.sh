if false
then
	rustup target add i686-pc-windows-gnu
fi

cargo build --target i686-pc-windows-gnu
cargo build --target i686-pc-windows-gnu -r

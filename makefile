
setup:
	# the `thumbv7em-none-eabihf` described an embedded ARM system.
	# https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/
	rustup target add thumbv7em-none-eabihf

build:
	# by passing a --target argument we cross compile our executable for a bare
	# metal target system. Since the target system has no operating system,
	# the linker does not try to link the C runtime and our build success without
	# any errors, unlike running `cargo build` directly.
	cargo build --target thumbv7em-none-eabihf

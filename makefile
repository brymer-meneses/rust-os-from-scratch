
setup:
# the `thumbv7em-none-eabihf` described an embedded ARM system.
# https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/
	rustup target add thumbv7em-none-eabihf

# In order to recompile core libraries, cargo needs access to the rust source
# code, which we can install with rustup component add rust-src.
	rustup install nightly
	rustup component add rust-src --toolchain nightly
	rustup component add llvm-tools-preview

# a tool that simplifies the creation of bootimage
# https://github.com/rust-osdev/bootimage
	cargo install bootimage


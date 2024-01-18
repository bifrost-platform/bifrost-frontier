extern crate rustc_version;

fn main() {
	println!(
		"cargo:rustc-env=RUSTC_VERSION={}",
		rustc_version::version().expect("Failed to fetch rustc version")
	);
}

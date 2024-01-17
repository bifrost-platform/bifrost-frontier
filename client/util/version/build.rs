extern crate rustc_version;

use std::{env, fs::File, io::Write, path::Path};

const ERROR_MSG: &'static str = "Failed to generate metadata file";

fn main() {
	let version = rustc_version::version().expect(ERROR_MSG);

	create_file(
		"meta.rs",
		format!(
			"
			/// Returns compiler version.
			pub fn rustc_version() -> &'static str {{
				\"{version}\"
			}}
		",
			version = version,
		),
	);
}

fn create_file(filename: &str, data: String) {
	let out_dir = env::var("OUT_DIR").expect(ERROR_MSG);
	let dest_path = Path::new(&out_dir).join(filename);
	let mut f = File::create(&dest_path).expect(ERROR_MSG);
	f.write_all(data.as_bytes()).expect(ERROR_MSG);
}

// wengwengweng

//! Common File System Functions

use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
fn get_res_dir() -> PathBuf {

	use core_foundation::bundle;

	let bundle = bundle::CFBundle::main_bundle();
	let path = bundle
		.executable_url().expect("Cannot get executable dir")
		.to_path().expect("to_path error")
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.join("Resources");

	return path;

}

#[cfg(not(target_os = "macos"))]
fn get_res_dir() -> PathBuf {

	use std::env;

	let path = env::current_exe()
		.expect("Cannot get application dir")
		.parent().expect("Cannot get application dir")
		.to_path_buf();

	return path;

}

/// check if given file exists
pub fn exists(path: impl AsRef<Path>) -> bool {
	return validate_path(path).is_some();
}

fn validate_path(path: impl AsRef<Path>) -> Option<PathBuf> {

	let path = path.as_ref();

	if !Path::new(path).exists() {

		let with_res = get_res_dir().join(path);

		if Path::new(&with_res).exists() {
			return Some(with_res);
		} else {
			return None;
		}

	} else {

		return Some(path.to_owned());

	}

}

/// get a list of all filenames under given directory
pub fn glob(path: impl AsRef<Path>) -> Vec<String> {

	let path = path.as_ref();

	let listings = glob::glob(&format!("{}", path.display()))
		.or(glob::glob(&format!("{}", get_res_dir().join(path).display())))
		.expect(&format!("failed to read dir \"{}\"", path.display()));

	return listings
		.map(|s| s.expect("failed to glob"))
		.map(|s| s.into_os_string())
		.map(|s| s.into_string())
		.map(|s| s.expect("failed to glob"))
		.collect::<Vec<String>>();

}

/// get bytes read from given file
pub fn read_bytes(path: impl AsRef<Path>) -> Vec<u8> {

	let path = path.as_ref();
	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path.display()));

	if let Ok(content) = fs::read(&path) {
		return content;
	} else {
		panic!("failed to read file \"{}\"", path.display());
	}

}

/// get string read from given file
pub fn read_str(path: impl AsRef<Path>) -> String {

	let path = path.as_ref();
	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path.display()));

	if let Ok(content) = fs::read_to_string(&path) {
		return content;
	} else {
		panic!("failed to read file \"{}\"", path.display());
	}

}

/// get the basename of given file
pub fn basename(path: impl AsRef<Path>) -> String {

	let path = path.as_ref();
	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path.display()));

	if let Some(name) = Path::new(&path).file_stem() {
		return name.to_str().expect("failed to get basename").to_owned();
	} else {
		panic!("failed to read file \"{}\"", path.display());
	}

}


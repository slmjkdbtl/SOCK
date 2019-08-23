// wengwengweng

use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
	FileWrite(PathBuf),
	FileRead(PathBuf),
	FileBasename(PathBuf),
	FileExt(PathBuf),
	FileCopy(PathBuf, PathBuf),
	FileRemove(PathBuf),
	Rename(PathBuf),
	DirRead(PathBuf),
	DirRemove(PathBuf),
	Mkdir(PathBuf),
	GetDataDir,
	IO,
	Net,
	Image,
	Window,
	Wasm,
	Gamepad,
	Audio,
	Parse,
	Thread,
	FromUtf8,
	HTTPMessage,
	Lua,
	MaxDraw,
	GfxPop,
	Font,
	ObjLoad,
	Input,
	TexPack,
	OpenGL(String),
	Misc(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self {
			Error::FileWrite(p) => write!(f, "failed to write {}", p.display()),
			Error::FileRead(p) => write!(f, "failed to read {}", p.display()),
			Error::FileBasename(p) => write!(f, "failed to get basename for {}", p.display()),
			Error::FileExt(p) => write!(f, "failed to get extension for {}", p.display()),
			Error::FileCopy(p1, p2) => write!(f, "failed to copy {} to {}", p1.display(), p2.display()),
			Error::FileRemove(p) => write!(f, "failed to remove file {}", p.display()),
			Error::DirRemove(p) => write!(f, "failed to remove dir {}", p.display()),
			Error::DirRead(p) => write!(f, "failed to read dir {}", p.display()),
			Error::Mkdir(p) => write!(f, "failed to create directory {}", p.display()),
			Error::Rename(p) => write!(f, "failed to rename {}", p.display()),
			Error::GetDataDir => write!(f, "failed to get data dir"),
			Error::IO => write!(f, "io error"),
			Error::Net => write!(f, "network error"),
			Error::Image => write!(f, "image error"),
			Error::Window => write!(f, "window error"),
			Error::Wasm => write!(f, "wasm error"),
			Error::Gamepad => write!(f, "gamepad error"),
			Error::Audio => write!(f, "audio error"),
			Error::Parse => write!(f, "parse error"),
			Error::Thread => write!(f, "thread error"),
			Error::FromUtf8 => write!(f, "failed to convert from utf8"),
			Error::HTTPMessage => write!(f, "failed to parse http message"),
			Error::Lua => write!(f, "lua error"),
			Error::MaxDraw => write!(f, "reached maximum draw calls"),
			Error::GfxPop => write!(f, "cannot pop anymore"),
			Error::ObjLoad => write!(f, "failed to load obj"),
			Error::Font => write!(f, "font error"),
			Error::Input => write!(f, "input error"),
			Error::TexPack => write!(f, "texture packing error"),
			Error::OpenGL(s) => write!(f, "error: {}", s),
			Error::Misc(s) => write!(f, "error: {}", s),
		};
	}
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
	fn from(_: std::io::Error) -> Self {
		return Error::IO;
	}
}

impl From<std::sync::mpsc::TryRecvError> for Error {
	fn from(_: std::sync::mpsc::TryRecvError) -> Self {
		return Error::Thread;
	}
}

impl From<std::string::FromUtf8Error> for Error {
	fn from(_: std::string::FromUtf8Error) -> Self {
		return Error::FromUtf8;
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		return Error::Misc(s);
	}
}

impl From<std::ffi::OsString> for Error {
	fn from(s: std::ffi::OsString) -> Self {
		return Error::Misc(String::new());
	}
}

impl From<()> for Error {
	fn from(_: ()) -> Self {
		return Error::Misc(String::new());
	}
}

#[cfg(feature = "img")]
impl From<image::ImageError> for Error {
	fn from(_: image::ImageError) -> Self {
		return Error::Image;
	}
}

#[cfg(all(feature = "app", not(target_arch = "wasm32")))]
impl From<glutin::CreationError> for Error {
	fn from(_: glutin::CreationError) -> Self {
		return Error::Window;
	}
}

#[cfg(all(feature = "app", not(target_arch = "wasm32")))]
impl From<glutin::ContextError> for Error {
	fn from(_: glutin::ContextError) -> Self {
		return Error::Window;
	}
}

#[cfg(all(feature = "app", not(target_arch = "wasm32")))]
impl From<(glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)> for Error {
	fn from(_: (glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)) -> Self {
		return Error::Window;
	}
}

#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for Error {
	fn from(_: wasm_bindgen::JsValue) -> Self {
		return Error::Wasm;
	}
}

#[cfg(target_arch = "wasm32")]
impl From<web_sys::Element> for Error {
	fn from(_: web_sys::Element) -> Self {
		return Error::Wasm;
	}
}

// #[cfg(target_arch = "wasm32")]
// impl From<js_sys::Object> for Error {
// 	fn from(_: js_sys::Object) -> Self {
// 		return Error::Wasm;
// 	}
// }

#[cfg(feature = "audio")]
impl From<rodio::decoder::DecoderError> for Error {
	fn from(_: rodio::decoder::DecoderError) -> Self {
		return Error::Audio;
	}
}

#[cfg(all(not(target_os = "ios"), not(target_os = "android"), not(target_arch = "wasm32")))]
impl From<gilrs::Error> for Error {
	fn from(_: gilrs::Error) -> Self {
		return Error::Thread;
	}
}

#[cfg(feature = "img")]
impl From<tobj::LoadError> for Error {
	fn from(_: tobj::LoadError) -> Self {
		return Error::ObjLoad;
	}
}

#[cfg(feature = "http")]
impl From<url::ParseError> for Error {
	fn from(_: url::ParseError) -> Self {
		return Error::Net;
	}
}

#[cfg(feature = "http")]
impl From<httparse::Error> for Error {
	fn from(_: httparse::Error) -> Self {
		return Error::HTTPMessage;
	}
}

#[cfg(all(feature = "http", not(target_arch = "wasm32")))]
impl From<native_tls::Error> for Error {
	fn from(_: native_tls::Error) -> Self {
		return Error::Net;
	}
}

#[cfg(all(feature = "http", not(target_arch = "wasm32")))]
impl From<native_tls::HandshakeError<std::net::TcpStream>> for Error {
	fn from(_: native_tls::HandshakeError<std::net::TcpStream>) -> Self {
		return Error::Net;
	}
}
#[cfg(feature = "ase")]
impl From<serde_json::error::Error> for Error {
	fn from(_: serde_json::error::Error) -> Self {
		return Error::Parse;
	}
}


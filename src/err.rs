// wengwengweng

use std::fmt;

#[derive(Debug)]
pub enum Error {
	Fs(String),
	IO(std::io::Error),
	Net(String),
	Image(String),
	Window(String),
	Wasm(String),
	Audio(String),
	Thread(String),
	Lua(String),
	Gfx(String),
	Obj(String),
	Input(String),
	OpenGL(String),
	Misc(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self {
			Error::Fs(s) => write!(f, "fs error: {}", s),
			Error::IO(e) => write!(f, "io error: {}", e),
			Error::Net(s) => write!(f, "network error: {}", s),
			Error::Image(s) => write!(f, "image error: {}", s),
			Error::Window(s) => write!(f, "window error: {}", s),
			Error::Wasm(s) => write!(f, "wasm error: {}", s),
			Error::Audio(s) => write!(f, "audio error: {}", s),
			Error::Thread(s) => write!(f, "thread error: {}", s),
			Error::Lua(s) => write!(f, "lua error: {}", s),
			Error::Gfx(s) => write!(f, "gfx error: {}", s),
			Error::Obj(s) => write!(f, "obj error: {}", s),
			Error::Input(s) => write!(f, "input error: {}", s),
			Error::OpenGL(s) => write!(f, "opengl error: {}", s),
			Error::Misc(s) => write!(f, "misc error: {}", s),
		};
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		return Error::IO(err);
	}
}

impl From<std::sync::mpsc::TryRecvError> for Error {
	fn from(_: std::sync::mpsc::TryRecvError) -> Self {
		return Error::Thread(format!("failed to receive from another thread"));
	}
}

impl From<std::string::FromUtf8Error> for Error {
	fn from(_: std::string::FromUtf8Error) -> Self {
		return Error::Misc("failed to convert utf8 to string".into());
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

	fn from(err: image::ImageError) -> Self {

		use image::ImageError::*;

		let msg = match err {
			FormatError(..) => "format error".into(),
			DimensionError => "incorrect image dimension".into(),
			UnsupportedError(fmt) => format!("{} not supported", fmt),
			UnsupportedColor(c) => format!("{:?} not supported", c),
			NotEnoughData => "not enough data provided to decode the image".into(),
			IoError(..) => "io".into(),
			ImageEnd => "the end of image has been reached".into(),
			InsufficientMemory => "not enough memory".into(),
		};

		return Error::Image(msg);

	}

}

#[cfg(all(feature = "app", not(web)))]
impl From<glutin::CreationError> for Error {

	fn from(err: glutin::CreationError) -> Self {

		use glutin::CreationError::*;

		let msg = match err {
			OsError(..) => "".into(),
			NotSupported(..) => "".into(),
			NoBackendAvailable(..) => "no backend available".into(),
			RobustnessNotSupported => "robustness not supported".into(),
			OpenGlVersionNotSupported => "opengl version not supported".into(),
			NoAvailablePixelFormat => "pixel format not available".into(),
			PlatformSpecific(s) => format!("{}", s),
			Window(..) => "window creation error".into(),
			CreationErrors(..) => "window creation error".into(),
		};

		return Error::Window(msg);

	}

}

#[cfg(all(feature = "app", not(web)))]
impl From<glutin::ContextError> for Error {
	fn from(_: glutin::ContextError) -> Self {
		return Error::Window("failed to create window context".into());
	}
}

#[cfg(all(feature = "app", not(web)))]
impl From<(glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)> for Error {
	fn from(_: (glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)) -> Self {
		return Error::Window("failed to create window context".into());
	}
}

#[cfg(all(feature = "app", web))]
impl From<stdweb::web::error::InvalidCharacterError> for Error {
	fn from(_: stdweb::web::error::InvalidCharacterError) -> Self {
		return Error::Wasm(format!("invalid character"));
	}
}

// TODO: why this doesn't work
#[cfg(all(feature = "app", web))]
impl From<stdweb::serde::ConversionError> for Error {
	fn from(_: stdweb::serde::ConversionError) -> Self {
		return Error::Wasm(format!("conversion"));
	}
}

#[cfg(feature = "audio")]
impl From<rodio::decoder::DecoderError> for Error {
	fn from(_: rodio::decoder::DecoderError) -> Self {
		return Error::Audio("failed to decode".into());
	}
}

#[cfg(all(feature = "app", not(mobile), not(web)))]
impl From<gilrs::Error> for Error {
	fn from(_: gilrs::Error) -> Self {
		return Error::Input("gamepad error".into());
	}
}

#[cfg(feature = "img")]
impl From<tobj::LoadError> for Error {

	fn from(err: tobj::LoadError) -> Self {

		use tobj::LoadError::*;

		let msg = match err {
			OpenFileFailed => "".into(),
			ReadError => "".into(),
			UnrecognizedCharacter => "unrecognized character".into(),
			PositionParseError => "failed to parse positions".into(),
			NormalParseError => "failed to parse normals".into(),
			TexcoordParseError => "failed to parse tex coords".into(),
			FaceParseError => "failed to parse faces".into(),
			MaterialParseError => "failed to parse material".into(),
			InvalidObjectName => "invalid object name".into(),
			GenericFailure => "unknown error".into(),
		};

		return Error::Obj(msg);

	}

}

#[cfg(feature = "http")]
impl From<url::ParseError> for Error {
	fn from(_: url::ParseError) -> Self {
		return Error::Net("failed to parse url".into());
	}
}

#[cfg(feature = "http")]
impl From<httparse::Error> for Error {

	fn from(err: httparse::Error) -> Self {

		use httparse::Error::*;

		let msg = match err {
			HeaderName => "invalid header name".into(),
			HeaderValue => "invalid header value".into(),
			NewLine => "invalid byte in new line".into(),
			Status => "invalid response status".into(),
			Token => "invalid byte where token is required".into(),
			TooManyHeaders => "too many headers".into(),
			Version => "invalid http version".into(),
		};

		return Error::Net(msg);

	}

}

#[cfg(all(feature = "http", not(mobile), not(web)))]
impl From<native_tls::Error> for Error {
	fn from(_: native_tls::Error) -> Self {
		return Error::Net("tls error".into());
	}
}

#[cfg(all(feature = "http", not(mobile), not(web)))]
impl From<native_tls::HandshakeError<std::net::TcpStream>> for Error {
	fn from(_: native_tls::HandshakeError<std::net::TcpStream>) -> Self {
		return Error::Net("tls error".into());
	}
}


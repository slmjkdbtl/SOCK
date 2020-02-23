// wengwengweng

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use crate::Error;
use crate::Result;

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scheme {
	HTTP,
	HTTPS,
}

impl Scheme {
	pub fn port(&self) -> u16 {
		return match self {
			Scheme::HTTP => HTTP_PORT,
			Scheme::HTTPS => HTTPS_PORT,
		};
	}
}

impl FromStr for Scheme {

	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		return match s {
			"http" => Ok(Scheme::HTTP),
			"https" => Ok(Scheme::HTTPS),
			_ => Err(format!("failed to parse scheme from {}", s)),
		};
	}

}

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum Method {
	GET,
	POST,
	PUT,
	DELETE,
}

impl Method {
	pub fn as_str(&self) -> &'static str {
		use Method::*;
		return match self {
			GET => "GET",
			POST => "POST",
			PUT => "PUT",
			DELETE => "DELETE",
		};
	}
}

impl FromStr for Method {

	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		return match s {
			"GET" => Ok(Method::GET),
			"POST" => Ok(Method::POST),
			_ => Err(format!("failed to parse http method from {}", s)),
		};
	}

}

#[derive(Clone, Copy, Debug)]
pub enum Version {
	V10,
	V11,
	V20,
}

impl Version {
	pub fn as_str(&self) -> &'static str {
		return match self {
			Version::V10 => "HTTP/1.0",
			Version::V11 => "HTTP/1.1",
			Version::V20 => "HTTP/2.0",
		};
	}
}

impl From<u8> for Version {
	fn from(v: u8) -> Self {
		return match v {
			1 => Version::V10,
			11 => Version::V11,
			2 => Version::V20,
			_ => Version::V10,
		};
	}
}

impl FromStr for Version {

	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		return match s {
			"HTTP/1.0" => Ok(Version::V10),
			"HTTP/1.1" => Ok(Version::V11),
			"HTTP/2.0" => Ok(Version::V20),
			_ => Err(format!("failed to parse http version from {}", s)),
		};
	}

}

macro_rules! gen_content_type {

	($($name:ident($($ext:expr),*) => $msg:expr),*$(,)?) => {

		#[derive(Clone, Copy)]
		pub enum ContentType {
			$(
				$name,
			)*
		}

		impl ContentType {

			pub fn as_str(&self) -> &'static str {
				return match self {
					$(
						ContentType::$name => $msg,
					)*
				}
			}

			pub fn from_ext(ext: &str) -> Option<Self> {
				return match ext {
					$(
						$(
							$ext => Some(ContentType::$name),
						)*
					)*
					_ => None,
				};
			}

			pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {

				let path = path.as_ref();
				let ext = path
					.extension()?
					.to_os_string()
					.into_string()
					.ok()?;

				return Self::from_ext(&ext);

			}

		}

	}

}

gen_content_type! {
	Text("txt") => "text/plain; charset=utf-8",
	HTML("html", "htm") => "text/html; charset=utf-8",
	XML("xml") => "text/xml; charset=utf-8",
	CSV("csv") => "text/csv; charset=utf-8",
	Markdown("md", "markdown") => "text/markdown; charset=utf-8",
	CSS("css") => "text/css; charset=utf-8",
	PNG("png") => "image/png",
	JPEG("jpg", "jpeg") => "image/jpeg",
	GIF("gif") => "image/gif",
	PDF("pdf") => "application/pdf",
	JavaScript("js") => "application/javascript",
	JSON("json") => "application/json",
	GraphQL("graphql") => "application/graphql",
	ZIP("zip") => "application/zip",
	MP3("mp3") => "audio/mpeg",
	OGG("ogg") => "audio/ogg",
	WAV("wav") => "audio/wav",
	MIDI("midi") => "audio/midi",
	TTF("ttf") => "font/ttf",
	OTF("otf") => "font/otf",
	WOFF("woff") => "font/woff",
	WOFF2("woff2") => "font/woff2",
	MP4("mp4") => "video/mp4",
	MOV("mov") => "video/quicktime",
}

#[derive(Clone, Debug)]
pub enum Auth {
	Basic(String),
	Bearer(String),
	Digest(String),
}

impl ToString for Auth {
	fn to_string(&self) -> String {
		return match self {
			Auth::Basic(s) => format!("Basic {}", s),
			Auth::Bearer(s) => format!("Bearer {}", s),
			Auth::Digest(s) => format!("Digest {}", s),
		};
	}
}

#[derive(Clone, Copy, Debug)]
pub enum StatusRange {
	Info,
	Success,
	Redirect,
	ClientErr,
	ServerErr,
}

macro_rules! gen_status {

	($($code:expr, $name:ident => $reason:expr),*$(,)?) => {

		#[derive(Clone, Copy, Debug)]
		pub enum Status {
			$(
				$name,
			)*
		}

		impl Status {

			pub fn from_code(code: u16) -> Result<Self> {
				return match code {
					$(
						$code => Ok(Status::$name),
					)*
					_ => Err(format!("failed to get status from code {}", code)),
				};
			}

			pub fn reason(&self) -> &'static str {
				return match self {
					$(
						Status::$name => $reason,
					)*
				};
			}

			pub fn code(&self) -> u16 {
				return match self {
					$(
						Status::$name => $code,
					)*
				};
			}

			pub fn success(&self) -> bool {
				let c = self.code();
				return c >= 200 && c <= 299;
			}

			pub fn redirect(&self) -> bool {
				let c = self.code();
				return c >= 300 && c <= 399;
			}

			pub fn client_err(&self) -> bool {
				let c = self.code();
				return c >= 400 && c <= 499;
			}

			pub fn server_err(&self) -> bool {
				let c = self.code();
				return c >= 500 && c <= 599;
			}

		}

	}

}

gen_status! {
	100, Continue => "Continue",
	101, SwitchingProtocols => "Switching Protocols",
	102, Processing => "Processing",
	200, Ok => "OK",
	201, Created => "Created",
	202, Accepted => "Accepted",
	203, NonAuthoritativeInformation => "Non-Authoritative Information",
	204, NoContent => "No Content",
	205, ResetContent => "Reset Content",
	206, PartialContent => "Partial Content",
	207, MultiStatus => "Multi-Status",
	208, AlreadyReported => "Already Reported",
	226, ImUsed => "IM Used",
	300, MultipleChoices => "Multiple Choices",
	301, MovedPermanently => "Moved Permanently",
	302, Found => "Found",
	303, SeeOther => "See Other",
	304, NotModified => "Not Modified",
	305, UseProxy => "Use Proxy",
	307, TemporaryRedirect => "Temporary Redirect",
	308, PermanentRedirect => "Permanent Redirect",
	400, BadRequest => "Bad Request",
	401, Unauthorized => "Unauthorized",
	402, PaymentRequired => "Payment Required",
	403, Forbidden => "Forbidden",
	404, NotFound => "Not Found",
	405, MethodNotAllowed => "Method Not Allowed",
	406, NotAcceptable => "Not Acceptable",
	407, ProxyAuthenticationRequired => "Proxy Authentication Required",
	408, RequestTimeout => "Request Timeout",
	409, Conflict => "Conflict",
	410, Gone => "Gone",
	411, LengthRequired => "Length Required",
	412, PreconditionFailed => "Precondition Failed",
	413, PayloadTooLarge => "Payload Too Large",
	414, UriTooLong => "URI Too Long",
	415, UnsupportedMediaType => "Unsupported Media Type",
	416, RangeNotSatisfiable => "Range Not Satisfiable",
	417, ExpectationFailed => "Expectation Failed",
	418, ImATeapot => "I'm a teapot",
	421, MisdirectedRequest => "Misdirected Request",
	422, UnprocessableEntity => "Unprocessable Entity",
	423, Locked => "Locked",
	424, FailedDependency => "Failed Dependency",
	426, UpgradeRequired => "Upgrade Required",
	428, PreconditionRequired => "Precondition Required",
	429, TooManyRequests => "Too Many Requests",
	431, RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
	451, UnavailableForLegalReasons => "Unavailable For Legal Reasons",
	500, InternalServerError => "Internal Server Error",
	501, NotImplemented => "Not Implemented",
	502, BadGateway => "Bad Gateway",
	503, ServiceUnavailable => "Service Unavailable",
	504, GatewayTimeout => "Gateway Timeout",
	505, HttpVersionNotSupported => "HTTP Version Not Supported",
	506, VariantAlsoNegotiates => "Variant Also Negotiates",
	507, InsufficientStorage => "Insufficient Storage",
	508, LoopDetected => "Loop Detected",
	510, NotExtended => "Not Extended",
	511, NetworkAuthenticationRequired => "Network Authentication Required"
}


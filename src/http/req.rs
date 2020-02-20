// wengwengweng

use url::Url;

use crate::Result;

use super::*;

#[derive(Clone)]
pub struct Request {
	method: Method,
	scheme: Scheme,
	version: Version,
	host: String,
	path: String,
	port: u16,
	headers: HeaderMap,
	body: Body,
}

impl Request {

	pub fn parse(buf: &[u8]) -> Result<Self> {

		let mut headers = [httparse::EMPTY_HEADER; 128];
		let mut req = httparse::Request::new(&mut headers);

		let body_pos = match req
			.parse(&buf)
			.map_err(|_| format!("failed to parse request"))? {
			httparse::Status::Complete(len) => len,
			httparse::Status::Partial => return Err(format!("incomplete request message")),
		};

		let method = req.method
			.ok_or(format!("failed to parse request method"))?
			.parse::<Method>()?;

		let path = req.path
			.ok_or(format!("failed to parse request path"))?;

		let version = req.version
			.ok_or(format!("failed to parse request version"))?;

		let body = &buf[body_pos..];

		return Ok(Self {
			method: method,
			version: version.into(),
			scheme: Scheme::HTTP,
			host: String::new(),
			path: path.to_owned(),
			port: 80,
			headers: HeaderMap::new(),
			body: Body::from_bytes(body),
		});

	}

	pub fn from_url(method: Method, url: &str) -> Result<Self> {

		let url = Url::parse(url).map_err(|_| format!("failed to parse url"))?;
		let scheme = url
			.scheme()
			.parse::<Scheme>()
			.map_err(|_| format!("failed to parse url scheme"))?;

		let host = url
			.host_str()
			.ok_or(format!("failed to parse url host addr"))?;

		let path = url.path();
		let mut headers = HeaderMap::new();

		headers.set(Header::Host, host);

		return Ok(Self {
			method: method,
			version: Version::V10,
			scheme: scheme,
			host: host.to_owned(),
			path: path.to_owned(),
			port: scheme.port(),
			headers: headers,
			body: Body::empty(),
		});

	}

	pub fn set_scheme(&mut self, s: Scheme) {
		self.scheme = s;
	}

	pub fn set_method(&mut self, m: Method) {
		self.method = m;
	}

	pub fn set_host(&mut self, h: &str) {
		self.host = h.to_owned();
	}

	pub fn set_path(&mut self, p: &str) {
		self.path = p.to_owned();
	}

	pub fn set_port(&mut self, p: u16) {
		self.port = p;
	}

	pub fn set_header(&mut self, key: Header, value: &str) {
		self.headers.set(key, value);
	}

	pub fn set_body(&mut self, data: Body) {
		self.body = data;
	}

	pub fn scheme(&self) -> Scheme {
		return self.scheme;
	}

	pub fn version(&self) -> Version {
		return self.version;
	}

	pub fn method(&self) -> Method {
		return self.method;
	}

	pub fn host(&self) -> &str {
		return &self.host;
	}

	pub fn path(&self) -> &str {
		return &self.path;
	}

	pub fn port(&self) -> u16 {
		return self.port;
	}

	pub fn headers(&self) -> &HeaderMap {
		return &self.headers;
	}

	pub fn body(&self) -> &Body {
		return &self.body;
	}

	pub fn message(&self) -> Vec<u8> {

		let mut m = Vec::new();

		m.extend_from_slice(&format!("{} {} {}", self.method().to_string(), self.path(), self.version().to_string()).as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.headers.to_string().as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.body.as_bytes());

		return m;

	}

}


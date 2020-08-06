// wengwengweng

use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::*;
use math::*;
use input::*;
use window::*;
use conf::CanvasRoot;

// TODO: gamepad input

/// The Window Context
pub struct Window {
	gl: Rc<glow::Context>,
	canvas: web_sys::HtmlCanvasElement,
	window: web_sys::Window,
	document: web_sys::Document,
	render_loop: Option<glow::RenderLoop>,
	pressed_keys: HashSet<Key>,
	pressed_mouse: HashSet<Mouse>,
	styles: HashMap<&'static str, String>,
	mouse_pos: Vec2,
	width: i32,
	height: i32,
	cursor_hidden: bool,
	cursor_locked: bool,
	prev_cursor: CursorIcon,
	title: String,
}

fn build_styles(map: &HashMap<&'static str, String>) -> String {
	let mut styles = String::new();
	for (prop, val) in map {
		styles.push_str(&format!("{}: {};", prop, val));
	}
	return styles;
}

impl Window {

	pub(crate) fn new(conf: &conf::Conf) -> Result<Self> {

		let window = web_sys::window()
			.ok_or_else(|| format!("failed to get window"))?;

		let document = window
			.document()
			.ok_or_else(|| format!("failed to get document"))?;

		let canvas = document
			.create_element("canvas")
			.map_err(|_| format!("failed to create canvas"))?
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.map_err(|_| format!("failed to create canvas"))?;

		let mut styles = hmap![];

		canvas.set_width(conf.width as u32);
		canvas.set_height(conf.height as u32);
		canvas.set_attribute("alt", &conf.title);
		canvas.set_attribute("tabindex", "0");
		styles.insert("outline", "none".to_string());

		match conf.canvas_root {
			CanvasRoot::Body => {
				document
					.body()
					.ok_or_else(|| format!("failed to get body"))?
					.append_child(&canvas)
					.map_err(|_| format!("failed to append canvas"))?;
			},
			CanvasRoot::Element(query) => {
				document
					.query_selector(query)
					.map_err(|_| format!("failed to get {}", query))?
					.ok_or_else(|| format!("failed to get {}", query))?
					.append_child(&canvas)
					.map_err(|_| format!("failed to append canvas"))?;
			},
		};

		canvas.focus();

		if conf.cursor_hidden {
			styles.insert("cursor", "none".to_string());
		}

		if conf.cursor_locked {
			canvas.request_pointer_lock();
		}

		if conf.fullscreen {
			canvas.request_fullscreen();
		}

		let render_loop = glow::RenderLoop::from_request_animation_frame();

		canvas.set_attribute("style", &build_styles(&styles));

		let mut config = web_sys::WebGlContextAttributes::new();

		config.antialias(false);
		config.depth(true);
		config.stencil(true);

		let webgl_ctx = canvas
			.get_context_with_context_options("webgl2", &config)
			.map_err(|_| format!("failed to get webgl context"))?
			.ok_or_else(|| format!("failed to get webgl context"))?
			.dyn_into::<web_sys::WebGl2RenderingContext>()
			.map_err(|_| format!("failed to get webgl context"))?;

		let gl = glow::Context::from_webgl2_context(webgl_ctx);

		return Ok(Self {
			gl: Rc::new(gl),
			window: window,
			document: document,
			canvas: canvas,
			render_loop: Some(render_loop),
			pressed_keys: hset![],
			pressed_mouse: hset![],
			mouse_pos: vec2!(),
			styles: styles,
			width: conf.width,
			height: conf.height,
			cursor_hidden: conf.cursor_hidden,
			cursor_locked: conf.cursor_locked,
			prev_cursor: CursorIcon::Normal,
			title: conf.title.to_string(),
		});

	}

}

impl Window {

	pub(crate) fn gl(&self) -> &Rc<glow::Context> {
		return &self.gl;
	}

	pub(crate) fn swap(&self) -> Result<()> {
		return Ok(());
	}

	pub fn focused(&self) -> bool {
		return self.document.active_element() == Some(self.canvas.clone().into());
	}

	/// check if a key is currently pressed
	pub fn key_down(&self, k: Key) -> bool {
		return self.pressed_keys.contains(&k);
	}

	/// get current ([KeyMod](input::KeyMod))
	pub fn key_mods(&self) -> KeyMod {
		return KeyMod {
			shift: self.key_down(Key::LShift) || self.key_down(Key::RShift),
			ctrl: self.key_down(Key::LCtrl) || self.key_down(Key::RCtrl),
			alt: self.key_down(Key::LAlt) || self.key_down(Key::RAlt),
			meta: self.key_down(Key::LMeta) || self.key_down(Key::RMeta),
		};
	}

	/// check if a mouse button is currently pressed
	pub fn mouse_down(&self, m: Mouse) -> bool {
		return self.pressed_mouse.contains(&m);
	}

	/// check if a gamepad button is currently pressed
	pub fn gamepad_down(&self, id: GamepadID, b: GamepadButton) -> bool {
		return false;
	}

	/// get gamepad axis position
	pub fn gamepad_axis(&self, id: GamepadID, b: GamepadAxis) -> Vec2 {
		return vec2!(0);
	}

	/// get current dpi
	pub fn dpi(&self) -> f32 {
		return 1.0;
	}

	/// get current window width
	pub fn width(&self) -> i32 {
		return self.width;
	}

	/// get current window height
	pub fn height(&self) -> i32 {
		return self.height;
	}

	/// get a touch position
	pub fn touch_pos(&self, _: TouchID) -> Option<Vec2> {
		return Some(self.mouse_pos);
	}

	/// get current mouse position
	pub fn mouse_pos(&self) -> Vec2 {
		return self.mouse_pos;
	}

	/// set mouse position
	pub fn set_mouse_pos(&mut self, _: Vec2) -> Result<()> {
		return Ok(());
	}

	/// set fullscreen
	pub fn set_fullscreen(&mut self, b: bool) {
		if b {
			self.canvas.request_fullscreen();
		} else {
			self.document.exit_fullscreen();
		}
	}

	/// check if is fullscreen
	pub fn is_fullscreen(&self) -> bool {
		return self.document.fullscreen_element().is_some();
	}

	fn add_style(&mut self, prop: &'static str, val: &str) {
		self.styles.insert(prop, val.to_string());
		self.canvas.set_attribute("style", &build_styles(&self.styles));
	}

	/// set cursor hidden
	pub fn set_cursor_hidden(&mut self, b: bool) {
		if b {
			self.add_style("cursor", "none");
		} else {
			self.add_style("cursor", &format!("cursor: {}", self.prev_cursor.to_web()));
		}
		self.cursor_hidden = b;
	}

	/// check if is cursor hidden
	pub fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	/// set cursor locked
	pub fn set_cursor_locked(&mut self, b: bool) {
		if b {
			self.canvas.request_pointer_lock();
		} else {
			self.document.exit_pointer_lock();
		}
		self.cursor_locked = b;
	}

	/// check if is cursor locked
	pub fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	/// set window title
	pub fn set_title(&mut self, s: &str) {
		self.title = s.to_owned();
		self.canvas.set_attribute("alt", s);
	}

	/// get window title
	pub fn title(&self) -> &str {
		return &self.title;
	}

	/// set cursor icon
	pub fn set_cursor(&mut self, c: CursorIcon) {
		self.prev_cursor = c;
		self.canvas.set_attribute("style", &format!("cursor: {}", c.to_web()));
	}

	/// quit
	pub fn quit(&mut self) {}

	pub(crate) fn run(
		mut self,
		mut handle: impl FnMut(&mut Self, WindowEvent) -> Result<()> + 'static,
	) -> Result<()> {

		use input::Event::*;
		use std::sync::mpsc;

		let (event_tx, event_rx) = mpsc::channel();

		enum WebEvent {
			KeyPress(web_sys::KeyboardEvent),
			KeyRelease(web_sys::KeyboardEvent),
			MouseMove(web_sys::MouseEvent),
			MousePress(web_sys::MouseEvent),
			MouseRelease(web_sys::MouseEvent),
			Wheel(web_sys::WheelEvent),
			Fullscreen(web_sys::Event),
		}

		macro_rules! add_event {

			($name:expr, $ty:ty, $t:ident) => {

				let event_tx_2 = event_tx.clone();

				let handler = Closure::wrap(Box::new((move |e: $ty| {
					// TODO: I want to prevent stuff like space / arrow keys scrolling, but this also prevents browser default keys like refresh / tab switch, not good
					e.prevent_default();
					event_tx_2.send(WebEvent::$t(e));
				})) as Box<dyn FnMut(_)>);

				self.canvas
					.add_event_listener_with_callback($name, handler.as_ref().unchecked_ref())
					.map_err(|_| format!("failed to add event {}", $name))?;

				handler.forget();

			};

		}

		add_event!("keydown", web_sys::KeyboardEvent, KeyPress);
		add_event!("keyup", web_sys::KeyboardEvent, KeyRelease);
		add_event!("mousemove", web_sys::MouseEvent, MouseMove);
		add_event!("mousedown", web_sys::MouseEvent, MousePress);
		add_event!("mouseup", web_sys::MouseEvent, MouseRelease);
		add_event!("wheel", web_sys::WheelEvent, Wheel);
		add_event!("fullscreenchange", web_sys::Event, Fullscreen);

		use glow::HasRenderLoop;

		let render_loop = match self.render_loop.take() {
			Some(l) => l,
			None => return Ok(()),
		};

		render_loop.run(move |running: &mut bool| {

			let res = || -> Result<()> {

				let mut events = vec![];

				for e in event_rx.try_iter() {

					match e {

						WebEvent::KeyPress(e) => {
							if let Some(k) = Key::from_web(&e) {
								events.push(KeyPressRepeat(k));
								if !self.key_down(k) {
									events.push(KeyPress(k));
								}
								self.pressed_keys.insert(k);
							}
							let key = e.key();
							if key.len() == 1 {
								if let Some(ch) = key.chars().next() {
									events.push(CharInput(ch));
								}
							}
						},

						WebEvent::KeyRelease(e) => {
							if let Some(k) = Key::from_web(&e) {
								self.pressed_keys.remove(&k);
								events.push(KeyRelease(k));
							}
						},

						WebEvent::MouseMove(e) => {

							let (w, h) = (self.width as f32, self.height as f32);
							// TODO: doesn't work on firefox, weird on safari
							let mpos = vec2!(e.offset_x(), e.offset_y());
							let mpos = vec2!(mpos.x - w / 2.0, h / 2.0 - mpos.y as f32);
							let prev_mpos = self.mouse_pos;

							self.mouse_pos = mpos;

							if prev_mpos != vec2!(0) {
								events.push(MouseMove(mpos - prev_mpos));
							}

						},

						WebEvent::MousePress(_) => {
							self.canvas.focus();
							self.pressed_mouse.insert(Mouse::Left);
							events.push(MousePress(Mouse::Left));
						},

						WebEvent::MouseRelease(_) => {
							self.pressed_mouse.remove(&Mouse::Left);
							events.push(MouseRelease(Mouse::Left));
						},

						WebEvent::Wheel(e) => {
							events.push(Wheel(vec2!(-e.delta_x(), e.delta_y()), input::ScrollPhase::Solid));
						},

						WebEvent::Fullscreen(e) => {

							let cw = self.canvas.width();
							let ch = self.canvas.height();

							let (w, h) = if self.is_fullscreen() {

								let ww = self.window
									.inner_width()
									.map_err(|_| format!("failed to get window size"))?
									.as_f64()
									.ok_or_else(|| format!("failed to get window size"))?;

								let wh = self.window
									.inner_height()
									.map_err(|_| format!("failed to get window size"))?
									.as_f64()
									.ok_or_else(|| format!("failed to get window size"))?;

								let c_aspect = cw as f32 / ch as f32;
								let w_aspect = ww as f32 / wh as f32;

								if c_aspect > w_aspect {
									(ww as i32, (ww as f32 / c_aspect) as i32)
								} else {
									((wh as f32 * c_aspect) as i32, wh as i32)
								}

							} else {
								(cw as i32, ch as i32)
							};

							self.width = w;
							self.height = h;
							events.push(Resize(w, h));

						},

					}

				}

				for e in events {
					handle(&mut self, WindowEvent::Input(e))?;
				}

				handle(&mut self, WindowEvent::Frame)?;

				return Ok(());

			}();

			if let Err(err) = res {
				elog!("{}", err);
			}

		});

		return Ok(());

	}

	/// toggle fullscreen state
	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	/// toggle cursor hidden state
	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	/// toggle cursor lock state
	pub fn toggle_cursor_locked(&mut self) {
		self.set_cursor_locked(!self.is_cursor_locked());
	}

	/// minimize window
	pub fn minimize(&self) {}

}

impl CursorIcon {
	fn to_web(&self) -> &'static str {
		return match self {
			CursorIcon::Normal => "default",
			CursorIcon::Hand => "pointer",
			CursorIcon::Cross => "crosshair",
			CursorIcon::Move => "move",
			CursorIcon::Progress => "progress",
			CursorIcon::Wait => "wait",
			CursorIcon::Text => "text",
		};
	}
}

impl Key {

	fn from_web(e: &web_sys::KeyboardEvent) -> Option<Self> {

		return match e.code().as_ref() {
			"KeyQ" => Some(Key::Q),
			"KeyW" => Some(Key::W),
			"KeyE" => Some(Key::E),
			"KeyR" => Some(Key::R),
			"KeyT" => Some(Key::T),
			"KeyY" => Some(Key::Y),
			"KeyU" => Some(Key::U),
			"KeyI" => Some(Key::I),
			"KeyO" => Some(Key::O),
			"KeyP" => Some(Key::P),
			"KeyA" => Some(Key::A),
			"KeyS" => Some(Key::S),
			"KeyD" => Some(Key::D),
			"KeyF" => Some(Key::F),
			"KeyG" => Some(Key::G),
			"KeyH" => Some(Key::H),
			"KeyJ" => Some(Key::J),
			"KeyK" => Some(Key::K),
			"KeyL" => Some(Key::L),
			"KeyZ" => Some(Key::Z),
			"KeyX" => Some(Key::X),
			"KeyC" => Some(Key::C),
			"KeyV" => Some(Key::V),
			"KeyB" => Some(Key::B),
			"KeyN" => Some(Key::N),
			"KeyM" => Some(Key::M),
			"Digit1" => Some(Key::Key1),
			"Digit2" => Some(Key::Key2),
			"Digit3" => Some(Key::Key3),
			"Digit4" => Some(Key::Key4),
			"Digit5" => Some(Key::Key5),
			"Digit6" => Some(Key::Key6),
			"Digit7" => Some(Key::Key7),
			"Digit8" => Some(Key::Key8),
			"Digit9" => Some(Key::Key9),
			"Digit0" => Some(Key::Key0),
			"F1" => Some(Key::F1),
			"F2" => Some(Key::F2),
			"F3" => Some(Key::F3),
			"F4" => Some(Key::F4),
			"F5" => Some(Key::F5),
			"F6" => Some(Key::F6),
			"F7" => Some(Key::F7),
			"F8" => Some(Key::F8),
			"F9" => Some(Key::F9),
			"F10" => Some(Key::F10),
			"F11" => Some(Key::F11),
			"F12" => Some(Key::F12),
			"Minus" => Some(Key::Minus),
			"Equal" => Some(Key::Equal),
			"Comma" => Some(Key::Comma),
			"Period" => Some(Key::Period),
			"Backquote" => Some(Key::Backquote),
			"Slash" => Some(Key::Slash),
			"Backslash" => Some(Key::Backslash),
			"Semicolon" => Some(Key::Semicolon),
			"Quote" => Some(Key::Quote),
			"ArrowUp" => Some(Key::Up),
			"ArrowDown" => Some(Key::Down),
			"ArrowLeft" => Some(Key::Left),
			"ArrowRight" => Some(Key::Right),
			"Escape" => Some(Key::Esc),
			"Tab" => Some(Key::Tab),
			"Space" => Some(Key::Space),
			"Backspace" => Some(Key::Backspace),
			"Enter" => Some(Key::Enter),
			"ShiftLeft" => Some(Key::LShift),
			"ShiftRight" => Some(Key::RShift),
			"AltLeft" => Some(Key::LAlt),
			"AltRight" => Some(Key::RAlt),
			"MetaLeft" => Some(Key::LMeta),
			"MetaRight" => Some(Key::RMeta),
			"ControlLeft" => Some(Key::LCtrl),
			"ControlRight" => Some(Key::RCtrl),
			_ => None,

		};

	}

}


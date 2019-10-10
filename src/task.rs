// wengwengweng

//! Simple Threading Utilities

use std::collections::VecDeque;

use std::sync::mpsc;
use std::thread;

pub struct TaskPool<T: Send + 'static> {
	queue: VecDeque<Task<T>>,
	active: Vec<Task<T>>,
	max: u32,
}

impl<T: Send + 'static> TaskPool<T> {

	pub fn new(max: u32) -> Self {
		return Self {
			queue: VecDeque::new(),
			active: vec![],
			max: max,
		};
	}

	pub fn exec(&mut self, f: impl FnOnce() -> T + Send + 'static) {

		self.queue.push_back(Task::new(f));
		self.adjust();

	}

	fn adjust(&mut self) {

		self.active.retain(|t| !t.done());

		for _ in 0..self.max as usize - self.active.len() {
			if let Some(mut task) = self.queue.pop_front() {
				task.start();
				self.active.push(task);
			}
		}

	}

	pub fn poll(&mut self) -> Vec<T> {

		let mut basket = vec![];

		for task in &mut self.active {
			if let Some(data) = task.poll() {
				basket.push(data);
			}
		}

		self.adjust();

		return basket;

	}

	pub fn clear_queue(&mut self) {
		self.queue.clear();
	}

}

pub struct Task<T: Send + 'static> {
	rx: Option<mpsc::Receiver<T>>,
	action: Option<Box<dyn FnOnce() -> T + Send + 'static>>,
	done: bool,
}

impl<T: Send + 'static> Task<T> {

	pub fn new(f: impl FnOnce() -> T + Send + 'static) -> Self {
		return Self {
			action: Some(Box::new(f)),
			done: false,
			rx: None,
		};
	}

	pub fn exec(f: impl FnOnce() -> T + Send + 'static) -> Self {

		let mut task = Self::new(f);

		task.start();

		return task;

	}

	pub fn start(&mut self) {

		if let Some(action) = self.action.take() {

			let (tx, rx) = mpsc::channel();

			// TODO: deal with error inside thread::spawn
			thread::spawn(move || {
				tx.send(action()).expect("thread failure");
			});

			self.rx = Some(rx);

		}

	}

	pub fn started(&self) -> bool {
		return self.rx.is_some();
	}

	pub fn done(&self) -> bool {
		return self.done;
	}

	pub fn block(&mut self) -> Option<T> {

		let rx = self.rx.as_ref()?;
		let data = rx.recv().ok()?;

		self.done = true;

		return Some(data);

	}

	pub fn poll(&mut self) -> Option<T> {

		let rx = self.rx.as_ref()?;

		if self.done {
			return None;
		}

		let data = rx.try_recv().ok()?;

		self.done = true;

		return Some(data);

	}

}


///


trait Eventable {
	key() -> 
}

struct EventManager<Events> {}

impl<Events> EventManager<Events> {
	fn new() -> Self {
		EventManager {}
	}

	fn on(&mut self, event: E, callback: dyn Fn(E)) {
		//
	}

	fn send(&self, event: E) {}
}

fn test() {
	enum Events {
		A,
		B(i32),
		C(f64),
	}

	let event_manager = EventManager::new::<Events>();
	event_manager.on(Events::A, || {})
}

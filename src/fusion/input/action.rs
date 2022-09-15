use super::{InputData, InputHandlerHandler};
use rustc_hash::FxHashSet;
use std::{fmt::Debug, iter::FromIterator, mem::swap, sync::Arc};

pub trait InputActionState: Sized + Clone + Send + Sync + 'static {}
impl<T: Sized + Clone + Send + Sync + 'static> InputActionState for T {}

pub type ActiveCondtion<S> = fn(&InputData, state: &S) -> bool;

#[derive(Clone)]
pub struct InputAction<S: InputActionState> {
	pub capture_on_trigger: bool,
	pub active_condition: ActiveCondtion<S>,

	pub started_acting: FxHashSet<Arc<InputData>>,
	pub actively_acting: FxHashSet<Arc<InputData>>,
	pub stopped_acting: FxHashSet<Arc<InputData>>,
	queued_inputs: FxHashSet<Arc<InputData>>,
}
impl<S: InputActionState> InputAction<S> {
	pub fn new(capture_on_trigger: bool, active_condition: ActiveCondtion<S>) -> Self {
		Self {
			capture_on_trigger,
			active_condition,

			started_acting: FxHashSet::default(),
			actively_acting: FxHashSet::default(),
			stopped_acting: FxHashSet::default(),
			queued_inputs: FxHashSet::default(),
		}
	}

	fn update(&mut self, external: &mut InputAction<S>) {
		self.started_acting = FxHashSet::from_iter(
			self.queued_inputs
				.difference(&self.actively_acting)
				.cloned(),
		);
		self.stopped_acting = FxHashSet::from_iter(
			self.actively_acting
				.difference(&self.queued_inputs)
				.cloned(),
		);
		swap(&mut self.actively_acting, &mut self.queued_inputs);
		self.queued_inputs.clear();

		external.started_acting = self.started_acting.clone();
		external.actively_acting = self.actively_acting.clone();
		external.stopped_acting = self.stopped_acting.clone();
		external.started_acting = self.started_acting.clone();

		self.capture_on_trigger = external.capture_on_trigger;
		self.active_condition = external.active_condition;
	}

	fn input_event(&mut self, input_data: &Arc<InputData>, state: &S) -> bool {
		if (self.active_condition)(input_data, state) {
			self.queued_inputs.insert(input_data.clone());
			true
		} else {
			false
		}
	}
}
impl<S: InputActionState> PartialEq for InputAction<S> {
	fn eq(&self, other: &Self) -> bool {
		self.capture_on_trigger == other.capture_on_trigger
			&& self.active_condition as usize == other.active_condition as usize
	}
}
impl<S: InputActionState> Debug for InputAction<S> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("InputAction")
			.field("capture_on_trigger", &self.capture_on_trigger)
			.field("started_acting", &self.started_acting)
			.field("actively_acting", &self.actively_acting)
			.field("stopped_acting", &self.stopped_acting)
			.field("queued_inputs", &self.queued_inputs)
			.finish()
	}
}

#[derive(Debug, Default)]
pub struct InputActionHandler<S: InputActionState> {
	actions: Vec<InputAction<S>>,
	state: S,
	back_state: S,
}
impl<S: InputActionState> InputActionHandler<S> {
	pub fn new(state: S) -> Self {
		Self {
			actions: Vec::new(),
			back_state: state.clone(),
			state,
		}
	}

	pub fn update_actions<'a>(
		&mut self,
		actions: impl IntoIterator<Item = &'a mut InputAction<S>>,
	) {
		self.back_state = self.state.clone();

		self.actions = actions
			.into_iter()
			.map(|action| {
				if let Some(internal_action) = self
					.actions
					.iter_mut()
					.find(|internal_action| **internal_action == *action)
				{
					internal_action.update(action);
				}
				action.clone()
			})
			.collect();
	}
}
impl<S: InputActionState> InputHandlerHandler for InputActionHandler<S> {
	fn input(&mut self, input: InputData) -> bool {
		let input = Arc::new(input);
		self.actions
			.iter_mut()
			.map(|action| action.input_event(&input, &self.state) && action.capture_on_trigger)
			.any(|b| b)
	}
}

#[tokio::test]
async fn fusion_input_action_handler() {
	use crate::fusion::{client::Client, fields::SphereField, input::InputHandler};
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	struct InputActionHandlerTest {
		field: SphereField,
		input_handler: crate::fusion::HandlerWrapper<InputHandler, InputActionHandler<f32>>,
		hover_action: InputAction<f32>,
	}

	let field = SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
		.unwrap();
	let input_action_test = InputActionHandlerTest {
		input_handler: InputHandler::create(client.get_root(), None, None, &field, |_, _| {
			InputActionHandler::new(0.05)
		})
		.unwrap(),
		hover_action: InputAction::new(false, |input_data, max_distance| {
			dbg!(input_data);
			input_data.distance < *max_distance
		}),
		field,
	};

	impl crate::fusion::client::LifeCycleHandler for InputActionHandlerTest {
		fn logic_step(&mut self, info: crate::fusion::client::LogicStepInfo) {
			println!("Life cycle step {}s", info.elapsed);
			self.input_handler
				.lock_inner()
				.update_actions([&mut self.hover_action].into_iter());
			// dbg!(&self.hover_action);
		}
	}

	let _root = client.wrap_root(input_action_test);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
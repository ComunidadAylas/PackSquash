use crate::terminal_title_setter::{
	TerminalTitleSetter, TerminalTitleSetterTrait, TerminalTitleString
};
use std::sync::atomic::{AtomicUsize, Ordering};
use strum::FromRepr;

/// Controller for the title to be shown in the terminal or console attached to this process.
/// There are several title phases, each corresponding to one phase in the application lifecycle.
/// Each title phase has one or several titles.
pub struct TerminalTitleController {
	title_setter: TerminalTitleSetter,
	current_title_index: AtomicUsize,
	title_strings: [TerminalTitleString; 7]
}

/// Represents the terminal titles that can be shown.
#[derive(FromRepr, Default, Copy, Clone)]
enum TerminalTitle {
	#[default]
	Idle,
	ProcessingPack1,
	ProcessingPack2,
	ProcessingPack3,
	ProcessingPack4,
	Finishing1,
	Finishing2
}

impl TerminalTitleController {
	/// Creates a new terminal title controller, if it is possible to change the terminal or console
	/// title. It may not be possible to change the title if the process does not have any associated
	/// terminal or console, for example.
	///
	/// The initial, default title will be shown when creating the title controller.
	pub fn new() -> Option<Self> {
		TerminalTitleSetter::init().map(|title_setter| {
			let initial_title_index = TerminalTitle::default() as usize;
			let title_strings = [
				// Convert string literals to terminal title strings. This allows optimizations
				// for some platforms, such as Windows. There must be one per variant of TerminalTitle
				"PackSquash".into(),
				"PackSquash - Optimizing [-]".into(),
				"PackSquash - Optimizing [\\]".into(),
				"PackSquash - Optimizing [|]".into(),
				"PackSquash - Optimizing [/]".into(),
				"PackSquash - Finishing [ ]".into(),
				"PackSquash - Finishing [·]".into()
			];

			title_setter.set_title(&title_strings[initial_title_index]);

			Self {
				title_setter,
				current_title_index: AtomicUsize::new(initial_title_index),
				title_strings
			}
		})
	}

	/// Moves on to the next title of the current phase, and then shows that title.
	pub fn advance_and_show(&self) {
		let mut next_title_index = 0;
		self.current_title_index
			.fetch_update(
				Ordering::AcqRel,
				Ordering::Acquire,
				|previous_title_index| {
					next_title_index = match TerminalTitle::from_repr(previous_title_index).unwrap() {
						TerminalTitle::Idle => TerminalTitle::Idle,
						TerminalTitle::ProcessingPack1 => TerminalTitle::ProcessingPack2,
						TerminalTitle::ProcessingPack2 => TerminalTitle::ProcessingPack3,
						TerminalTitle::ProcessingPack3 => TerminalTitle::ProcessingPack4,
						TerminalTitle::ProcessingPack4 => TerminalTitle::ProcessingPack1,
						TerminalTitle::Finishing1 => TerminalTitle::Finishing2,
						TerminalTitle::Finishing2 => TerminalTitle::Finishing1
					} as usize;

					Some(next_title_index)
				}
			)
			.unwrap();

		self.title_setter
			.set_title(&self.title_strings[next_title_index]);
	}

	/// Moves on to the first title of the next title phase, and then shows that title.
	pub fn next_title_phase(&self) {
		let mut next_title_index = 0;
		self.current_title_index
			.fetch_update(
				Ordering::AcqRel,
				Ordering::Acquire,
				|previous_title_index| {
					next_title_index = match TerminalTitle::from_repr(previous_title_index).unwrap() {
						TerminalTitle::Idle => TerminalTitle::ProcessingPack1,
						TerminalTitle::ProcessingPack1 => TerminalTitle::Finishing1,
						TerminalTitle::ProcessingPack2 => TerminalTitle::Finishing1,
						TerminalTitle::ProcessingPack3 => TerminalTitle::Finishing1,
						TerminalTitle::ProcessingPack4 => TerminalTitle::Finishing1,
						TerminalTitle::Finishing1 => TerminalTitle::Finishing1,
						TerminalTitle::Finishing2 => TerminalTitle::Finishing2
					} as usize;

					Some(next_title_index)
				}
			)
			.unwrap();

		self.title_setter
			.set_title(&self.title_strings[next_title_index]);
	}
}

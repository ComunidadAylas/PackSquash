use crate::terminal_title_setter::{
	TerminalTitleSetter, TerminalTitleSetterTrait, TerminalTitleString
};

/// Controller for the title to be shown in the terminal or console attached to this process.
/// There are several title phases, each corresponding to one phase in the application lifecycle.
/// Each title phase has one or several titles.
pub struct TerminalTitleController {
	title_setter: TerminalTitleSetter,
	current_title: TerminalTitle,
	title_strings: [TerminalTitleString<'static>; 7]
}

/// Represents the terminal titles that can be shown.
#[derive(Copy, Clone)]
enum TerminalTitle {
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
	/// No title will be shown after creating the title controller. The callee is responsible for
	/// calling [`TerminalTitleController::show`] if it is desired to show the first title of the first
	/// phase.
	pub fn new() -> Option<Self> {
		TerminalTitleSetter::init().map(|title_setter| Self {
			title_setter,
			current_title: TerminalTitle::Idle,
			title_strings: [
				// Convert string literals to terminal title strings. This allows optimizations
				// for some platforms, such as Windows. There must be one per variant of TerminalTitle
				"PackSquash".into(),
				"PackSquash - Optimizing [-]".into(),
				"PackSquash - Optimizing [\\]".into(),
				"PackSquash - Optimizing [|]".into(),
				"PackSquash - Optimizing [/]".into(),
				"PackSquash - Finishing [ ]".into(),
				"PackSquash - Finishing [·]".into()
			]
		})
	}

	/// Shows the current title for the current title phase.
	pub fn show(&self) {
		self.title_setter
			.set_title(&self.title_strings[self.current_title as usize])
	}

	/// Moves on to the next title of the current phase, and then shows that title.
	pub fn advance_and_show(&mut self) {
		self.current_title = match self.current_title {
			TerminalTitle::Idle => TerminalTitle::Idle,
			TerminalTitle::ProcessingPack1 => TerminalTitle::ProcessingPack2,
			TerminalTitle::ProcessingPack2 => TerminalTitle::ProcessingPack3,
			TerminalTitle::ProcessingPack3 => TerminalTitle::ProcessingPack4,
			TerminalTitle::ProcessingPack4 => TerminalTitle::ProcessingPack1,
			TerminalTitle::Finishing1 => TerminalTitle::Finishing2,
			TerminalTitle::Finishing2 => TerminalTitle::Finishing1
		};

		self.show();
	}

	/// Moves on to the first title of the next title phase. The new title is not
	/// shown; call [`TerminalTitleController::show`] to show it.
	pub fn next_title_phase(&mut self) {
		self.current_title = match self.current_title {
			TerminalTitle::Idle => TerminalTitle::ProcessingPack1,
			TerminalTitle::ProcessingPack1 => TerminalTitle::Finishing1,
			TerminalTitle::ProcessingPack2 => TerminalTitle::Finishing1,
			TerminalTitle::ProcessingPack3 => TerminalTitle::Finishing1,
			TerminalTitle::ProcessingPack4 => TerminalTitle::Finishing1,
			TerminalTitle::Finishing1 => TerminalTitle::Finishing1,
			TerminalTitle::Finishing2 => TerminalTitle::Finishing2
		}
	}
}

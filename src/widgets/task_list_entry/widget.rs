use gtk::prelude::{
	BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, WidgetExt,
};
use relm4::{
	adw,
	gtk::{
		self,
		traits::{GtkWindowExt, OrientableExt},
	},
	Component, ComponentParts, ComponentSender, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::{
	fl,
	widgets::task_list_entry::model::{
		TaskListEntryComponent, TaskListEntryMode,
	},
};

use super::messages::{TaskListEntryInput, TaskListEntryOutput};

#[relm4::component(pub)]
impl Component for TaskListEntryComponent {
	type Input = TaskListEntryInput;
	type Output = TaskListEntryOutput;
	type Init = Option<String>;
	type CommandOutput = ();

	view! {
		#[root]
		adw::Window {
			set_hide_on_close: true,
			set_default_width: 320,
			set_resizable: false,
			set_modal: true,

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,

				adw::HeaderBar {
					set_show_end_title_buttons: true,
					set_css_classes: &["flat"],
					set_title_widget: Some(&gtk::Box::default())
				},
				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_margin_all: 20,
					set_spacing: 10,
					gtk::Image {
							set_icon_size: gtk::IconSize::Large,
							set_icon_name: Some(match model.mode {
								TaskListEntryMode::New => icon_name::PLUS,
								TaskListEntryMode::Edit => icon_name::PENCIL_AND_PAPER
							}),
					},
					gtk::Label {
						set_css_classes: &["title-4"],
						set_label: match model.mode {
							TaskListEntryMode::New => "You're about to add a list.",
							TaskListEntryMode::Edit => "You're about to rename this list."
						},
					},
					gtk::Label {
						set_label: "Pick a descriptive name.",
					},
					#[name = "new_list_entry"]
					gtk::Entry {
						set_placeholder_text: Some(fl!("list-name")),
						set_buffer: &model.name,
						connect_activate => TaskListEntryInput::HandleEntry,
					},
					gtk::Button {
						set_css_classes: &["suggested-action"],
						set_label: model.label.as_str(),
						connect_clicked => TaskListEntryInput::HandleEntry,
					},
				}
			}
		}
	}

	fn init(
		init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = if let Some(name) = init {
			TaskListEntryComponent {
				name: gtk::EntryBuffer::new(Some(name)),
				mode: TaskListEntryMode::Edit,
				label: fl!("rename").clone(),
			}
		} else {
			TaskListEntryComponent {
				name: gtk::EntryBuffer::new(Some("")),
				mode: TaskListEntryMode::New,
				label: fl!("add-list").clone(),
			}
		};

		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: ComponentSender<Self>,
		root: &Self::Root,
	) {
		match message {
			TaskListEntryInput::HandleEntry => {
				let name = self.name.text();

				match self.mode {
					TaskListEntryMode::New => {
						sender
							.output(TaskListEntryOutput::AddTaskListToSidebar(
								name.to_string(),
							))
							.unwrap_or_default();
					},
					TaskListEntryMode::Edit => {
						sender
							.output(TaskListEntryOutput::RenameList(name.to_string()))
							.unwrap_or_default();
					},
				}
				root.close();
			},
		}
	}
}

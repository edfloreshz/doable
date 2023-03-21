use crate::{
	application::plugin::Plugin,
	factories::task_list::model::TaskListFactoryModel,
	widgets::smart_lists::sidebar::model::SmartList,
};

#[derive(Debug)]
pub enum SidebarComponentInput {
	PluginSelected(Plugin),
	EnableService(Plugin),
	DisableService(Plugin),
	RemoveService(Plugin),
	AddPluginToSidebar(Plugin),
	SelectSmartList(SmartList),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarComponentOutput {
	PluginSelected(Plugin),
	ListSelected(Box<TaskListFactoryModel>),
	Forward,
	Notify(String, u32),
	DisablePlugin,
	SelectSmartList(SmartList),
}

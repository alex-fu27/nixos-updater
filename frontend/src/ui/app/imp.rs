use glib::subclass::InitializingObject;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/de/afuchs/NixOSUpdater/app.ui")]
pub struct UpdaterWindow {
//	#[template_child]
//	pub button: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for UpdaterWindow {
	// `NAME` needs to match `class` attribute of template
	const NAME: &'static str = "UpdaterWindow";
	type Type = super::UpdaterWindow;
	type ParentType = adw::ApplicationWindow;

	fn class_init(klass: &mut Self::Class) {
		crate::ui::UpdaterOverviewPage::ensure_type();
		crate::ui::UpdaterPreferencesPage::ensure_type();
		klass.bind_template();
	}

	fn instance_init(obj: &InitializingObject<Self>) {
		obj.init_template();
	}
}

// Trait shared by all GObjects
impl ObjectImpl for UpdaterWindow {
	fn constructed(&self) {
		// Call "constructed" on parent
		self.parent_constructed();

//		// Connect to "clicked" signal of `button`
//		self.button.connect_clicked(move |button| {
//			// Set the label to "Hello World!" after the button has been clicked on
//			button.set_label("Hello World!");
//		});
	}
}

// Trait shared by all widgets
impl WidgetImpl for UpdaterWindow {}

// Trait shared by all windows
impl WindowImpl for UpdaterWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for UpdaterWindow {}
impl AdwApplicationWindowImpl for UpdaterWindow {}


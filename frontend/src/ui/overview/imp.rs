use glib::subclass::InitializingObject;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/de/afuchs/NixOSUpdater/overview.ui")]
pub struct UpdaterOverviewPage {
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for UpdaterOverviewPage {
	// `NAME` needs to match `class` attribute of template
	const NAME: &'static str = "UpdaterOverviewPage";
	type Type = super::UpdaterOverviewPage;
	type ParentType = adw::Bin;

	fn class_init(klass: &mut Self::Class) {
		klass.bind_template();
	}

	fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
		 obj.init_template();
	}
}

// Trait shared by all GObjects
impl ObjectImpl for UpdaterOverviewPage {
	fn constructed(&self) {
		// Call "constructed" on parent
		self.parent_constructed();
	}
}

// Trait shared by all widgets
impl WidgetImpl for UpdaterOverviewPage {}

impl BinImpl for UpdaterOverviewPage {}


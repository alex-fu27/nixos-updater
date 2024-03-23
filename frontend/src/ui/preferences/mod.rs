mod imp;

use glib::Object;
use gtk::{gio, glib};

glib::wrapper! {
	 pub struct UpdaterPreferencesPage(ObjectSubclass<imp::UpdaterPreferencesPage>)
		  @extends adw::PreferencesPage, gtk::Widget,
		  @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for UpdaterPreferencesPage {
    fn default() -> Self {
		  Self::new()
    }
}

impl UpdaterPreferencesPage {
	 fn new() -> Self {
		  Object::builder().build()
	 }
}



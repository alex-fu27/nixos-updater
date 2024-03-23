mod imp;

use glib::Object;
use gtk::{gio, glib};

glib::wrapper! {
	 pub struct UpdaterOverviewPage(ObjectSubclass<imp::UpdaterOverviewPage>)
		  @extends adw::Bin, gtk::Widget,
		  @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for UpdaterOverviewPage {
    fn default() -> Self {
		  Self::new()
    }
}

impl UpdaterOverviewPage {
	 fn new() -> Self {
		  Object::builder().build()
	 }
}



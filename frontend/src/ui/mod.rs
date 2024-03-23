mod imp;

use glib::Object;
use gtk::{gio, glib};
use adw::Application;

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

glib::wrapper! {
	 pub struct UpdaterWindow(ObjectSubclass<imp::UpdaterWindow>)
		  @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
		  @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
				gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl UpdaterWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }
}


mod ui;

use gtk::prelude::*;
use gtk::glib; 
use adw::{Application, ApplicationWindow};
use gio::*;

const APP_ID: &str = "de.afuchs.NixOSUpdater";

fn main() -> glib::ExitCode {
	 gio::resources_register_include!("resources.gresource")
		  .expect("Failed to register resources.");
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();


    // Connect to "activate" signal of `app`
    app.connect_activate(activate);

    // Run the application
    app.run()

}

fn push_example(app: &Application) {
    let mut n = gio::Notification::new("Aktualisierung verfügbar");
    n.set_body(Some("Eine Systemaktaulisierung wurde vorbereitet. Wann soll sie angewendet werden?"));
    n.add_button("Sofort", "detail");
    n.add_button("Beim nächsten Neustart", "detail");
    app.send_notification(Some("test"), &n);
}

fn activate(app: &Application) {
    // Create a window and set the title
    let window = ui::UpdaterWindow::new(app);
	 window.present();

}

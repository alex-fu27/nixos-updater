use dbus::blocking::{Connection, Proxy};
use std::time::Duration;

struct Client {
    con: Connection,
}

impl Client {
    fn new() -> Result<Self, dbus::Error> {
        let con = Connection::new_session()?;
        Ok(Self { con })
    }

    fn get_proxy<'a>(&'a self) -> Proxy<'_, &'a Connection> {
        self.con.with_proxy("de.afuchs.NixOSUpdater", "/de/afuchs/NixOSUpdater", Duration::from_millis(5000))
    }
}

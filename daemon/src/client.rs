use crate::consts;

use dbus::blocking::{Connection, Proxy};
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use std::time::Duration;

pub struct Client {
    con: Connection,
}

impl Client {
    pub fn new() -> Result<Self, dbus::Error> {
        let con = Connection::new_session()?;
        Ok(Self { con })
    }

    fn get_proxy(&self) -> Proxy<'_, &'_ Connection> {
        self.con.with_proxy(consts::NAME, "/de/afuchs/NixOSUpdater", Duration::from_millis(5000))
    }

    pub fn print_status(&self) -> anyhow::Result<()> {
        let status: String = self.get_proxy().get(consts::NAME, "UpdateState")?;
        println!("UpdateState={}", status);
        if status == "processing" {
            let status: String = self.get_proxy().get(consts::NAME, "ProcessState")?;
            println!("ProcessState={}", status);
        }
        Ok(())
    }
}

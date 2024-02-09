use dbus::blocking::Connection;
use dbus_crossroads::{Crossroads, Context, PropContext};
use std::error::Error;

const NAME: &str = "de.afuchs.NixOSUpdater";

struct Hello {
    count: u32,
    last_name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let con = Connection::new_session()?;
    con.request_name(NAME, false, true, false)?;

    let mut cr = Crossroads::new();

    let iface_token = cr.register(NAME, |b| {
        let last_name_changed = b.property::<String, _>("LastName")
            .get(|_ctx: &mut PropContext, hello: &mut Hello| {
                Ok(hello.last_name.clone())
            }).changed_msg_fn();
        b.method("Hello", ("name",), ("reply",), move |ctx: &mut Context, hello: &mut Hello, (name,): (String,)| {
            println!("Hello called from {}.", name);
            hello.count += 1;
            hello.last_name = name.clone();
            let reply = format!("Hello, {} you are number {}.", name, hello.count);

            if let Some(msg) = last_name_changed(ctx.path(), &hello.last_name) {
                ctx.push_msg(msg);
            }

            Ok((reply,))
        });
    });

    cr.insert("/de/afuchs/NixOSUpdater/hello", &[iface_token], Hello { count: 0, last_name: "none".into() });
    cr.serve(&con)?;
    unreachable!()
}

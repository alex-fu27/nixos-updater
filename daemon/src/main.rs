use dbus::message::MatchRule;
use dbus::channel::MatchingReceiver;
use futures::future;
use dbus_tokio::connection;
use dbus_crossroads::{Crossroads, Context, PropContext, MethodErr};
use tokio::time::sleep;
use std::time::Duration;
use std::sync::{Mutex, Arc};

const NAME: &str = "de.afuchs.NixOSUpdater";

struct Hello {
    count: u32,
    last_name: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (resource, con) = connection::new_session_sync()?;

    // spawn resource, will only finish on error
    let _handle = tokio::spawn(async {
        let err = resource.await;
        panic!("Lost D-Bus connection: {}", err);
    });


    con.request_name(NAME, false, true, false).await?;

    let mut cr = Crossroads::new();
    
    // tell Crossroads how to spawn tasks
    cr.set_async_support(Some((con.clone(), Box::new(|x| { tokio::spawn(x); }))));

    let iface_token = cr.register(NAME, |b| {
        let last_name_changed = b.property::<String, _>("LastName")
            .get(|_ctx: &mut PropContext, mh: &mut Arc<Mutex<Hello>>| {
                let hello = mh.lock().unwrap();
                Ok(hello.last_name.clone())
            }).changed_msg_fn();
        b.method_with_cr_async("Hello", ("name",), ("reply",), move |mut ctx, cr, (name,): (String,)| {
            let mh: Arc<Mutex<Hello>> = Arc::clone(&cr.data_mut(ctx.path()).unwrap());
            println!("Hello called from {}. Please wait...", name);

            async move {
                sleep(Duration::from_millis(2000)).await;

                let reply = {
                    let mut hello = mh.lock().unwrap();
                    hello.count += 1;
                    hello.last_name = name.clone();
                    format!("Hello, {} you are number {}.", name, hello.count)
                };
                println!("reply: {}", reply);

//                if let Some(msg) = last_name_changed(ctx.path(), &hello.last_name) {
//                    ctx.push_msg(msg);
//                }

                ctx.reply(Ok((reply,)))
            }
        });
    });

    let data = Arc::new(Mutex::new(Hello { count: 0, last_name: "none".into() }));
    cr.insert("/de/afuchs/NixOSUpdater/hello", &[iface_token], data);
    con.start_receive(MatchRule::new_method_call(), Box::new(move |msg, conn| {
        cr.handle_message(msg, conn).unwrap();
        true
    }));

    future::pending::<()>().await;
    unreachable!()
}

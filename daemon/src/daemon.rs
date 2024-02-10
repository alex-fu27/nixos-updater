use dbus::{Path, Message};
use dbus::arg::RefArg;
use dbus::message::MatchRule;
use dbus::channel::MatchingReceiver;
use futures::future;
use dbus_tokio::connection;
use dbus_crossroads::{Crossroads, Context, PropContext, MethodErr, IfaceBuilder};
use tokio::time::sleep;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::fmt;

use crate::consts;

#[derive(Debug)]
enum ProcessState {
    Evaluating,
    Building,
    Switching,
}

impl ProcessState {
    fn to_str(&self) -> &'static str {
        use ProcessState::*;
        match self {
            Evaluating => "evaluating",
            Building => "building",
            Switching => "switching",
        }
    }
}

#[derive(Debug)]
enum UpdateError {
    EvaluationFailed,
    BuildFailed,
    SwitchFailed,
}

impl UpdateError {
    fn to_str(&self) -> &'static str {
        use UpdateError::*;
        match self {
            EvaluationFailed => "evaluation_failed",
            BuildFailed => "build_failed",
            SwitchFailed => "switch_failed",
        }
    }
}

#[derive(Debug)]
struct UpgradeReadyInfo {
    requires_reboot: bool,
}

#[derive(Debug)]
enum UpdateState {
    UpToDate,
    Processing(ProcessState),
    Ready(UpgradeReadyInfo),
    Error(UpdateError),
}

impl UpdateState {
    fn to_str(&self) -> &'static str {
        use UpdateState::*;
        match self {
            UpToDate => "up_to_date",
            Processing(_) => "processing",
            Ready(_) => "ready",
            Error(_) => "error",
        }
    }
}

struct DaemonState {
    update_state: UpdateState,
}

impl DaemonState {
    fn initial() -> Self {
        Self { update_state: UpdateState::UpToDate }
    }
}

type SyncedDaemonState = Arc<Mutex<DaemonState>>;

type DbusPropFun = Box<dyn Fn(&Path<'_>, &dyn RefArg) -> Option<Message> + Send + Sync + 'static>;
struct DbusProperties {
    version: DbusPropFun,
    update_state: DbusPropFun,
    process_state: DbusPropFun,
}

impl DbusProperties {
    fn new(b: &mut IfaceBuilder<SyncedDaemonState>) -> Self {
        Self {
            version: b.property::<String, _>("Version")
                .get(|_ctx: &mut PropContext, mh: &mut SyncedDaemonState| {
                    Ok(clap::crate_version!().to_string())
                }).changed_msg_fn(),

            update_state: b.property::<String, _>("UpdateState")
                .get(|_ctx: &mut PropContext, mh: &mut SyncedDaemonState| {
                    Ok(mh.lock().unwrap().update_state.to_str().to_string())
                }).changed_msg_fn(),

            process_state: b.property::<String, _>("ProcessState")
                .get(|_ctx: &mut PropContext, mh: &mut SyncedDaemonState| {
                    let ds = &mh.lock().unwrap().update_state;
                    match ds {
                        UpdateState::Processing(ps) => Ok(ps.to_str().to_string()),
                        _ => Err(MethodErr::failed("no update is being processed")),
                    }
                }).changed_msg_fn(),
        }
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (resource, con) = connection::new_session_sync()?;

    // spawn , will only finish on error
    let _handle = tokio::spawn(async {
        let err = resource.await;
        panic!("Lost D-Bus connection: {}", err);
    });


    con.request_name(consts::NAME, false, true, false).await?;

    let mut cr = Crossroads::new();
    
    // tell Crossroads how to spawn tasks
    cr.set_async_support(Some((con.clone(), Box::new(|x| { tokio::spawn(x); }))));

    let iface_token = cr.register(consts::NAME, |b| {
        let props = Arc::new(DbusProperties::new(b));
        
        b.method_with_cr_async("StartUpdate", (), (), move |mut ctx, cr, _: ()| {
            let mh: SyncedDaemonState = Arc::clone(&cr.data_mut(ctx.path()).unwrap());
            let props = Arc::clone(&props);

            async move {
                {
                    let mut ds = mh.lock().unwrap();
                    ds.update_state = UpdateState::Processing(ProcessState::Evaluating);
                    ctx.push_msg((props.update_state)(ctx.path(), &ds.update_state.to_str().to_string()).unwrap());
                }
                sleep(Duration::from_millis(3000)).await;
                {
                    let mut ds = mh.lock().unwrap();
                    ds.update_state = UpdateState::UpToDate;
                    ctx.push_msg((props.update_state)(ctx.path(), &ds.update_state.to_str().to_string()).unwrap());
                }

                ctx.reply(Ok(()))
            }
        });
    });

    cr.insert("/de/afuchs/NixOSUpdater", &[iface_token], Arc::new(Mutex::new(DaemonState::initial())));
    con.start_receive(MatchRule::new_method_call(), Box::new(move |msg, conn| {
        cr.handle_message(msg, conn).unwrap();
        true
    }));

    future::pending::<()>().await;
    unreachable!()
}


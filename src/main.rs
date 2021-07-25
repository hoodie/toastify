#[cfg(all(unix, not(target_os = "macos")))]
use std::convert::TryFrom;
use std::ffi::OsString;

use clap::{crate_authors, crate_description, crate_version, Clap};
use notify_rust;
use notify_rust::Notification;

#[cfg(all(unix, not(target_os = "macos")))]
use notify_rust::{Hint, Urgency};

#[cfg(all(unix, not(target_os = "macos")))]
fn parse_urgency(s: &str) -> Result<Urgency, notify_rust::Error> {
    <Urgency as TryFrom<&str>>::try_from(s)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn parse_hint(pattern: &str) -> Result<Hint, String> {
    let parts = pattern.splitn(2, ':').collect::<Vec<&str>>();
    let (name, value) = (parts[0], parts[1]);
    Hint::from_key_val(name, value)
}
/// Shows a notification.
#[derive(Clap)]
struct Send {
    /// Title / Summary of the Notification.
    summary: String,

    /// Message body.
    body: Option<String>,

    /// Set a specific app-name manually.
    #[clap(short = 'a', long)]
    app_name: Option<String>,

    /// Time until expiration in milliseconds. 0 means forever. -1 means the server decides
    #[clap(short = 't', long)]
    expire_time: Option<i32>,

    /// Icon of notification.
    #[clap(short, long)]
    icon: Option<OsString>,

    /// Specifies the ID and overrides existing notifications with the same ID.
    #[clap(long)]
    id: Option<u32>,

    /// Set a category.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[clap(short, long)]
    category: Option<Vec<String>>,

    /// Also prints notification to stdout.
    #[clap(short, long)]
    debug: bool,

    // Specifies basic extra data to pass. Valid types are int, double, string and byte. Pattern: TYPE:NAME:VALUE
    #[cfg(all(unix, not(target_os = "macos")))]
    #[clap(short, long, parse(try_from_str = parse_hint))]
    hint: Option<Hint>,

    /// How urgent is it.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[clap(short, long, parse(try_from_str = parse_urgency))]
    urgency: Option<Urgency>,
}

#[derive(Clap)]
#[clap(author = crate_authors!(), version = crate_version!(), about = crate_description!())]
enum Toastify {
    /// Shows a notification.
    Send(Send),

    /// Shows information about the running notification server.
    #[cfg(all(unix, not(target_os = "macos")))]
    Info,

    /// Starts a little notification server for testing.
    #[cfg(all(unix, not(target_os = "macos")))]
    Server,
}

fn main() {
    let args = Toastify::parse();

    match args {
        #[cfg(all(unix, not(target_os = "macos")))]
        Toastify::Server => {
            use notify_rust::server::NotificationServer;
            use std::thread;

            let server = NotificationServer::create();
            thread::spawn(move || {
                NotificationServer::start(&server, |notification| println!("{:#?}", notification))
            });

            println!("Press enter to exit.\n");

            std::thread::sleep(std::time::Duration::from_millis(1_000));

            Notification::new()
                .summary("Notification Logger")
                .body("If you can read this in the console, the server works fine.")
                .show()
                .expect("Was not able to send initial test message");

            let mut _devnull = String::new();
            let _ = std::io::stdin().read_line(&mut _devnull);
            println!("Thank you for choosing toastify.");
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        Toastify::Info => {
            match notify_rust::get_server_information() {
                Ok(info) => println!("server information:\n {:?}\n", info),
                Err(error) => eprintln!("{}", error),
            }

            match notify_rust::get_capabilities() {
                Ok(caps) => println!("capabilities:\n {:?}\n", caps),
                Err(error) => eprintln!("{}", error),
            }
        }
        Toastify::Send(send) => {
            let mut notification = Notification::new();

            notification.summary(&send.summary);

            if let Some(appname) = send.app_name {
                notification.appname(&appname);
            }

            if let Some(icon) = send.icon {
                notification.icon(icon.to_str().unwrap());
            }

            if let Some(body) = send.body {
                notification.body(&body);
            }

            #[cfg(all(unix, not(target_os = "macos")))]
            if let Some(categories) = send.category {
                categories.iter().for_each(|category| {
                    notification.hint(notify_rust::Hint::Category(category.to_owned()));
                })
            }

            if let Some(timeout) = send.expire_time {
                if timeout >= -1 {
                    notification.timeout(timeout);
                } else {
                    println!(
                        "Timeout should be -1, 0, or positive. {} is invalid",
                        timeout
                    );
                }
            }

            #[cfg(all(unix, not(target_os = "macos")))]
            if let Some(urgency) = send.urgency {
                notification.urgency(urgency);
            }

            if let Some(id) = send.id {
                notification.id(id);
            }

            #[cfg(all(unix, not(target_os = "macos")))]
            if let Some(hint) = send.hint {
                notification.hint(hint);
            }

            if send.debug {
                dbg!(notification);
                // println!("{:?}", notification);
            } else {
                notification.show().unwrap();
            }
        }
    }
}

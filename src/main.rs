use clap::{Parser, Subcommand, ValueEnum};
#[cfg(all(unix, not(target_os = "macos")))]
use notify_rust::Hint;
use notify_rust::{error::Result as nResult, Notification, Urgency};

#[derive(ValueEnum, Clone, Copy)]
pub enum UrgencyShim {
    Low,
    Normal,
    Critical,
}

impl From<UrgencyShim> for Urgency {
    fn from(urgency: UrgencyShim) -> Urgency {
        match urgency {
            UrgencyShim::Low => Urgency::Low,
            UrgencyShim::Normal => Urgency::Normal,
            UrgencyShim::Critical => Urgency::Critical,
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn parse_hint(pattern: &str) -> Result<Hint, String> {
    let parts = pattern.split(':').collect::<Vec<&str>>();
    if parts.len() != 3 {
        return Err("Wrong number of segments".into());
    }
    let (_typ, name, value) = (parts[0], parts[1], parts[2]);
    Hint::from_key_val(name, value)
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
enum Commands {
    // /// Starts a little notification server for testing
    // #[cfg(all(unix, not(target_os = "macos")))]
    // Server,
    /// Shows information about the running notification server
    #[cfg(all(unix, not(target_os = "macos")))]
    Info,
    /// Shows a notification
    Send {
        /// Title of the Notification.
        title: String,

        /// Message body
        body: Option<String>,
        /// Set a specific app-name manually.
        #[arg(short, long)]
        app_name: Option<String>,
        /// Set a specific sound manually.
        #[arg(short, long)]
        sound_name: Option<String>,
        #[command(flatten)]
        platform_args: PlatformArgs,
    },
}

#[derive(clap::Args)]
struct PlatformArgs {
    /// Time until expiration in milliseconds. On Windows, 25000 or more shows a long toast.
    #[cfg(not(target_os = "macos"))]
    #[arg(short = 't', long)]
    expire_time: Option<i32>,

    /// Icon of notification.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[arg(short = 'i', long)]
    icon: Option<std::path::PathBuf>,

    /// Specifies the ID and overrides existing notifications with the same ID.
    #[cfg(all(unix, not(target_os = "macos")))]
    id: Option<u32>, // TODO: Type is u32 or string?

    /// Set a category.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[arg(short, long)]
    categories: Option<Vec<String>>,

    /// Specifies basic extra data to pass. Valid types are int, double, string and byte. Pattern: TYPE:NAME:VALUE
    #[cfg(all(unix, not(target_os = "macos")))]
    #[arg(long, value_parser = parse_hint)]
    hint: Option<Hint>,

    /// How urgent is it. On Windows, critical keeps the toast on screen until dismissed.
    #[cfg(not(target_os = "macos"))]
    #[arg(short, long, value_enum)]
    urgency: Option<UrgencyShim>,

    /// Also prints notification to stdout
    #[cfg(all(unix, not(target_os = "macos")))]
    #[arg(short, long)]
    debug: bool,

    /// Subtitle shown below the title.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    #[arg(long)]
    subtitle: Option<String>,

    /// Image shown inside the notification.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    #[arg(long)]
    image: Option<std::path::PathBuf>,

    /// Bundle identifier of the app to send the notification as, e.g. com.apple.Terminal.
    #[cfg(target_os = "macos")]
    #[arg(short, long, conflicts_with = "app_name")]
    bundle_id: Option<String>,

    /// AppUserModelID of the app to send the notification as. Defaults to PowerShell.
    #[cfg(target_os = "windows")]
    #[arg(long)]
    app_id: Option<String>,
}

fn main() -> nResult<()> {
    let args = Cli::parse();

    match args.command {
        // #[cfg(all(unix, not(target_os = "macos")))]
        // Commands::Server => {
        //     use notify_rust::server::NotificationServer;
        //     use std::thread;
        //     let server = NotificationServer::create();
        //     thread::spawn(move || {
        //         NotificationServer::start(&server, |notification| println!("{:#?}", notification))
        //     });

        //     println!("Press enter to exit.\n");

        //     std::thread::sleep(std::time::Duration::from_millis(1_000));

        //     Notification::new()
        //         .summary("Notification Logger")
        //         .body("If you can read this in the console, the server works fine.")
        //         .show()
        //         .expect("Was not able to send initial test message");

        //     let mut _devnull = String::new();
        //     let _ = std::io::stdin().read_line(&mut _devnull);
        //     println!("Thank you for choosing toastify.");
        // }
        #[cfg(all(unix, not(target_os = "macos")))]
        Commands::Info => {
            let info = notify_rust::get_server_information()?;
            println!("server information:\n {:?}\n", info);

            let caps = notify_rust::get_capabilities()?;
            println!("capabilities:\n {:?}\n", caps);
            Ok(())
        }
        Commands::Send {
            title,
            body,
            app_name,
            sound_name,
            platform_args: extra_args,
        } => {
            let mut notification = Notification::new();

            notification.summary(&title);

            if let Some(body) = body {
                notification.body(&body);
            }

            if let Some(appname) = &app_name {
                notification.appname(appname);
            }

            if let Some(sound_name) = sound_name {
                notification.sound_name(&sound_name);
            }

            #[cfg(not(target_os = "macos"))]
            {
                if let Some(timeout) = extra_args.expire_time {
                    notification.timeout(timeout);
                }

                if let Some(urgency) = extra_args.urgency {
                    notification.urgency(urgency.into());
                }
            }

            #[cfg(all(unix, not(target_os = "macos")))]
            {
                if let Some(id) = extra_args.id {
                    notification.id(id);
                }

                if let Some(icon) = extra_args.icon {
                    notification.icon(icon.to_str().expect("Icon path is not valid unicode"));
                }

                if let Some(hint) = extra_args.hint {
                    notification.hint(hint);
                }

                if let Some(categories) = extra_args.categories {
                    for category in categories {
                        notification.hint(Hint::Category(category));
                    }
                }

                if extra_args.debug {
                    #[allow(deprecated)]
                    notification.show_debug()
                } else {
                    notification.show()
                }
                .map(|_| ())
            }

            #[cfg(any(target_os = "macos", target_os = "windows"))]
            {
                if let Some(subtitle) = &extra_args.subtitle {
                    notification.subtitle(subtitle);
                }

                if let Some(image) = &extra_args.image {
                    notification
                        .image_path(image.to_str().expect("Image path is not valid unicode"));
                }
            }

            #[cfg(target_os = "macos")]
            {
                // appname is ignored by the macOS backend, the sender is set via bundle id instead
                let bundle_id = extra_args.bundle_id.or_else(|| {
                    app_name
                        .as_deref()
                        .map(notify_rust::get_bundle_identifier_or_default)
                });
                if let Some(bundle_id) = bundle_id {
                    notify_rust::set_application(&bundle_id)?;
                }

                notification.show().map(|_| ())
            }

            #[cfg(target_os = "windows")]
            {
                if let Some(app_id) = &extra_args.app_id {
                    notification.app_id(app_id);
                }

                notification.show().map(|_| ())
            }
        }
    }
}

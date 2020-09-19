static DBUS_PARSEC_CONTROL_OBJ_PATH: &str = "/com/github/puiterwijk/DBusPARSEC/Control";
static DBUS_NAME: &str = "com.github.puiterwijk.dbus_parsec";

use anyhow::{ensure, Result};

use std::env;

use dbus::blocking::Connection;
use std::time::Duration;

mod dbus_parsec_control {
    include!(concat!(env!("OUT_DIR"), "/dbus_parsec_control_client.rs"));
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    ensure!(args.len() == 5 && args[1] == "prime", "Usage: {} prime <secret-type> <secret-group> <secret-name>", args[0]);
    let (secret_type, secret_group, secret_name) = (&args[2], &args[3], &args[4]);

    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(DBUS_NAME, DBUS_PARSEC_CONTROL_OBJ_PATH, Duration::from_millis(5000));

    use dbus_parsec_control::ComGithubPuiterwijkDBusPARSECControl;

    let pubkey = proxy.get_public_key("networkmanager", "foo")?;

    println!("Pubkey: {:?}", pubkey);

    println!("Hello control");

    Ok(())
}

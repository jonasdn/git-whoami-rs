extern crate git2;
use std::env;

type Result<T> = std::result::Result<T, git2::Error>;

//
// Return valid value from config for key, or Error.
//
fn get_entry_string_for<'a>(config: &'a git2::Config, key: &'a str) -> Result<String> {
    let entry = config.get_entry(key)?;
    match entry.value() {
        Some(val) => Ok(String::from(val)),
        None => Err(git2::Error::new(
            git2::ErrorCode::GenericError,
            git2::ErrorClass::None,
            "invalid value",
        )),
    }
}

//
// If a valid identity is found in config, return it, or Err in the format of:
//   "name <email>"
//
fn get_identity(config: git2::Config) -> Result<String> {
    let name = get_entry_string_for(&config, "user.name")?;
    let email = get_entry_string_for(&config, "user.email")?;
    Ok(format!("{} <{}>", name, email))
}

//
// First check the config from the current git repo, and if no valid
// identity is found there, try the global config.
//
fn main() {
    if let Ok(repo) = git2::Repository::discover(env::current_dir().unwrap()) {
        // first check local git config
        match get_identity(repo.config().unwrap()) {
            Ok(id) => println!("{}", id),
            Err(_) => { // if no valid identity found, try the global config
                match get_identity(git2::Config::open_default().unwrap()) {
                    Ok(id) => println!("{}", id),
                    Err(_) => eprintln!("No valid git identity found."),
                };
            }
        };
    } else {
        eprintln!("Not a git repository.");
    }
}

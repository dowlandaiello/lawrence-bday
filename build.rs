use log::error;
use std::{
    io::ErrorKind,
    process::{self, Command},
};

/// The name of the program to use for minifying CSS
/// Not necessary for vanilla CSS
const CSS_BACKEND: &'static str = "tailwindcss";

/// Name of the env variable for optimizing builds
const OPT_ENV_KEY: &'static str = "NODE_ENV";

/// Environment variable value used to fully optimize tailwind builds
const OPT_ENV_VAL: &'static str = "production";

/// This script checks for the presence of tailwindcss, and runs the optimizer
/// and the respective level for release, vs debug builds.
fn main() {
    env_logger::init();

    let mut compile_css = Command::new(CSS_BACKEND);

    // Enable nodejs production variables if in release mode
    if cfg!(debug_assertions) {
        compile_css.env(OPT_ENV_KEY, OPT_ENV_VAL);
    }

	// See config for more information
	compile_css.args(["-c", "./tailwind.config.js", "-o", "./tailwind.css"]);

    // Give more granular results if an error occurs
    if let Err(e) = compile_css.spawn() {
        match e.kind() {
			// Tailwind is REQUIRED
            ErrorKind::NotFound => error!(
                "Unable to run {CSS_BACKEND}: {e}. Please make sure {CSS_BACKEND} \
				is installed and in your PATH."
            ),
            e => error!("Unable to run {CSS_BACKEND}: {e}."),
        }

        // Exit with error
        process::exit(1);
    }
}

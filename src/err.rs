use colored::*;
use std::process::exit;

pub struct Errors;

impl Errors {

    pub fn missing_cmd() {
        println!("{} {}", "[ERR]".bold().red(), "Missing command argument".cyan().italic());
        println!("{}{}{}", "Usage".underline().yellow(), ": ".italic().bold().magenta(), "dotmate [command] [arguments]".magenta().reverse());
        exit(1);

    }

    pub fn unknown_cmd(cmd: &str) {
        println!("{} {} {}", "[ERR]".bold().red(), "Unknown command:".cyan().italic(), cmd);
        println!("{} {} {}", "[HINT]".bold().yellow(), "Consider using the command".cyan().italic(), "help".underline());
        exit(1);
    }

    pub fn wformat_usrepo(usrepo: String) {
        println!("{} {} {}", "[ERR]".bold().red(), "Wrong format for user/repo in `install`: ".cyan().italic(), usrepo);
        println!("{} {} {}", "[HINT]".bold().yellow(), "Install format: ".cyan().italic(), "username/repository".underline());
        exit(1);
    }

    pub fn wfile_install() {
        println!("{} {}", "[ERR]".bold().red(), "No .mate file found neither user/repo argument".cyan().italic());
        println!("{} {} {}", "[HINT]".bold().yellow(), "Enter in a dotmate dir or use".cyan().italic(),"username/repository".underline());
        exit(1);
    }

    pub fn missing_field(path: String) {
        println!("{} {} {}", "[ERR]".bold().red(), "Missing configuration field in .mate:".cyan().italic(), path.as_str().underline());
        exit(1);
    }

    pub fn missing_distro() {
        println!("{} {}", "[INFO]".bold().blue(), "Distro dependencies not found in .mate configuration".cyan().italic());
    }
}

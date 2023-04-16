use clap::{Args, Parser, Subcommand};
use passwords::PasswordGenerator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialiase a new password database
    Init {
        /// name of password database
        name: String,
        /// save-place of the password database
        #[arg(short, long, defaut_value_t = "~/.passrie", value_name = "PATH")]
        path: String,
    },
    /// Create s new password
    Create {
        /// name of password
        name: String,
        /// user for the password
        user: String,
        /// the web url of login
        #[arg(short, long)]
        url: Option<String>,
        /// the memo of the password
        #[arg(short, long)]
        note: Option<String>,
        /// use the generator of password to get one
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        random: Option<bool>,
        /// your favorate password
        #[arg(short, long)]
        password: Option<String>,
        /// save to which database
        #[arg(short, long, default_value_t = Some(String::from("@default")))]
        database: Option<String>,
    },
    /// Display of password
    Show {
        /// show which password
        name: String,
        /// show the password from this database
        #[arg(short, long, default_value_t = Some(String::from("@default")))]
        database: Option<String>,
    },
    /// Generate a password
    Generate {
        /// set the leight of password
        #[arg(short, long, default_value_t = Some(20))]
        leight: Option<usize>,
        /// don't use number in password
        #[arg(short, long, action = clap::ArgAction::SetFalse)]
        no_number: Option<bool>,
        /// don't use uppercase letters in password
        #[arg(short, long, action = clap::ArgAction::SetFalse)]
        no_uppercase: Option<bool>,
        /// don't use symbols in password
        #[arg(short, long, action = clap::ArgAction::SetFalse)]
        no_symbols: Option<bool>,
        /// Adaptation of password to the website
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        web_using: Option<bool>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Init {name, path} => {},
        Command::Create {name,user,url,note,random,password,database} => {},
        Command::Show {name,database} => {},
        Command::Generate {leight,no_number,no_uppercase,no_symbols,web_using} => {
            let pg = PasswordGenerator::new().length(leight.unwrap()).numbers(no_number.is_some()).lowercase_letters(true).uppercase_letters(no_uppercase.is_some()).symbols(no_symbols.is_some()).spaces(false).exclude_similar_characters(web_using.is_some()).strict(true);
            println!("{}", pg.generate_one().unwrap());
        },
    }
}

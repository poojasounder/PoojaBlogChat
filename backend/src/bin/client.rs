use clap::{Command, Arg};

extern crate backend;

#[tokio::main]
async fn main() {
    let matches= Command::new("backend")
        .about("backend commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))

                )
                .subcommand(
                    Command::new("list")
                        .about("List existing users")
                )

                .subcommand(
                    Command::new("delete")
                        .about("Delete user by ID")
                        .arg_required_else_help(true)
                        .arg(Arg::new("id").required(true))
                )
        )
        .get_matches();
    match matches.subcommand(){
        Some(("users", sub_matches)) => match sub_matches.subcommand(){
            Some(("create", sub_matches)) => backend::commands::create_user(
                sub_matches.get_one::<String>("username").unwrap().to_owned(),
                sub_matches.get_one::<String>("password").unwrap().to_owned(),
                sub_matches.get_many::<String>("roles").unwrap().map(|v| v.to_owned()).collect(),
            ).await,
            Some(("list", _)) => backend::commands::list_users().await,
            Some(("delete", _)) => backend::commands::delete_user(
                sub_matches.get_one::<i32>("username").unwrap().to_owned(),
            ).await,
            _ => {},
        },
        _ => {},
    }
}
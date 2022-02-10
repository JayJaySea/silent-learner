use card_creator;
use card_reviewer;

use clap::{
    app_from_crate,
    arg,
    AppSettings,
    Subcommand,
    App,
};

fn main() {
    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("add")
            .about("Adds new flashcard for you to learn")
            .arg(arg!(-q --question [VALUE]))
            .arg(arg!(-a --answer [VALUE])),
        )
        .subcommand(
            App::new("review")
            .about("Shows review menu to rehearse cards")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            match (sub_matches.value_of("question"), sub_matches.value_of("answer")) {
                (Some(question), Some(answer)) => {
                },
                (Some(_), None) | (None, Some(_)) => println!("Both -q and -a arguments needs to be specified, of none at all."),
                _ => println!("lol"),
            }
        },

        Some(("review", _)) => println!(
            "'card review' was used :)"
        ),

        _ => unreachable!(""),
    }
}

use clap::{
    app_from_crate,
    arg,
    AppSettings,
    App,
};

use card::{
    Menu,
    Choice,
};

use card::cli::add::AddMenu;
use card_manager::card::{
    CardManager,
    Card,
};

fn main() {
    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("add")
            .about("Adds new flashcard for you to learn.\nWhen used with --question and --answer flags, flashcard is created and program ends.\nWhen used without any flags, menu is displayed to add cards until user decides to quit.\n")
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
                    CardManager::save_card(
                        Card::new()
                            .with_question(question)
                            .with_answer(answer)
                        );
                },

                (Some(_), None) | (None, Some(_)) => 
                    println!("Both -q and -a arguments needs to be specified, of none at all."),

                _ => {
                    let mut menu = AddMenu::new();
                    loop {
                        menu.run();

                        match menu.choice() {
                            Some(c) => match c {
                                Choice::Accept => {
                                    menu.save_card();
                                },
                                Choice::Discard => (),
                                Choice::Save => {
                                    menu.save_card();
                                    break;
                                },
                                Choice::Quit => break,
                                _ => unreachable!(),
                            }

                            None => (),
                        }

                        menu.new_card();
                    }

                },
            }
        },

        Some(("review", _)) => println!(
            "'card review' was used :)"
        ),

        _ => unreachable!(""),
    }
}

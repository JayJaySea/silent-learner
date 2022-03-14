use clap::{
    app_from_crate,
    AppSettings,
    App,
};

use card::{
    Menu,
    Choice,
};

use card::cli::add::AddMenu;
use card::cli::review::ReviewMenu;

fn main() {
    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("add")
            .about("Shows menu to add cards")
        )
        .subcommand(
            App::new("review")
            .about("Shows menu to review cards")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            let mut menu = AddMenu::new();
            loop {
                menu.run();

                match menu.choice("Enter a choice: ") {
                    Some(c) => match c {
                        Choice::Accept => menu.save_card(), 
                        Choice::Discard => menu.new_card(),
                        Choice::SetLabel => menu.set_card_label(),
                        Choice::NewLabel => menu.new_card_label(),
                        Choice::Save => {
                            menu.save_card();
                            break;
                        },
                        Choice::Quit => break,
                        _ => (),
                    }

                    None => (),
                }
            }
        },

        Some(("review", _)) => {
            let mut menu = ReviewMenu::new();

            loop {
                menu.run();

                match menu.choice("Enter a choice: ") {
                    Some(c) => match c {
                        Choice::Remembered => {
                            menu.mark_card(1);
                        },
                        Choice::Forgotten => {
                            menu.mark_card(0);
                        },
                            
                        Choice::Save => {
                            menu.save_progress();
                            break;
                        },
                        Choice::Quit => break,
                        _ => (),
                    }

                    None => (),
                }
            }
        },

        _ => unreachable!(""),
    }
}

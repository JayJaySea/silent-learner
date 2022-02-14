pub mod card;



#[cfg(test)]
mod tests {
    use crate::card::{
        Card,
        CardManager,
        Mode,
    };
    use std::time::{ SystemTime, UNIX_EPOCH};

    #[test]
    fn create_new_card() {
        let card = Card::new();
        assert_eq!(card.question(), "");
        assert_eq!(card.answer(), "");
        assert_eq!(card.label(), "");
        assert_eq!(card.next_review(), 
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards").as_secs() / 86400
            );
        assert_eq!(card.level(), 0);
    }

    #[test]
    fn create_custom_card() {
        let mut card = Card::new();

        card.with_question("Hello")
            .with_answer("World")
            .with_label("!")
            .with_next_review(1234)
            .with_level(10);

        assert_eq!(card.question(), "Hello");
        assert_eq!(card.answer(), "World");
        assert_eq!(card.label(), "!");
        assert_eq!(card.next_review(), 1234);
        assert_eq!(card.level(), 10);

        card.with_question("lol");
        card.with_question("lol");
    }

    #[test]
    fn create_half_custom_card() {
        let mut card = Card::new();

        card.with_question("Hello")
            .with_answer("World")
            .with_label("!");

        assert_eq!(card.question(), "Hello");
        assert_eq!(card.answer(), "World");
        assert_eq!(card.label(), "!");
        assert_eq!(card.next_review(), 
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards").as_secs() / 86400
            );
        assert_eq!(card.level(), 0);
    }

    #[test]
    fn create_card_manager() {
        let mut manager = CardManager::new(Mode::Add);
    }
}

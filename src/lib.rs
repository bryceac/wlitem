mod item;
mod priority;
mod item_parsing_error;
mod save_vec;

pub use priority::Priority as Priority;
pub use item::Item as Item;
pub use item_parsing_error::ItemParsingError as ItemParsingError;
pub use save_vec::Save as Save;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn invalid_enum_is_none() {
        let invalid_data = Priority::from_str("hello");

        assert_eq!(invalid_data, None)
    }

    #[test]
    fn enum_parses_correctly() {
        let enum_value = Priority::from_str("low").unwrap();

        assert_eq!(enum_value, Priority::Low)
    }

    #[test]
    fn url_is_none_if_not_valid() {
        let sample_item = Item::builder()
        .set_name("Sample Item")
        .set_url("")
        .build();

        assert_eq!(sample_item.url, None)
    }

    #[test]
    fn item_creation_throws_error_when_not_enough_fields() {
        let sample_item = Item::from_str("Nintendo Switch\t1\thigh");

        assert_eq!(sample_item.err().unwrap(), ItemParsingError::TooFewFields(3))
    }

    #[test]
    fn item_creation_throws_error_when_too_many_fields() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\thigh\thttps://example.com\thello");

        assert_eq!(sample_item.err().unwrap(), ItemParsingError::TooManyFields(6))
    }

    #[test]
    fn create_item_via_text() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\thigh\thttps://example.com");

        assert!(sample_item.is_ok())
    }

    #[test]
    fn item_parses_from_text_correctly() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\thigh\thttps://example.com");
        let expected_item = Item::from("10", "Nintendo Switch", 1, "high", "https://example.com", vec![]);

        assert_eq!(sample_item.ok().unwrap(), expected_item);
    }

    #[test]
    fn item_priority_defaults_to_medium() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\t\thttps://example.com");

        assert_eq!(sample_item.ok().unwrap().priority, Priority::Medium)
    }

    #[test]
    fn save_items_to_json() {

        let sample_items = vec![
            Item::from("7746C39C-D951-4E03-840C-8E91AF0B6D1D", "Nintendo Switch 2", 1, "high", "https://example.com/nintendo_switch_2", vec![]),
            Item::from("BBDF7A43-C38B-4943-93A0-BDD6B47A2685", "Mario Kart World", 1, "high", "https://example.com/nintendo_switch_2", vec![
                "Only if I get the Switch 2".to_owned()
            ]),
            Item::from("6AA6A21C-F22D-40A5-AE1A-D7E2B4F87724", "Dragon Quest 7", 1, "", "https://example.com/nintendo_switch_2", vec![
                "Only if I get the Switch 2".to_owned()
            ])
        ];

        assert!(sample_items.save("wishlist.json").is_ok())
    }

    #[test]
    fn parse_items_from_json() {
        let expected_items = vec![
            Item::from("7746C39C-D951-4E03-840C-8E91AF0B6D1D", "Nintendo Switch 2", 1, "high", "https://example.com/nintendo_switch_2", vec![]),
            Item::from("BBDF7A43-C38B-4943-93A0-BDD6B47A2685", "Mario Kart World", 1, "high", "https://example.com/nintendo_switch_2", vec![
                "Only if I get the Switch 2".to_owned()
            ]),
            Item::from("6AA6A21C-F22D-40A5-AE1A-D7E2B4F87724", "Dragon Quest 7", 1, "", "https://example.com/nintendo_switch_2", vec![
                "Only if I get the Switch 2".to_owned()
            ])
        ];

        let decoded_items = Item::from_file("wishlist.json");

        assert_eq!(decoded_items.unwrap(), expected_items)
    }

    #[test]
    fn save_items_to_tsv() {
        let sample_items = vec![
            Item::from("7746C39C-D951-4E03-840C-8E91AF0B6D1D", "Nintendo Switch 2", 1, "high", "https://example.com/nintendo_switch_2", vec![]),
            Item::from("BBDF7A43-C38B-4943-93A0-BDD6B47A2685", "Mario Kart World", 1, "high", "https://example.com/nintendo_switch_2", vec![]),
            Item::from("6AA6A21C-F22D-40A5-AE1A-D7E2B4F87724", "Dragon Quest 7", 1, "", "https://example.com/nintendo_switch_2", vec![])
        ];

        assert!(sample_items.save_tsv("wishlist.tsv").is_ok())
    }

    #[test]
    fn parse_items_from_tsv() {
        let expected_items = vec![
            Item::from("7746C39C-D951-4E03-840C-8E91AF0B6D1D", "Nintendo Switch 2", 1, "high", "https://example.com/nintendo_switch_2", vec![]),
            Item::from("BBDF7A43-C38B-4943-93A0-BDD6B47A2685", "Mario Kart World", 1, "high", "https://example.com/nintendo_switch_2", vec![]),
            Item::from("6AA6A21C-F22D-40A5-AE1A-D7E2B4F87724", "Dragon Quest 7", 1, "", "https://example.com/nintendo_switch_2", vec![])
        ];

        assert_eq!(Item::from_tsv_file("wishlist.tsv").unwrap(), expected_items)
    }
}

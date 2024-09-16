

#[cfg(test)]
mod tests {
    use crate::objects::message::Message;

    #[test]
    fn test_message() {
        let message = Message::new(
            1,
           "Hello, World!".to_string(),
            1,
            1,
            1,
            None
        );
        println!("{}", message);
        assert_eq!(message.content, "Hello, World!");
    }
}
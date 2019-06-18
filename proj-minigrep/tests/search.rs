#[cfg(test)]
mod tests {

    use minigrep::*;

    fn gen_content() -> &'static str {
        "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog.
To tell your name the livelong day.
To an admiring bog!"
    }

    #[test]
    fn no_result() {
        let empty_vec : Vec<&str> = vec![];
        assert_eq!(
            empty_vec,
            search_sensitive("notsuchphrase", gen_content())
        );
    }

    #[test]
    fn one_result() {
        assert_eq!(
            vec!["They'd banish us, you know."],
            search_sensitive("banis", gen_content())
        );
    }

    #[test]
    fn multiple_results() {
        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!",
            ],
            search_sensitive("body", gen_content())
        );
    }

    #[test]
    fn case_sensitive() {
        assert_eq!(
            vec![
                "To tell your name the livelong day.",
                "To an admiring bog!",
            ],
            search_sensitive("To", gen_content())
        );
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(
            vec![
                "Are you nobody, too?",
                "How dreary to be somebody!",
                "To tell your name the livelong day.",
                "To an admiring bog!",
            ],
            search_insensitive("TO", gen_content())
        );
    }
}

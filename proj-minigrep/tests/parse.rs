#[cfg(test)]
mod tests {

    use minigrep::*;

    #[test]
    fn correct_number() {
        let args = vec![
            "minigrep".to_string(),
            "pattern".to_string(),
            "file-name.txt".to_string(),
        ];
        assert!(Config::new(&args).is_ok());
    }

    #[test]
    #[should_panic(expected = "Wrong number")]
    fn too_few_args() {
        let args = vec![
            "minigrep".to_string(),
            "lalalala".to_string(),
        ];
        Config::new(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "Wrong number")]
    fn too_many_args() {
        let args = vec![
            "minigrep".to_string(),
            "lalalala".to_string(),
            "hahahaha".to_string(),
            "fafafafa".to_string(),
        ];
        Config::new(&args).unwrap();
    }
}
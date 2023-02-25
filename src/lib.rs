/// Passes in doctest:
/// ```
/// let _matches = clap::Command::new("ClapExamples")
///     .version("0.0.0")
///     .author("Christoph <gaoler@electronicpanopticon.com>")
///     .about("Clap Passing Doctest")
///     .arg_required_else_help(true);
/// // assert!(true);
/// ```
///
/// Fails in doctest:
/// ```
/// let _matches = clap::Command::new("ClapExamples")
///     .version("0.0.0")
///     .author("Christoph <gaoler@electronicpanopticon.com>")
///     .about("Clap Failing Doctest")
///     .arg_required_else_help(true)
///     .get_matches();
/// // assert!(true);
/// ```
pub fn boop() -> String {
    "Boop!".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn passes_but_throws_error() {
        let _matches = clap::Command::new("ClapExamples")
            .version("1.0.0")
            .author("Christoph <gaoler@electronicpanopticon.com>")
            .about("Clap Test Passes But Throws Error")
            .arg_required_else_help(true)
            .get_matches();
        assert!(true);
    }

    #[test]
    fn passes() {
        let _matches = clap::Command::new("ClapExamples")
            .version("1.0.0")
            .author("Christoph <gaoler@electronicpanopticon.com>")
            .about("Clap Test Passes")
            .arg_required_else_help(true);
        assert!(true);
    }
}

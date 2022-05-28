use chrono::NaiveDate;
use regex::Regex;

#[derive(Default, Debug)]
pub struct PostId(pub String);

impl PostId {
    pub fn new(str: String) -> anyhow::Result<Self> {
        // ref. https://regexland.com/regex-dates/
        let regex =
            Regex::new(r"^(\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01]))_\d{2}$").unwrap();
        anyhow::ensure!(
            regex.is_match(&str),
            "The file name must have the specific format."
        );
        let date = regex.captures(&str).unwrap().get(1).unwrap().as_str();
        let is_valid_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok();
        anyhow::ensure!(is_valid_date, "The date is invalid.");

        Ok(Self(str))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("2020-01-01_01")]
    #[test_case("2020-01-01_10")]
    #[test_case("2020-01-01_99")]
    #[test_case("2020-12-12_01")]
    fn success(input: &str) {
        assert!(PostId::new(input.to_string()).is_ok());
    }

    #[test_case("2020-01-01-01"; "Unexpected delimiter(hyphen)")]
    #[test_case("2020_01_01-01"; "Unexpected delimiter(underbar)")]
    #[test_case("10-01-01_01"; "Invalid year")]
    #[test_case("2020-13-01_01"; "Invalid month")]
    #[test_case("2020-04-90_01"; "Invalid day")]
    fn failure(input: &str) {
        assert!(PostId::new(input.to_string()).is_err());
    }
}

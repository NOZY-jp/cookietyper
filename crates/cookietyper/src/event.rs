#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Event {
    InvalidCommand,
    EarnCookies(i32),
    ShowCookiesAmount,
    ShowCps,
}

impl From<String> for Event {
    fn from(s: String) -> Self {
        let s = s.trim();

        if !s.starts_with("/") {
            let s_num = s.len();
            return Event::EarnCookies(s_num as i32);
        }

        match s {
            "/cc" => Event::ShowCookiesAmount,
            "/cps" => Event::ShowCps,
            _ => Event::InvalidCommand,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Event;

    #[test]
    fn test_from_str() {
        assert_eq!(Event::from("12345".to_string()), Event::EarnCookies(5));
        assert_eq!(Event::from("123".to_string()), Event::EarnCookies(3));
        assert_eq!(Event::from("/cc".to_string()), Event::ShowCookiesAmount);
        assert_eq!(Event::from("/cps".to_string()), Event::ShowCps);
        assert_eq!(Event::from("/meaw".to_string()), Event::InvalidCommand);
        assert_ne!(Event::from("123".to_string()), Event::EarnCookies(5));
    }
}

use error::*;
use serde_json;

const CRLF: &'static str = "\r\n";
const STRING_PREFIX: &'static str = "+";
const INT_PREFIX: &'static str = ":";
const JSON_PERFIX: &'static str = "?";

macro_rules! serialize_template { () => ("{prefix}{value}{crlf}") }

// This type represents one data in the database.
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    String(String),
    Int(i64),
    JSON(serde_json::Value),
}

impl Data {
    pub fn into_string(self) -> String {
        match self {
            Data::String(string) => Self::serialize_string(string),
            Data::Int(int) => Self::serialize_int(int),
            Data::JSON(json) => Self::serialize_json(json),
        }
    }

    pub fn try_from(string: String) -> Result<Data> {
        if string.len() < 2 || !string.ends_with(CRLF) {
            return Err(Error::new(ErrorKind::InvalidSerializedString));
        }

        if string.starts_with(STRING_PREFIX) {
            Ok(Self::from_string(string)?)
        } else if string.starts_with(INT_PREFIX) {
            Ok(Self::from_int(string)?)
        } else if string.starts_with(JSON_PERFIX) {
            Ok(Self::from_json(string)?)
        } else {
            Err(Error::new(ErrorKind::InvalidSerializedString))
        }
    }

    fn from_string(s: String) -> Result<Data> {
        Ok(Data::String(String::from(&s[1..s.len() - 2])))
    }

    fn from_int(s: String) -> Result<Data> {
        Ok(Data::Int(s[1..s.len() - 2].parse::<i64>()?))
    }

    fn from_json(s: String) -> Result<Data> {
        Ok(Data::JSON(serde_json::from_str(&s[1..s.len() - 2])?))
    }

    fn serialize_string(s: String) -> String {
        format!(serialize_template!(),
                prefix = STRING_PREFIX,
                value = s,
                crlf = CRLF)
    }

    fn serialize_int(i: i64) -> String {
        format!(serialize_template!(),
                prefix = INT_PREFIX,
                value = i,
                crlf = CRLF)
    }

    fn serialize_json(json: serde_json::Value) -> String {
        format!(serialize_template!(),
                prefix = JSON_PERFIX,
                value = json.to_string(),
                crlf = CRLF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(Data::String(String::from("test_\r\nfrom_string")),
                   Data::from_string(String::from("+test_\r\nfrom_string\r\n")).unwrap());
    }

    #[test]
    fn test_from_int() {
        assert_eq!(Data::Int(22),
                   Data::from_int(String::from(":22\r\n")).unwrap());
    }

    #[test]
    fn test_from_invalid_int() {
        assert!(Data::from_int(String::from(":22invalid888\r\n")).is_err());
    }

    #[test]
    fn test_serialize_string() {
        assert_eq!("+test\r\n_serialize\r\n_string\r\n",
                   Data::serialize_string(String::from("test\r\n_serialize\r\n_string")));
    }

    #[test]
    fn test_serialize_int() {
        assert_eq!(":666\r\n", Data::serialize_int(666));
    }

    #[test]
    fn test_serialize_json() {
        assert_eq!("?{\"age\":18,\"name\":\"David\"}\r\n",
                   Data::serialize_json(json!({"name": "David","age": 18})));
    }

    #[test]
    fn test_serializble_into() {
        assert_eq!("+666\r\n", Data::String(String::from("666")).into_string());
        assert_eq!(":666\r\n", Data::Int(666).into_string());
    }

    #[test]
    fn test_try_from_invalid() {
        assert!(Data::try_from(String::from("")).is_err());
        assert!(Data::try_from(String::from("\r\n")).is_err());
        assert!(Data::try_from(String::from("11111")).is_err());
    }

    #[test]
    fn test_try_from_string() {
        assert_eq!(Data::String(String::from("666")),
                   Data::try_from(String::from("+666\r\n")).unwrap());
    }

    #[test]
    fn test_try_from_int() {
        assert_eq!(Data::Int(666),
                   Data::try_from(String::from(":666\r\n")).unwrap());
    }

    #[test]
    fn test_try_from_json() {
        assert_eq!(Data::JSON(json!({"name": "David","age": 18})),
                   Data::try_from(String::from("?{\"age\":18,\"name\":\"David\"}\r\n")).unwrap());
    }
}

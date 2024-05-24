// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 characters.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

#[derive(Debug, thiserror::Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    TitleEmpty,
    #[error("The title cannot be longer than 50 characters")]
    TitleTooLong,
}

// without `thiserror` - impl `Display` on own:
// #[derive(Debug)]
// pub enum TicketTitleError {
//     TitleEmpty,
//     TitleTooLong,
// }

// impl std::fmt::Display for TicketTitleError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TicketTitleError::TitleEmpty => write!(f, "Title cannot be empty"),
//             TicketTitleError::TitleTooLong => {
//                 write!(f, "Title cannot be longer than 50 characters")
//             }
//         }
//     }
// }

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_title(&value)?;

        Ok(TicketTitle(value))
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_title(value)?;

        Ok(TicketTitle(value.to_string()))
    }
}

fn validate_title(title: &str) -> Result<(), TicketTitleError> {
    match title {
        "" => Err(TicketTitleError::TitleEmpty),
        title if title.len() > 50 => Err(TicketTitleError::TitleTooLong),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The title cannot be longer than 50 characters"
        );
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}

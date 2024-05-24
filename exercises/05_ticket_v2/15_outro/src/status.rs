// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, thiserror::Error)]
#[error(
    "{} is an invalid status. Please provide a valid status (ToDo, InProgress, Done).",
    status
)]
pub struct StatusError {
    status: String,
}

// without `thiserror`

// #[derive(Debug)]
// pub struct StatusError {
//     stauts: String,
// }

// impl std::fmt::Display for StatusError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{} is an invalid status. Please provide a valid status (ToDo, InProgress, Done).",
//             self.status
//         )
//     }
// }

impl TryFrom<String> for Status {
    type Error = StatusError;

    // fn try_from(value: String) -> Result<Self, Self::Error> {
    //     match value.to_lowercase().as_str() {
    //         "todo" => Ok(Status::ToDo),
    //         "inprogress" => Ok(Status::InProgress),
    //         "done" => Ok(Status::Done),
    //     }
    // }

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_status(&value)
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_status(value)
    }
}

fn validate_status(status: &str) -> Result<Status, StatusError> {
    match status.to_lowercase().as_str() {
        "todo" => Ok(Status::ToDo),
        "inprogress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        _ => Err(StatusError {
            status: status.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}

use std::{
    env, fmt, error,
    fs::File,
    io::{self, BufRead},
    rc::Rc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    Short,
    Long,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Defense {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faint {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Body {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Combination {
    pub description: String,
    pub distance: Distance,
    pub defense: Defense,
    pub faint: Faint,
    pub body: Body,
    pub url: Option<String>,
}

#[derive(Debug)]
pub enum CombinationError {
    IoError(std::io::Error),
    ParseError(String),
}

const FIELD_COUNT : usize = 6;
const DELIMITER : &str = ";";
const COMMENT : &str = "#";
const SHORT : &str = "short";
const LONG : &str = "long";
const YES : &str = "yes";
const NO : &str = "no";

impl Combination {
    fn new(
        description: String,
        distance: Distance,
        defense: Defense,
        faint: Faint,
        body: Body,
        url: Option<String>
    ) -> Combination {
        Combination {
            description,
            distance,
            defense,
            faint,
            body,
            url,
        }
    }
}

impl fmt::Display for CombinationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CombinationError::IoError(e) => write!(f, "I/O error: {}", e),
            CombinationError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl From<io::Error> for CombinationError {
    fn from(error: io::Error) -> Self {
        CombinationError::IoError(error)
    }
}

impl error::Error for CombinationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CombinationError::IoError(e) => Some(e),
            CombinationError::ParseError(_) => None,
        }
    }
}

pub fn load_data(path: &str) -> Result<Vec<Rc<Combination>>, CombinationError> {
    println!("CWD is {:?}, path is {:?}", env::current_dir()?, path);
    let mut data: Vec<Rc<Combination>> = vec![];
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?.trim().to_owned();
        if line.starts_with(COMMENT) || line.is_empty() {
            continue;
        }
        let combination = parse_combination(&line)?;
        data.push(combination);
    }
    Ok(data)
}

fn parse_combination(line: &str) -> Result<Rc<Combination>, CombinationError> {
    let el: Vec<&str> = line.split(DELIMITER).collect();
    if el.len() != FIELD_COUNT {
        return Err(CombinationError::ParseError(format!(
            "Expect {} elements delimited by {} in {:?}",
            FIELD_COUNT, DELIMITER, line
        )));
    }
    let description = el[0].trim().to_owned();
    let distance = el[1].trim();
    let distance = if distance.eq_ignore_ascii_case(LONG) {
        Distance::Long
    } else if distance.eq_ignore_ascii_case(SHORT) {
        Distance::Short
    } else {
        return Err(CombinationError::ParseError(format!(
            "Unknown distance {:?} in {:?}",
            distance, line
        )));
    };
    let defense = el[2];
    let defense = match parse_yes_no(defense, Defense::Yes, Defense::No) {
        Some(val) => val,
        None => {
            return Err(CombinationError::ParseError(format!(
                "Unknown defense {:?} in {:?}",
                defense, line
            )));
        }
    };
    let faint = el[3];
    let faint = match parse_yes_no(faint, Faint::Yes, Faint::No) {
        Some(val) => val,
        None => {
            return Err(CombinationError::ParseError(format!(
                "Unknown faint {:?} in {:?}",
                faint, line
            )));
        }
    };
    let body = el[4];
    let body = match parse_yes_no(body, Body::Yes, Body::No) {
        Some(val) => val,
        None => {
            return Err(CombinationError::ParseError(format!(
                "Unknown body {:?} in {:?}",
                body, line
            )));
        }
    };
    let url = el[5];
    let url = if url.is_empty() {
        None
    } else {
        Some(url.trim().to_owned())
    };
    Ok(Rc::new(Combination::new(
        description,
        distance,
        defense,
        faint,
        body,
        url,
    )))
}

fn parse_yes_no<T>(field: &str, yes: T, no: T) -> Option<T> {
    let field = field.trim();
    if field.eq_ignore_ascii_case(YES) {
        Some(yes)
    } else if field.eq_ignore_ascii_case(NO) {
        Some(no)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import all functions and types from the parent module

    #[test]
    fn test_parse_combination() {
        assert_eq!(
            parse_combination("1-1-2-step_back-2; Long;  Yes;  No;  No; https://example.com")
                .unwrap()
                .as_ref(),
            &Combination {
                description: "1-1-2-step_back-2".to_owned(),
                distance: Distance::Long,
                defense: Defense::Yes,
                faint: Faint::No,
                body: Body::No,
                url: Some("https://example.com".to_owned()),
            }
        );
    }

    #[test]
    fn test_parse_error_five_elements() {
        assert_eq!(
            parse_combination("1-1-2-step_back-2; Long;  Yes")
                .unwrap_err()
                .to_string(),
            "Parse error: Expect five elements delimited by ; in \"1-1-2-step_back-2; Long;  Yes\""
                .to_owned()
        );
    }

    #[test]
    fn test_parse_error_distance() {
        assert_eq!(
            parse_combination("1-1-2-step_back-2; XXX;  Yes;  No;  No;")
                .unwrap_err()
                .to_string(),
            "Parse error: Unknown distance \"XXX\" in \"1-1-2-step_back-2; XXX;  Yes;  No;  No;\""
                .to_owned()
        );
    }

    #[test]
    fn test_parse_error_defence() {
        assert_eq!(
            parse_combination("1-1-2-step_back-2; Long;  XXX;  No;  No;")
                .unwrap_err()
                .to_string(),
            "Parse error: Unknown defense \"  XXX\" in \"1-1-2-step_back-2; Long;  XXX;  No;  No;\""
                .to_owned()
        );
    }

    #[test]
    fn test_parse_error_faint() {
        assert_eq!(
            parse_combination("1-1-2-step_back-2; Long;  Yes;  XXX;  No;")
                .unwrap_err()
                .to_string(),
            "Parse error: Unknown faint \"  XXX\" in \"1-1-2-step_back-2; Long;  Yes;  XXX;  No;\""
                .to_owned()
        );
    }

    #[test]
    fn test_parse_error_body() {
        assert_eq!(
            parse_combination("1-1-2-step_back-2; Long;  Yes;  No;  XXX;")
                .unwrap_err()
                .to_string(),
            "Parse error: Unknown body \"  XXX\" in \"1-1-2-step_back-2; Long;  Yes;  No;  XXX;\""
                .to_owned()
        );
    }

    #[test]
    fn test_load_data() {
        const PATH: &str = "./combinations.txt";
        assert!(load_data(PATH).is_ok());
    }
}

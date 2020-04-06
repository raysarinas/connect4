use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Hash, Eq, Copy)]
pub enum Token {
    R,
    Y,
    T,
    O
}

impl Token {
    pub fn opposite(&self) -> Self {
        match self {
            Token::T => Token::O,
            Token::O => Token::T,
            _ => panic!("Token Opposite Called on R or Y token")
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::R => write!(f, "R"),
            Token::Y => write!(f, "Y"),
            Token::T => write!(f, "T"),
            Token::O => write!(f, "O")
        }
    }

}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s.to_uppercase()[..];

        match s {
            "T" => Ok(Token::T),
            "O" => Ok(Token::O),
            "R" => Ok(Token::R),
            "Y" => Ok(Token::Y),
            _ => Err(())
        }
    }
}
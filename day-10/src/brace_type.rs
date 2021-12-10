#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BraceType {
    Regular,
    Square,
    Curly,
    Angle,
}

impl BraceType {
    pub fn from_char(c: char) -> BraceType {
        match c {
            '(' | ')' => BraceType::Regular,
            '[' | ']' => BraceType::Square,
            '{' | '}' => BraceType::Curly,
            '<' | '>' => BraceType::Angle,
            _ => panic!("Invalid char"),
        }
    }

    pub fn is_open(c: char) -> bool {
        match c {
            '(' | '[' | '{' | '<' => true,
            ')' | ']' | '}' | '>' => false,
            _ => panic!("Invalid char"),
        }
    }

    pub fn get_score_part_1(&self) -> i32 {
        match self {
            BraceType::Regular => 3,
            BraceType::Square => 57,
            BraceType::Curly => 1197,
            BraceType::Angle => 25137,
        }
    }

    pub fn get_score_part_2(&self) -> usize {
        match self {
            BraceType::Regular => 1,
            BraceType::Square => 2,
            BraceType::Curly => 3,
            BraceType::Angle => 4,
        }
    }
}

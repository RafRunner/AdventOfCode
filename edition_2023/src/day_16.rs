use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
enum RayType {
    Upward,
    Downward,
    Rightward,
    Leftward,
}

#[derive(Debug, PartialEq, Eq)]
enum MirrorType {
    Foward,
    Back,
}

impl MirrorType {
    fn parse(char: char) -> Self {
        match char {
            '/' => Self::Foward,
            '\\' => Self::Back,
            _ => panic!("{char} is not a valid mirror"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SplitterType {
    Horizontal,
    Vertical,
}

impl SplitterType {
    fn parse(char: char) -> Self {
        match char {
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            _ => panic!("{char} is not a valid splitter"),
        }
    }
}

#[derive(Debug)]
enum TileType {
    Empty(HashSet<RayType>),
    Mirror(MirrorType),
    Splitter(SplitterType),
}

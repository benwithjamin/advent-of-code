use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use num::{CheckedAdd, CheckedSub, Integer, Unsigned};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: FromStr> FromStr for Coordinate<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err("Input must be in the format 'x, y'".to_string());
        }

        let x = parts[0]
            .trim()
            .parse::<T>()
            .map_err(|_| format!("Failed to parse x: '{}'", parts[0]))?;
        let y = parts[1]
            .trim()
            .parse::<T>()
            .map_err(|_| format!("Failed to parse y: '{}'", parts[1]))?;

        Ok(Coordinate { x, y })
    }
}

impl<T: Display + Debug> Debug for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl<T: Display> Display for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Unsigned + CheckedAdd<Output = T>> CheckedAdd for Coordinate<T> {
    fn checked_add(&self, other: &Self) -> Option<Self> {
        if let (Some(x), Some(y)) = (self.x.checked_add(&other.x), self.y.checked_add(&other.y)) {
            Some(Self { x, y })
        } else {
            None
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Unsigned + CheckedSub<Output = T>> CheckedSub for Coordinate<T> {
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        if let (Some(x), Some(y)) = (self.x.checked_sub(&other.x), self.y.checked_sub(&other.y)) {
            Some(Self { x, y })
        } else {
            None
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Coordinate<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Eq + PartialEq + Ord + Copy> Ord for Coordinate<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl<T: Ord + PartialOrd> PartialOrd for Coordinate<T>
where
    T: std::marker::Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Integer + Copy> Coordinate<T> {
    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            Coordinate {
                x: self.x - num::one(),
                y: self.y,
            },
            Coordinate {
                x: self.x + num::one(),
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x,
                y: self.y + num::one(),
            },
        ]
    }

    pub fn extended_neighbours(&self) -> Vec<Self> {
        vec![
            Coordinate {
                x: self.x - num::one(),
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x - num::one(),
                y: self.y,
            },
            Coordinate {
                x: self.x - num::one(),
                y: self.y + num::one(),
            },
            Coordinate {
                x: self.x,
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x,
                y: self.y + num::one(),
            },
            Coordinate {
                x: self.x + num::one(),
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x + num::one(),
                y: self.y,
            },
            Coordinate {
                x: self.x + num::one(),
                y: self.y + num::one(),
            },
        ]
    }

    pub fn neighbour(&self, direction: Direction) -> Self {
        match direction {
            Direction::NorthWest => Coordinate {
                x: self.x - num::one(),
                y: self.y - num::one(),
            },
            Direction::North => Coordinate {
                x: self.x,
                y: self.y - num::one(),
            },
            Direction::NorthEast => Coordinate {
                x: self.x + num::one(),
                y: self.y - num::one(),
            },
            Direction::East => Coordinate {
                x: self.x + num::one(),
                y: self.y,
            },
            Direction::SouthEast => Coordinate {
                x: self.x + num::one(),
                y: self.y + num::one(),
            },
            Direction::South => Coordinate {
                x: self.x,
                y: self.y + num::one(),
            },
            Direction::SouthWest => Coordinate {
                x: self.x - num::one(),
                y: self.y + num::one(),
            },
            Direction::West => Coordinate {
                x: self.x - num::one(),
                y: self.y,
            },
        }
    }

    pub fn direction_to(&self, other: &Self) -> Option<Direction> {
        if *self == *other {
            None
        } else if self.x - num::one() == other.x && self.y == other.y {
            Some(Direction::West)
        } else if self.x + num::one() == other.x && self.y == other.y {
            Some(Direction::East)
        } else if self.x == other.x && self.y - num::one() == other.y {
            Some(Direction::North)
        } else if self.x == other.x && self.y + num::one() == other.y {
            Some(Direction::South)
        } else {
            None
        }
    }
}

impl<T: Integer + Copy + CheckedSub + CheckedAdd + Unsigned> Coordinate<T> {
    pub fn checked_neighbour(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::NorthWest => self.checked_sub(&Coordinate {
                x: num::one::<T>(),
                y: num::one::<T>(),
            }),
            Direction::North => self.checked_sub(&Coordinate {
                x: num::zero::<T>(),
                y: num::one::<T>(),
            }),
            Direction::NorthEast => {
                if let Some(loc) = self.checked_neighbour(Direction::North) {
                    loc.checked_neighbour(Direction::East)
                } else {
                    None
                }
            }
            Direction::East => self.checked_add(&Coordinate {
                x: num::one::<T>(),
                y: num::zero::<T>(),
            }),
            Direction::SouthEast => self.checked_add(&Coordinate {
                x: num::one::<T>(),
                y: num::one::<T>(),
            }),
            Direction::South => self.checked_add(&Coordinate {
                x: num::zero::<T>(),
                y: num::one::<T>(),
            }),
            Direction::SouthWest => {
                if let Some(loc) = self.checked_neighbour(Direction::South) {
                    loc.checked_neighbour(Direction::West)
                } else {
                    None
                }
            }
            Direction::West => self.checked_sub(&Coordinate {
                x: num::one::<T>(),
                y: num::zero::<T>(),
            }),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    North,
    NorthEast,
    NorthWest,
    East,
    South,
    SouthEast,
    SouthWest,
    West,
}

pub struct MapData<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<T>>,
}

impl<T> MapData<T>
where
    T: Copy,
{
    pub fn new(width: impl Into<usize>, height: impl Into<usize>, data: Vec<Vec<T>>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
            data,
        }
    }

    pub fn new_from_str(input_data: &str) -> Result<Self, String>
    where
        T: From<char>,
    {
        return Self::new_from_str_with_parser(input_data, |c| Ok(T::from(c)));
    }

    pub fn new_from_str_with_parser<F>(input_data: &str, parse_fn: F) -> Result<Self, String>
    where
        F: Fn(char) -> Result<T, String>,
    {
        let width = input_data
            .lines()
            .next()
            .ok_or_else(|| "Input data did not contain any lines".to_string())?
            .len();

        if !input_data.lines().all(|line| line.len() == width) {
            return Err(format!("Not all lines are the same length ({width})"));
        }

        let height = input_data.lines().count();

        let data: Result<Vec<Vec<T>>, String> = input_data
            .lines()
            .enumerate()
            .map(|(_, line)| {
                line.chars()
                    .enumerate()
                    .map(|(_, c)| parse_fn(c))
                    .collect::<Result<Vec<T>, String>>()
            })
            .collect();

        let data = data?;

        Ok(Self {
            width,
            height,
            data,
        })
    }

    pub fn get<C: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(
        &self,
        coordinate: &Coordinate<C>,
    ) -> Option<T> {
        if (0..self.width).contains(&coordinate.x.into())
            && (0..self.height).contains(&coordinate.y.into())
        {
            return Some(unsafe { self.unchecked_get(coordinate) });
        }

        None
    }

    pub unsafe fn unchecked_get<C: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(
        &self,
        coordinate: &Coordinate<C>,
    ) -> T {
        self.data[coordinate.y.into()][coordinate.x.into()]
    }

    pub fn get_valid_adjacent_coordinates<
        C: Integer + PartialEq + PartialOrd + Into<usize> + From<usize> + Copy,
    >(
        &self,
        coordinate: &Coordinate<C>,
    ) -> Vec<Coordinate<C>> {
        let adjacent: Vec<Coordinate<C>> = coordinate
            .neighbours()
            .into_iter()
            .filter(|coordinate| {
                (0..self.width).contains(&coordinate.x.into())
                    && (0..self.height).contains(&coordinate.y.into())
            })
            .collect();

        adjacent
    }

    pub fn enumerate<C: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(
        &self,
    ) -> impl Iterator<Item = (Coordinate<C>, &T)> {
        self.data.iter().enumerate().flat_map(|(row_index, row)| {
            row.iter().enumerate().map(move |(column_index, item)| {
                (
                    Coordinate {
                        x: column_index.into(),
                        y: row_index.into(),
                    },
                    item,
                )
            })
        })
    }
}

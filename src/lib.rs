#![allow(unused)]

use std::{collections::VecDeque, io::{self, Write}};
use crossterm::{execute, terminal::{ScrollUp, SetSize, size}};
use std::time::Duration;

pub struct SnakeGame {
    score: u32,
    rows: u32,
    columns: u32,
    snake: VecDeque<Point2D>,
    food_location: Point2D,
    face_direction: Option<Direction>
}

impl SnakeGame {
    pub fn move_snake(self: &Self, direction: Direction) -> Result<(), String> {
        match direction {
            Direction::Left => Ok(()),
            Direction::Right => Ok(()),
            Direction::Up => Ok(()),
            Direction::Down => Ok(()),
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Left, Right, Up, Down
}

impl Direction {
    pub fn are_opposite(self: &Self, other: &Direction) -> bool {
        if self == &Direction::Left && other == &Direction::Right { true }
        else if self == &Direction::Right && other == &Direction::Left { true }
        else if self == &Direction::Up && other == &Direction::Down { true }
        else if self == &Direction::Down && other == &Direction::Up { true }
        else { false }
    }
}

struct Point2D {
    x: u32,
    y: u32
}
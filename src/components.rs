use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

// Нужен для фильтрации объектов управляемых с клавиатуры
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

// Тип для отправки в систему работы с клавиатурой
pub enum MovementCommand {
    Stop,
    Move(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Текущее положение объекта
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

// Текущее положение и скорость объекта
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    // Конкретный лист спрайтов для рендеринга
    pub sprite_sheet: usize,
    // Текущая область спрайт-листа для визуализации
    pub region: Rect,
}

// Текущая скорость и направление объекта
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

// Состояние анимации объекта
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MoveAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

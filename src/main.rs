mod components;
mod systems;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

use specs::prelude::*;

use crate::components::*;
use crate::systems::animator::*;
use crate::systems::keyboard::*;
use crate::systems::physics::*;
use crate::systems::renderer::*;



fn character_animation_frames(
    sprite_sheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            sprite_sheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;

    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("World of Conscription", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();

    // Так как зависимостей между системами пока нет оставляю массив пустым
    let mut dispatcher = DispatcherBuilder::new()
        .with(Keyboard, "Keyboard", &[])
        .with(Physics, "Physics", &["Keyboard"])
        .with(Animator, "Animator", &["Keyboard"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world.res);
    Sd::setup(&mut world.res);

    // Инициализация ресурсов
    let movement_command: Option<MovementCommand> = None;
    world.add_resource(movement_command);

    // Подключение используемых текстур
    let textures = [texture_creator.load_texture("assets/bardo.png")?];

    // Описание текстуры игрока
    let player_sprite_sheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);
    let player_animation = MoveAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            player_sprite_sheet,
            player_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            player_sprite_sheet,
            player_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            player_sprite_sheet,
            player_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            player_sprite_sheet,
            player_top_left_frame,
            Direction::Left,
        ),
    };

    world
        .create_entity()
        .with(KeyboardControlled)
        // В этом мире координаты 0, 0 это центр мира
        .with(Position(Point::new(0, 0)))
        // Начальная скорость игрока равна 0
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        let mut movement_command = None;

        // Обработка событий нажатия клавиш
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Stop);
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // Перерисовка
        render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &textures,
            world.system_data(),
        )?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

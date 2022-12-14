use euclid::{point2, vec2};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::collections::HashMap;
use structopt::StructOpt;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const DATA: &str = include_str!("../../data/day14.txt");
const SAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

type Point = euclid::default::Point2D<isize>;
type Vector = euclid::default::Vector2D<isize>;
type Rect = euclid::default::Rect<isize>;
type Box = euclid::default::Box2D<isize>;
type RockList = Vec<Vec<Point>>;

const SAND_ORIGIN: Point = point2(500, 0);

struct LineIter {
    current: Point,
    end: Point,
    delta: Vector,
}

impl LineIter {
    fn new(start: Point, end: Point) -> Self {
        let b = Box::from_points(&[start, end]);
        let start = b.min;
        let end = b.max;
        let mut delta = end - start;
        if delta.x > 0 {
            delta.x /= delta.x;
        }
        if delta.y > 0 {
            delta.y /= delta.y;
        }
        Self {
            current: start,
            delta,
            end,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.current.x > self.end.x || self.current.y > self.end.y {
            return None;
        }
        let next = self.current;
        self.current += self.delta;
        Some(next)
    }
}

#[derive(Debug)]
enum Block {
    Rock,
    Sand,
}

#[derive(Debug)]
struct RockFall {
    bounds: Rect,
    blocks: HashMap<Point, Block>,
    falling_sand: Option<Point>,
    floor: isize,
    units: usize,
}

impl RockFall {
    fn new(list: RockList, floor: isize) -> Self {
        let bounds = Rect::from_points(list.iter().flatten());
        let mut blocks = HashMap::new();
        for rock in list {
            for i in 0..rock.len() - 1 {
                let iter = LineIter::new(rock[i], rock[i + 1]).map(|p| (p, Block::Rock));
                blocks.extend(iter);
            }
        }
        Self {
            bounds,
            blocks,
            falling_sand: Some(SAND_ORIGIN),
            floor: floor.max(bounds.max_y() + 2),
            units: 1,
        }
    }

    fn render(&mut self, pixmap: &mut Pixmap) {
        pixmap.fill(Color::BLACK);
        let mut rock_paint = Paint::default();
        rock_paint.set_color_rgba8(90, 90, 90, 255);
        rock_paint.anti_alias = true;

        let mut sand_paint = Paint::default();
        sand_paint.set_color_rgba8(255, 195, 0, 255);
        sand_paint.anti_alias = true;
        let identity = Transform::from_scale(5.0, 5.0).post_translate(25.0, 25.0);

        for (block, block_type) in self
            .blocks
            .iter()
            .chain(self.falling_sand.as_ref().map(|p| (p, &Block::Sand)))
        {
            let p = *block - self.bounds.origin;
            let r = tiny_skia::Rect::from_xywh(p.x as f32, p.y as f32, 1.0, 1.0).unwrap();
            let path1 = PathBuilder::from_rect(r);
            let paint = match block_type {
                Block::Rock => &rock_paint,
                Block::Sand => &sand_paint,
            };
            pixmap.fill_path(&path1, paint, FillRule::Winding, identity, None);
        }
    }

    fn step(&mut self) -> Option<usize> {
        const DELTAS: &[Vector] = &[vec2(0, 1), vec2(-1, 1), vec2(1, 1)];
        if let Some(falling_sand) = self.falling_sand.as_mut() {
            for delta in DELTAS {
                let new_pos = *falling_sand + *delta;
                if new_pos.y != self.floor && !self.blocks.contains_key(&new_pos) {
                    *falling_sand = new_pos;
                    if new_pos.y < self.bounds.max_y() + 10 {
                        return None;
                    } else {
                        return Some(self.units - 1);
                    }
                }
            }
            self.blocks.insert(*falling_sand, Block::Sand);
            if *falling_sand == SAND_ORIGIN {
                return Some(self.units);
            }
            *falling_sand = SAND_ORIGIN;
            self.units += 1;
            return None;
        }
        None
    }
}

fn parse_point(s: &str) -> Point {
    let mut parts = s
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::ok)
        .map(Option::unwrap_or_default);

    point2(
        parts.next().unwrap_or_default(),
        parts.next().unwrap_or_default(),
    )
}

fn parse(s: &str) -> RockList {
    s.lines()
        .map(|s| s.split(" -> ").map(parse_point).collect::<Vec<_>>())
        .collect()
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

#[derive(Debug, StructOpt)]
#[structopt(name = "day14", about = "Falling sand.")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,

    /// No graphics
    #[structopt(long)]
    headless: bool,

    /// Floor level
    #[structopt(long, default_value = "11")]
    floor: isize,
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let opt = Opt::from_args();

    let rocklist = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

    let mut rockfall = RockFall::new(rocklist, opt.floor);

    if opt.headless {
        loop {
            if let Some(units) = rockfall.step() {
                println!("units = {units}");
                break;
            }
        }
    } else {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            WindowBuilder::new()
                .with_title("Day 14")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };

        let mut drawing = Pixmap::new(1000, 1000).unwrap();

        event_loop.run(move |event, _, control_flow| {
            if let Some(units) = rockfall.step() {
                println!("units = {units}");
            }
            rockfall.render(&mut drawing);

            if let Event::RedrawRequested(_) = event {
                pixels.get_frame_mut().copy_from_slice(drawing.data());
                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    pixels.resize_surface(size.width, size.height);
                }

                window.request_redraw();
            }
        });
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use euclid::rect;

    #[test]
    fn test_parse() {
        let l = parse(SAMPLE);
        assert_eq!(
            l,
            vec![
                vec![point2(498, 4), point2(498, 6), point2(496, 6)],
                vec![
                    point2(503, 4),
                    point2(502, 4),
                    point2(502, 9),
                    point2(494, 9)
                ]
            ]
        );

        let rockfall = RockFall::new(l, isize::MAX);
        assert_eq!(rockfall.bounds, rect(494, 4, 9, 5));
    }

    #[test]
    fn test_line_iter() {
        let points: Vec<_> = LineIter::new(point2(498, 4), point2(498, 6)).collect();
        dbg!(&points);
        assert_eq!(points, [point2(498, 4,), point2(498, 5,), point2(498, 6,)]);
        let points: Vec<_> = LineIter::new(point2(498, 6), point2(496, 6)).collect();
        assert_eq!(points, [point2(496, 6,), point2(497, 6,), point2(498, 6,)]);
    }

    #[test]
    fn test_part_1() {
        let l = parse(SAMPLE);
        let mut rockfall = RockFall::new(l, isize::MAX);
        loop {
            if let Some(amount) = rockfall.step() {
                assert_eq!(amount, 24);
                break;
            }
        }
    }

    #[test]
    fn test_part_2() {
        let l = parse(SAMPLE);
        let mut rockfall = RockFall::new(l, 0);
        loop {
            if let Some(amount) = rockfall.step() {
                assert_eq!(amount, 93);
                break;
            }
        }
    }
}

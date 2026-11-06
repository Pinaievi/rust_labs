#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rectangle { x, y, width, height }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn move_to(&self, x: i32, y: i32) -> Rectangle {
        Rectangle::new(x, y, self.width, self.height)
    }

    fn offset(&self, dx: i32, dy: i32) -> Rectangle {
        Rectangle::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    fn resize(&self, width: u32, height: u32) -> Rectangle {
        Rectangle::new(self.x, self.y, width, height)
    }
}

struct Chain {
    rectangles: Vec<Rectangle>,
}

impl Chain {
    fn new(first: Rectangle) -> Self {
        Chain {
            rectangles: vec![first],
        }
    }

    fn last(&self) -> Rectangle {
        *self.rectangles.last().unwrap()
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let new_rect = self.last().move_to(x, y);
        self.rectangles.push(new_rect);
    }

    fn offset(&mut self, dx: i32, dy: i32) {
        let new_rect = self.last().offset(dx, dy);
        self.rectangles.push(new_rect);
    }

    fn resize(&mut self, width: u32, height: u32) {
        let new_rect = self.last().resize(width, height);
        self.rectangles.push(new_rect);
    }

    fn update(&self) {
        println!("--- Оновлення: {} прямокутник(ів) ---", self.rectangles.len());
        for (i, r) in self.rectangles.iter().enumerate() {
            println!(
                "#{}: x={}, y={}, {}x{}",
                i + 1,
                r.x(),
                r.y(),
                r.width(),
                r.height()
            );
        }
        self.draw();
        println!();
    }


    fn draw(&self) {
        const W: usize = 50;
        const H: usize = 16;
        let mut canvas = vec![vec!['.'; W]; H];

        for (i, r) in self.rectangles.iter().enumerate() {
            let mark = std::char::from_digit(((i + 1) % 10) as u32, 10).unwrap();
            let x1 = r.x();
            let y1 = r.y();
            let x2 = r.x() + r.width() as i32 - 1;
            let y2 = r.y() + r.height() as i32 - 1;

            for yy in y1..=y2 {
                for xx in x1..=x2 {
                    let border = xx == x1 || xx == x2 || yy == y1 || yy == y2;
                    let inside = xx >= 0 && yy >= 0 && (xx as usize) < W && (yy as usize) < H;
                    if border && inside {
                        canvas[yy as usize][xx as usize] = mark;
                    }
                }
            }
        }

        for row in canvas {
            let line: String = row.into_iter().collect();
            println!("{}", line);
        }
    }
}

fn main() {
    let mut chain = Chain::new(Rectangle::new(2, 1, 10, 5));
    chain.update();

    chain.offset(8, 3);
    chain.update();

    chain.move_to(30, 2);
    chain.resize(14, 8);
    chain.update();

    let a = Rectangle::new(0, 0, 4, 4);
    let b = a.offset(5, 5);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}
use sfml::{
    graphics::PrimitiveType,
    graphics::{self, *},
    system::*,
    window::*,
};
struct Point<'s> {
    body: CircleShape<'s>,
    pos: Vector2f,
    color: graphics::Color,
}

impl<'s> Point<'s> {
    fn new(r: f32, c: graphics::Color) -> Point<'s> {
        Point {
            body: CircleShape::new(r, 30),
            pos: Vector2f::new(0., 0.),
            color: c,
        }
    }
    fn set_pos(&mut self, pos: Vector2f) {
        self.body.set_position(pos);
        self.pos = pos;
    }
    fn set_color(&mut self, c: graphics::Color) {
        self.body.set_fill_color(c);
    }
}

fn npos(i: f32, r: f32) -> Vector2f {
    Vector2f::new(i.to_radians().cos() * r, -i.to_radians().sin() * r)
}

fn fix_coord(x: f32, y: f32, window: &RenderWindow) -> Vector2f {
    Vector2f::new(
        x + (window.size().x as f32) / 2.,
        y + (window.size().y as f32) / 2.,
    )
}

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(1600, 900, VideoMode::desktop_mode().bits_per_pixel),
        "new",
        Style::CLOSE,
        &Default::default(),
    );
    let center: Vector2f = fix_coord(0., 0., &window);
    let mut k: Vec<Point> = Vec::new();
    let mut lines: Vec<Vec<Vertex>> = Vec::new();
    // let mut t = Time::default();
    let mut tc = Clock::start();
    // EQ
    let mut i = 0.;
    let r = 150.;
    loop { // Circle
        let mut p: Point = Point::new(0., graphics::Color::WHITE);
        p.set_pos(fix_coord(
            f32::from(i).to_radians().cos() * r,
            -f32::from(i).to_radians().sin() * r,
            &window,
        ));
        k.push(p);
        i += 0.01;
        if i >= 360. {
            break;
        }
    }
    let n = k.len();
    for i in 1..n {
        lines.push(vec![
            Vertex::new(
                Vector2f::new(k[i - 1].body.position().x, k[i - 1].body.position().y),
                Color::GREEN,
                Vector2f::new(0., 0.),
            ),
            Vertex::new(
                Vector2f::new(k[i].body.position().x, k[i].body.position().y),
                Color::RED,
                Vector2f::new(0., 0.),
            ),
        ]);
    }

    // AXIS
    let x_axis = vec![
        Vertex::new(Vector2f::new(0., 450.), Color::WHITE, Vector2f::new(0., 0.)),
        Vertex::new(
            Vector2f::new(1600., 450.),
            Color::WHITE,
            Vector2f::new(0., 0.),
        ),
    ];
    let y_axis = vec![
        Vertex::new(Vector2f::new(800., 0.), Color::WHITE, Vector2f::new(0., 0.)),
        Vertex::new(
            Vector2f::new(800., 900.),
            Color::WHITE,
            Vector2f::new(0., 0.),
        ),
    ];
    let rp = 5.; // Point Radio
    let mut p: Point = Point::new(rp, graphics::Color::RED);
    let mut l = 0.;
    let r1 = npos(l, 150.);
    let r2 = npos(l, 150.);
    p.set_pos(fix_coord(r1.x - rp, r2.y - rp, &window));
    let speed = 3.;
    // Update
    while window.is_open() {
        // P point
        let kl = npos(l, 150.);
        p.set_pos(fix_coord(kl.x - rp, kl.y - rp, &window));
        l += speed;
        tc.restart();

        // Linear Coefficient
        let m = -((p.body.position().x - center.x) / (p.body.position().y - center.y));
        let b = p.body.position().x * m - p.body.position().y;

        let mut p_line: Vec<Point> = Vec::new();

        let x = p.body.position().x as i32 - 100;
        let y = m * x as f32 - b;
        let mut pl1 = Point::new(5., graphics::Color::GREEN);
        pl1.set_pos(Vector2f::new(x as f32 + rp, y + rp));
        p_line.push(pl1);

        let x = p.body.position().x as i32 + 100;
        let y = m * x as f32 - b; // Line Equation
        let mut pl2 = Point::new(5., graphics::Color::GREEN);
        pl2.set_pos(Vector2f::new(x as f32 + rp, y + rp));
        p_line.push(pl2);

        let mut straight: Vec<Vec<Vertex>> = Vec::new();
        let n = p_line.len();
        for i in 1..n {
            straight.push(vec![
                Vertex::new(
                    Vector2f::new(p_line[i - 1].body.position().x, p_line[i - 1].body.position().y),
                    Color::YELLOW,
                    Vector2f::new(0., 0.),
                ),
                Vertex::new(
                    Vector2f::new(p_line[i].body.position().x, p_line[i].body.position().y),
                    Color::RED,
                    Vector2f::new(0., 0.),
                ),
            ]);
        }

        // y = xm - b;
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        // Drawing
        window.clear(Color::BLACK);
        window.draw_primitives(&x_axis, PrimitiveType::LINES, &RenderStates::default());
        window.draw_primitives(&y_axis, PrimitiveType::LINES, &RenderStates::default());

        for e in &k {
            window.draw(&e.body);
        }
        for e in &lines {
            window.draw_primitives(&e, PrimitiveType::LINES, &RenderStates::default());
        }
        for e in &straight {
            window.draw_primitives(&e, PrimitiveType::LINES, &RenderStates::default());
        }
        window.draw(&p.body);
        window.display();
    }
}

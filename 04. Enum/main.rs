#[allow(unused_variables)]
fn main() {
    let current_direction = Direction::North;
    let direction_events: Vec<Direction> = vec![
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::North
    ];

    for dir in &direction_events {
        println!("{}", describe_direction(dir))
    }

    let home_v4 = IpAddr::V4(192, 168, 1, 1);
    let loopback_v6 = IpAddr::V6(String::from("::1"));

    let event = WebEvent::KeyPress('c');

    if let WebEvent::KeyPress(c) = event {
        println!("Key pressed: {}", c);
    }

    let events = vec![
        WebEvent::PageLoad,
        WebEvent::PageUnload,
        WebEvent::KeyPress('a'),
        WebEvent::Click { x: 10, y: 20 },
        WebEvent::Scroll(15),
    ];

    for event in events {
        process_event(event);
    }
}
#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn describe_direction(dir: &Direction) -> &'static str {
    match dir {
        Direction::North => "Going up",
        Direction::South => "Going down",
        Direction::East => "Going right",
        Direction::West => "Going left",
    }
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i32, y: i32 },
    Scroll(i32),
}

fn process_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'", c),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
        WebEvent::Scroll(amount) => println!("Scrolled by {}", amount),
    }
}

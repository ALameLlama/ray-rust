use super::*;

// TODO: make these test actually test something,
// this is just so I can see if they're coming into ray
#[test]
fn test_ray_log() {
    ray!("Hello, Log!");
}

#[test]
fn test_ray_text() {
    ray!().text("Hello, Text!");
}

#[test]
fn test_ray_color() {
    ray!("Hello Color").color("green");
}

#[test]
fn test_ray_html() {
    ray!().html("<strong>Hello, HTML!</strong>");
}

#[test]
fn test_ray_clear_all() {
    ray!("Hello Clear");
    ray!().clear_all();
}

#[test]
fn test_ray_conffiti() {
    ray!().confetti();
}

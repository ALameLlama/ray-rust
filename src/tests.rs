use super::*;

// TODO: Add mocking? look into the request body and add tests to ensure the correct data is being sent
#[test]
fn test_ray_macro_no_args() {
    let ray = ray!();
    assert_eq!(ray.request.payloads.len(), 0);
}

#[test]
fn test_ray_macro_with_one_arg() {
    let ray = ray!("Hello, Ray Macro");
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_macro_with_multiple_args() {
    let ray = ray!("Hello", "Ray Macro");
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
#[should_panic(expected = "exited with code 1")]
fn test_rd_marco_no_args() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            if *payload == "exited with code 1" {
                std::process::exit(1);
            }
        }
    }));

    let ray = rd!();
    assert_eq!(ray.request.payloads.len(), 0);
}

#[test]
#[should_panic(expected = "exited with code 1")]
fn test_rd_macro_with_one_arg() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            if *payload == "exited with code 1" {
                std::process::exit(1);
            }
        }
    }));

    let ray = rd!("Hello, Rd Macro");
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
#[should_panic(expected = "exited with code 1")]
fn test_rd_macro_with_multiple_args() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            if *payload == "exited with code 1" {
                std::process::exit(1);
            }
        }
    }));

    let ray = rd!("Hello", "Rd Macro");
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_macro_with_struct() {
    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct TestStruct {
        name: String,
        age: i32,
    }

    let test_struct = TestStruct {
        name: "John".to_string(),
        age: 30,
    };

    let ray = ray!(test_struct);
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_log_function() {
    let mut ray = Ray::new();
    ray.log(vec!["Hello, Log!".to_string()]);
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_text_function() {
    let mut ray = Ray::new();
    ray.text("Hello, Text!");
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_color_function() {
    let mut ray = Ray::new();
    ray.text("Hello, Color").color("green");
    assert_eq!(ray.request.payloads.len(), 2);
}

#[test]
fn test_ray_html_function() {
    let mut ray = Ray::new();
    ray.html("<strong>Hello, HTML!</strong>");
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_clear_all_function() {
    let mut ray = Ray::new();
    ray.clear_all();
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_confetti_function() {
    let mut ray = Ray::new();
    ray.confetti();
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_charles_function() {
    let mut ray = Ray::new();
    ray.charles();
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_new_screen_function() {
    let mut ray = Ray::new();
    ray.new_screen(None);
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_new_screen_with_name_function() {
    let mut ray = Ray::new();
    ray.new_screen(Some("Hello, New Screen!"));
    assert_eq!(ray.request.payloads.len(), 1);
}

#[test]
fn test_ray_disable() {
    // TODO: Add a test to ensure the request is not sent
    let mut ray = Ray::new();
    ray.disable();
    assert_eq!(ray.disabled(), true);
    assert_eq!(ray.enabled(), false);
}

#[test]
fn test_ray_enabled() {
    // TODO: Add a test to ensure the request is sent
    let mut ray = Ray::new();
    ray.enable();
    assert_eq!(ray.enabled(), true);
    assert_eq!(ray.disabled(), false);
}

use cherrydoor_command::{Command, Heartbeat};

fn main() {
    let a = Command::new()
        .open_for(10)
        .display_text_for("__text__".to_string(), 10)
        .set_color_for(255, 255, 255, 10)
        .play_sound(1)
    .into_string().unwrap();

    let b = Command::new()
        .open()
    .into_string().unwrap();

    println!("{}", a);
    println!("{}", b);

    let c = String::from("0;0;\n");
    let d = String::from("04214556;0;\n");
    let e = String::from("0231;0;\n");

    println!("{:?}", Heartbeat::from_heartbeat(c));
    println!("{:?}", Heartbeat::from_heartbeat(d));
    println!("{:?}", Heartbeat::from_heartbeat(e));
}
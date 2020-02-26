pub mod platform;

fn main() {
    let window_desc = platform::WindowDesc::new(String::from("Pindae"));
    let instance_desc = platform::InstanceDesc::new();

    let mut instance = platform::Instance::new(instance_desc, window_desc).expect("An error occurred while creating platform::Instance");

    loop {
        let e = instance.poll_events();

        for x in &e {
            match x {
                platform::Events::OnUICreate => println!("OnUICreate"),
                _ => println!("Unsupported event"),
            }
        }
    }
}

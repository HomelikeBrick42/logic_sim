use logic_sim::*;

fn main() {
    let mut scene = Scene::new();
    let not = scene.add_component(Component::Or {
        inputs: [Input { state: false }, Input { state: false }],
        output: None,
    });
    let delay = scene.add_component(Component::Delay {
        input: Input { state: true },
        output: Some(Output {
            component: not,
            index: 1,
        }),
        state_last_frame: false,
    });
    println!("Before Update:");
    println!("{scene}");
    println!();
    dbg!(scene.update());
    println!("After Update 1:");
    println!("{scene}");
    println!();
    dbg!(scene.update());
    println!("After Update 2:");
    println!("{scene}");
    println!();
    dbg!(scene.update());
    println!("After Update 3:");
    println!("{scene}");
    println!();
    if let Component::Delay {
        input,
        output: _,
        state_last_frame: _,
    } = scene.get_component_mut(delay)
    {
        input.state = false;
    } else {
        unreachable!()
    }
    dbg!(scene.update());
    println!("After Update 4:");
    println!("{scene}");
    println!();
    dbg!(scene.update());
    println!("After Update 5:");
    println!("{scene}");
    println!();
    dbg!(scene.update());
    println!("After Update 6:");
    println!("{scene}");
}

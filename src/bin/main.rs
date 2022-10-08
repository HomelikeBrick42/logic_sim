use logic_sim::*;

fn main() {
    let mut scene = Scene::new();
    let not2 = scene.add_component(Component::Not {
        input: Input { state: false },
        output: None,
    });
    let _not1 = scene.add_component(Component::Not {
        input: Input { state: false },
        output: Some(Output {
            component: not2,
            index: 0,
        }),
    });
    println!("Before Update:");
    println!("{scene}");
    println!();
    scene.update();
    println!("After Update:");
    println!("{scene}");
}

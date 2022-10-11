use lerp::Lerp;
use raylib::prelude::*;

use logic_sim::*;

enum Selected {
    Nothing,
    Movement {
        component: ComponentID,
        offset: Vector2,
    },
    OutputConnection {
        component: ComponentID,
        index: usize,
    },
}

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as _, HEIGHT as _)
        .title("Logic Simulator")
        .vsync()
        .build();

    let mut camera = Camera2D {
        offset: raylib::math::Vector2 {
            x: WIDTH as f32 / 2.0,
            y: HEIGHT as f32 / 2.0,
        },
        target: raylib::math::Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut scene = Scene::new();
    scene.add_component(Component {
        typ: ComponentType::Not {
            input: Input { state: false },
            output: None,
        },
        position: Vector2 { x: 100.0, y: 100.0 },
    });
    scene.add_component(Component {
        typ: ComponentType::Or {
            inputs: [Input { state: false }, Input { state: false }],
            output: None,
        },
        position: Vector2 { x: 100.0, y: -50.0 },
    });
    scene.add_component(Component {
        typ: ComponentType::Delay {
            input: Input { state: false },
            output: None,
            state_last_frame: false,
        },
        position: Vector2 { x: -100.0, y: 0.0 },
    });

    let mut selected = Selected::Nothing;

    const UPDATE_INTERVAL: f32 = 1.0 / 5.0;
    let mut update_time = 0.0;
    while !rl.window_should_close() {
        let ts = rl.get_frame_time();

        update_time += ts;
        while update_time >= UPDATE_INTERVAL {
            scene.update();
            update_time -= UPDATE_INTERVAL;
        }

        // Movement
        {
            let mut move_direction = Vector2 { x: 0.0, y: 0.0 };
            if rl.is_key_down(KeyboardKey::KEY_W) {
                move_direction.y -= 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_S) {
                move_direction.y += 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_A) {
                move_direction.x -= 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_D) {
                move_direction.x += 1.0;
            }
            camera.target += move_direction.normalized() * 300.0 * ts;
        }

        let mouse_world_pos = rl.get_screen_to_world2D(
            Vector2 {
                x: rl.get_mouse_x() as _,
                y: rl.get_mouse_y() as _,
            },
            camera,
        );
        selected = match selected {
            Selected::Nothing => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    (&scene)
                        .into_iter()
                        .rev()
                        .find_map(|id| {
                            let component = scene.get_component(id);
                            let size = get_component_size(component);
                            let position = component.position - (size * 0.5);
                            if (Rectangle {
                                x: position.x,
                                y: position.y,
                                width: size.x,
                                height: size.y,
                            })
                            .check_collision_point_rec(mouse_world_pos)
                            {
                                Some(
                                    get_output_circles(component)
                                        .into_iter()
                                        .enumerate()
                                        .find_map(|(i, (position, radius))| {
                                            if check_collision_point_circle(
                                                mouse_world_pos,
                                                position,
                                                radius,
                                            ) {
                                                Some(Selected::OutputConnection {
                                                    component: id,
                                                    index: i,
                                                })
                                            } else {
                                                None
                                            }
                                        })
                                        .unwrap_or(Selected::Movement {
                                            component: id,
                                            offset: component.position - mouse_world_pos,
                                        }),
                                )
                            } else {
                                None
                            }
                        })
                        .unwrap_or(Selected::Nothing)
                } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
                    for id in &scene {
                        let component = scene.get_component(id);
                        if let Some((id, index)) = get_output_circles(component)
                            .into_iter()
                            .enumerate()
                            .find_map(|(i, (position, radius))| {
                                if check_collision_point_circle(mouse_world_pos, position, radius) {
                                    Some((id, i))
                                } else {
                                    None
                                }
                            })
                        {
                            if let Some(output) =
                                scene.get_component_mut(id).get_outputs_mut()[index]
                            {
                                scene.get_component_mut(output.component).get_inputs_mut()
                                    [output.index]
                                    .state = false;
                            }
                            scene.get_component_mut(id).get_outputs_mut()[index] = None;
                            break;
                        }
                    }
                    Selected::Nothing
                } else {
                    Selected::Nothing
                }
            }
            Selected::Movement {
                component: id,
                offset,
            } => {
                if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
                    Selected::Nothing
                } else {
                    scene.get_component_mut(id).position = rl.get_screen_to_world2D(
                        Vector2 {
                            x: rl.get_mouse_x() as _,
                            y: rl.get_mouse_y() as _,
                        } + offset,
                        camera,
                    );
                    Selected::Movement {
                        component: id,
                        offset,
                    }
                }
            }
            Selected::OutputConnection {
                component: id,
                index,
            } => {
                if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
                    if let Some((input_id, input_index)) = (&scene).into_iter().find_map(|id| {
                        if let Some(index) = get_input_circles(scene.get_component(id))
                            .into_iter()
                            .enumerate()
                            .find_map(|(index, (position, radius))| {
                                if check_collision_point_circle(mouse_world_pos, position, radius) {
                                    Some(index)
                                } else {
                                    None
                                }
                            })
                        {
                            Some((id, index))
                        } else {
                            None
                        }
                    }) {
                        if let Some(output) = scene.get_component_mut(id).get_outputs_mut()[index] {
                            scene.get_component_mut(output.component).get_inputs_mut()
                                [output.index]
                                .state = false;
                        }
                        scene.get_component_mut(id).get_outputs_mut()[index] = Some(Output {
                            component: input_id,
                            index: input_index,
                        });
                        if scene.has_cyclic_dependency() {
                            scene.get_component_mut(id).get_outputs_mut()[index] = None;
                        } else {
                            for other_id in &scene {
                                if other_id != id {
                                    let component = scene.get_component(other_id);
                                    for output in component.get_outputs() {
                                        if let Some(output) = *output {
                                            if output.component == input_id
                                                && output.index == input_index
                                            {
                                                if let Some(output) = scene
                                                    .get_component_mut(other_id)
                                                    .get_outputs_mut()[input_index]
                                                {
                                                    scene
                                                        .get_component_mut(output.component)
                                                        .get_inputs_mut()[output.index]
                                                        .state = false;
                                                }
                                                scene
                                                    .get_component_mut(other_id)
                                                    .get_outputs_mut()[input_index] = None;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Selected::Nothing
                } else {
                    Selected::OutputConnection {
                        component: id,
                        index,
                    }
                }
            }
        };

        // Drawing
        {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::DARKGRAY);

            // Scene
            {
                let mut d = d.begin_mode2D(camera);
                for id in &scene {
                    let component = scene.get_component(id);

                    let size = get_component_size(component);
                    d.draw_rectangle_v(component.position - (size * 0.5), size, Color::WHITE);

                    for (i, (position, radius)) in
                        get_input_circles(component).into_iter().enumerate()
                    {
                        d.draw_circle_v(
                            position,
                            radius,
                            if component.get_inputs()[i].state {
                                Color::GREEN
                            } else {
                                Color::RED
                            },
                        );
                    }
                    for (i, (position, radius)) in
                        get_output_circles(component).into_iter().enumerate()
                    {
                        d.draw_circle_v(
                            position,
                            radius,
                            if let Some(output) = component.get_outputs()[i] {
                                if scene.get_component(output.component).get_inputs()[output.index]
                                    .state
                                {
                                    Color::GREEN
                                } else {
                                    Color::RED
                                }
                            } else {
                                Color::GRAY
                            },
                        );
                    }
                }
                for id in &scene {
                    let component = scene.get_component(id);
                    for (i, output) in component
                        .get_outputs()
                        .iter()
                        .enumerate()
                        .filter_map(|(i, output)| output.map(|output| (i, output)))
                    {
                        let connected_component = scene.get_component(output.component);
                        let (start, _) = get_output_circles(component)[i];
                        let (end, _) = get_input_circles(connected_component)[output.index];

                        fn cubic_bezier(p: [Vector2; 4], t: f32) -> Vector2 {
                            let q = [p[0].lerp(p[1], t), p[1].lerp(p[2], t), p[2].lerp(p[3], t)];
                            let r = [q[0].lerp(q[1], t), q[1].lerp(q[2], t)];
                            let b = r[0].lerp(r[1], t);
                            b
                        }

                        fn draw_cubic_bezier(
                            d: &mut RaylibMode2D<RaylibDrawHandle>,
                            points: [Vector2; 4],
                            num_segments: usize,
                            color: Color,
                        ) {
                            for i in 0..num_segments {
                                d.draw_line_ex(
                                    cubic_bezier(points, i as f32 / num_segments as f32),
                                    cubic_bezier(points, (i + 1) as f32 / num_segments as f32),
                                    2.0,
                                    color,
                                );
                            }
                        }

                        draw_cubic_bezier(
                            &mut d,
                            [
                                start,
                                start + Vector2 { x: 100.0, y: 0.0 },
                                end - Vector2 { x: 100.0, y: 0.0 },
                                end,
                            ],
                            100,
                            if connected_component.get_inputs()[output.index].state {
                                Color::GREEN
                            } else {
                                Color::RED
                            },
                        );
                    }
                }
                for id in &scene {
                    let component = scene.get_component(id);

                    let name = component.get_name();
                    let font_size = 20;
                    let width = measure_text(name, font_size);

                    d.draw_text(
                        name,
                        component.position.x as i32 - width / 2,
                        component.position.y as i32 - font_size / 2,
                        font_size,
                        Color::DARKGRAY,
                    );
                }
            }

            d.draw_text(&format!("FPS: {}", 1.0 / ts), 12, 12, 20, Color::WHITE);
        }
    }
}

fn get_component_size(component: &Component) -> Vector2 {
    Vector2 {
        x: 150.0,
        y: (component
            .get_inputs()
            .len()
            .max(component.get_outputs().len())
            * 50) as f32,
    }
}

fn get_input_circles(component: &Component) -> Vec<(Vector2, f32)> {
    let size = get_component_size(component);
    let inputs = component.get_inputs();
    if inputs.len() % 2 == 0 {
        assert_eq!(inputs.len(), 2); // TODO: make this better
        [
            (
                component.position
                    - Vector2 {
                        x: size.x / 3.0,
                        y: -25.0,
                    },
                7.5,
            ),
            (
                component.position
                    - Vector2 {
                        x: size.x / 3.0,
                        y: 25.0,
                    },
                7.5,
            ),
        ]
        .into()
    } else {
        assert_eq!(inputs.len(), 1); // TODO: make this better
        [(
            component.position
                - Vector2 {
                    x: size.x / 3.0,
                    y: 0.0,
                },
            7.5,
        )]
        .into()
    }
}

fn get_output_circles(component: &Component) -> Vec<(Vector2, f32)> {
    let size = get_component_size(component);
    let outputs = component.get_outputs();
    if outputs.len() % 2 == 0 {
        assert_eq!(outputs.len(), 2); // TODO: make this better
        [
            (
                component.position
                    - Vector2 {
                        x: -size.x / 3.0,
                        y: -25.0,
                    },
                7.5,
            ),
            (
                component.position
                    - Vector2 {
                        x: -size.x / 3.0,
                        y: 25.0,
                    },
                7.5,
            ),
        ]
        .into()
    } else {
        assert_eq!(outputs.len(), 1); // TODO: make this better
        [(
            component.position
                - Vector2 {
                    x: -size.x / 3.0,
                    y: 0.0,
                },
            7.5,
        )]
        .into()
    }
}

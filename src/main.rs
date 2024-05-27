use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use rand::Rng;

const INVERT_Y: Vec2 = Vec2 {x: 1., y: -1.};
const CAT_COUNT: u32 = 9;
const CAT_REPEL_DIST: f32 = 75.0;
const CAT_ATTRACT_DIST: f32 = 100.0;
const CHASE_SPEED: f32 = 2.0;
const CHASE_ENERGY: f32 = 50.0;
const CHASE_DISTANCE: f32 = 200.0;
const TURN_AWAY: f32 = 0.1;
const EDGE_REPEL: f32 = 100.0;
const DOES_CAT_PLAY: f32 = 0.05;
const MAX_SPEED: f32 = 10.0;
const WAKE_CAT: f32 = 0.0002;
const TARGET_SEPERATION: f32 = 200.0;
const MIN_CAT_WAKEUP: u32 = 300;
const CAT_DIRECTION_CHANGE: f32 = 0.03;

#[derive(Component)]
struct Character {
    name: String,
    position: Vec2,
    size: f32,
}

#[derive(Component)]
struct Behavior {
    energy: f32,
    direction: f32,
    speed: f32,
}

#[derive(Component)]
struct Environment {
    screen_size: f32,
    mouse_pos: Vec2,
    center_cat_pos: Vec2,
    max_cat_dist: f32,
    score: u32,
    possible_score: u32,
    last_cat_wakeup: u32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct DistanceText;


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, (With<PrimaryWindow>, Without<Character>)>,
) {
    let mut rng = rand::thread_rng();
    let mid_point = Vec2{x: window.single().resolution.width() / 2., y: window.single().resolution.height() / 2.};
    let start_pos = Vec2 {x: 200., y: 200.};

    commands.spawn(Camera2dBundle::default());

    let environment = Environment {
        screen_size: (window.single().resolution.width() / 2.).min(window.single().resolution.height() / 2.),
        mouse_pos: start_pos.clone(),
        center_cat_pos: Vec2::ZERO,
        max_cat_dist: 0.0,
        score: 0,
        possible_score: 0,
        last_cat_wakeup: 0,
    };
    commands.spawn(environment);

    let me = Character { 
        name: "me".to_string(),
        position: start_pos,
        size: 30.0,
    };
    let me_bundle = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: me.size })),
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(me.position.x, me.position.y, 1.),
        ..default()
    };

    commands.spawn((me, me_bundle));

    for i in 0..CAT_COUNT {
        let cat = Character {
            name: format!("cat_{}", i),
            position: Vec2 {
                x: rng.gen_range(-mid_point.x / 4.0 .. mid_point.x / 4.0),
                y: rng.gen_range(-mid_point.y / 4.0 .. mid_point.y / 4.0)
            },
            size: 10.0,
        };

        let behavior = Behavior {
            energy: 0.0,
            direction: rng.gen_range(0.0..(2.0 * std::f32::consts::PI)),
            speed: 0.0,
        };

        let cat_bundle = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: cat.size })),
            material: materials.add(Color::hsl(0.1, 0.95, 0.7)),
            transform: Transform::from_xyz(cat.position.x, cat.position.y, 0.),
            ..default()
        };

        commands.spawn((cat, behavior, cat_bundle));
    }

    let font = TextStyle {
        font_size: 20.0,
        color: Color::GOLD,
        ..default()
    };

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                font.clone(),
            ),
            TextSection::new(
                "0",
                font.clone(),
            ),
            TextSection::new(
                "    Possible: ",
                font.clone(),
            ),
            TextSection::new(
                "0",
                font.clone(),
            ),
            TextSection::new(
                "         (0.000%)",
                font.clone(),
            ),
        ]),
        ScoreText,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Distance: ",
                font.clone(),
            ),
            TextSection::new(
                "0",
                font.clone(),
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(0.0),
            ..default()
        }),
        DistanceText,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Target:   ",
                font.clone(),
            ),
            TextSection::new(
                TARGET_SEPERATION.to_string(),
                font.clone(),
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(0.0),
            ..default()
        }),
    ));
}

fn update_me( 
    mut characters: Query<&mut Character, With<Character>>,
    window: Query<&Window, (With<PrimaryWindow>, Without<Character>)>,
    mut env_query: Query<&mut Environment>,
) {
    for mut character in &mut characters {
        if character.name == "me" {
            let mut environment = env_query.single_mut();
            let mouse_pos;
            if let Some(get_mouse_pos) = window.single().cursor_position() {
                mouse_pos = get_mouse_pos;
                environment.mouse_pos = mouse_pos;
            } else {
                mouse_pos = environment.mouse_pos;
            }

            let mid_point = Vec2{x: window.single().resolution.width() / 2., y: window.single().resolution.height() / 2.};
            let mid_point_to_mouse = mouse_pos - mid_point;
            let mouse_to_char = (mid_point_to_mouse * INVERT_Y) - character.position;
            let to_move = 10. * mouse_to_char / mid_point;

            character.position += to_move;
        } else {
            // Cat
        }
    }
}

fn stir_cats(
    mut cats: Query<(&mut Character, Option<&mut Behavior>)>,
    mut env_query: Query<&mut Environment>,
) {
    let mut rng = rand::thread_rng();
    let mut player_pos: Vec2 = Vec2::ZERO;
    let screen_size = env_query.single().screen_size;
    let mut environment = env_query.single_mut();

    environment.max_cat_dist = 0.0;

    for (character, _behavior) in &cats {  
        if character.name == "me" {
            player_pos = character.position.clone();
            break;
        }
    }

    let mut combinations = cats.iter_combinations_mut();
    while let Some([cat, other_cat]) = combinations.fetch_next() {
        let (cat_character, cat_behavior) = cat;
        let (other_cat_character, other_cat_behavior) = other_cat;
        
        if cat_character.name == "me" || other_cat_character.name == "me" {
            continue;
        }
     
        if let Some(mut cat_b) = cat_behavior {
            if let Some(mut other_b) = other_cat_behavior {
                let cat_dist: f32 = cat_character.position.distance(other_cat_character.position);
                if cat_dist > environment.max_cat_dist {
                    environment.max_cat_dist = cat_dist;
                    environment.center_cat_pos = (cat_character.position + other_cat_character.position) / 2.0;
                }
                if cat_dist > CAT_REPEL_DIST && cat_dist < CAT_ATTRACT_DIST && rng.gen::<f32>() <= DOES_CAT_PLAY {
                    // Sometimes cats play with each other.
                    //info!("Interaction {} {}", cat_character.name, other_cat_character.name, );
                    let ave_direction = (cat_b.direction + other_b.direction) / 2.0;
                    cat_b.direction = ave_direction;
                    other_b.direction = ave_direction;
                    cat_b.energy = cat_b.energy.max(other_b.energy);
                    other_b.energy = other_b.energy.max(cat_b.energy);
                    cat_b.speed = cat_b.speed.max(other_b.speed);
                    other_b.speed = other_b.speed.max(cat_b.speed);
                }
            }
        }
    }

    let mut wake_this_cat = "".to_string();
    if environment.last_cat_wakeup > MIN_CAT_WAKEUP {
        wake_this_cat = format!("cat_{}", rng.gen_range(0..CAT_COUNT));
        info!("Cats are too sleepy. Wake {}.", wake_this_cat);
    }

    for (character, behavior) in &mut cats {  
        if character.name == "me" {
            continue;
        }

        if let Some(mut b) = behavior {
            // Cat doesn't like being too close to player.
            // Move directly away.
            let player_cat_dist = character.position.distance(player_pos);
            if player_cat_dist < CHASE_DISTANCE {
                let cat_vector = (character.position - player_pos).normalize_or_zero();
                b.direction = cat_vector.to_angle();
                b.energy = CHASE_ENERGY;
                b.speed = CHASE_SPEED;
            }

            // Wake sleepy cat.
            if rng.gen::<f32>() <= WAKE_CAT || wake_this_cat == character.name {
                b.energy = CHASE_ENERGY;
                info!("Wake cat: {}", character.name);
                environment.last_cat_wakeup = 0;
            }

            b.direction += (rng.gen::<f32>() - 0.5) * CAT_DIRECTION_CHANGE;

            avoid_edges(&character, &mut b, screen_size);
            clamp_direction(&mut b.direction);
        }
    }

    environment.last_cat_wakeup += 1;
}

fn clamp_direction(direction: &mut f32) {
    while *direction <= -std::f32::consts::PI {
        *direction += std::f32::consts::PI * 2.0;
    }
    while *direction > std::f32::consts::PI {
        *direction -= std::f32::consts::PI * 2.0;
    }
}

fn avoid_edges(cat_character: &Character, behavior: &mut Behavior, screen_size: f32) {
    /* Cats don't like being at the edge of the screen or outside it.
     * Turn them back towards the center if they are too close to an edge. */
    let distance_to_center = cat_character.position.length();
    let angle_to_center = cat_character.position.to_angle();
    
    if distance_to_center.abs() < screen_size - EDGE_REPEL {
        return;
    }

    let angle_diff = angle_to_center - behavior.direction;
    if angle_diff.abs() < std::f32::consts::PI / 1.8 {
        if angle_diff > 0. {
            behavior.direction -= TURN_AWAY;
        } else {
            behavior.direction += TURN_AWAY;
        }
    } else if angle_diff.abs() > 3.0 * std::f32::consts::PI / 2.2 {
        if angle_diff > 0. {
            behavior.direction += TURN_AWAY;
        } else {
            behavior.direction -= TURN_AWAY;
        }
    }

    // Make sure cats are moving if they are possible off the screen to make sure they come back eventually.
    if distance_to_center.abs() >= screen_size {
        behavior.energy = CHASE_ENERGY;
        behavior.speed = CHASE_SPEED;
    }
}

fn update_positions(
    mut characters: Query<(&mut Character, Option<&mut Behavior>, &mut Transform)>,
) {
    let mut rng = rand::thread_rng();

    for (mut character, behavior, mut translation) in &mut characters {
        if let Some(mut b) = behavior {
            if b.energy > 1.0 {
                b.energy -= 1.0;
                if b.energy > 0.0 && rng.gen::<f32>() > 0.001 {
                    b.speed += 0.1;
                }
            } else {
                b.energy = 0.0;
                if b.speed > 0.2 {
                    b.speed -= 0.1;
                } else {
                    b.speed = 0.0;
                }
            }
            character.position += Vec2::from_angle(b.direction) * b.speed;

            b.speed = b.speed.min(MAX_SPEED);
        }
        translation.translation = character.position.extend(0.);
    }
}

fn draw_overlay(
    mut gizmos: Gizmos,
    mut env_query: Query<&mut Environment>,
    mut text_query: Query<(&mut Text, Option<&ScoreText>, Option<&DistanceText>)>,
) {
    let mut environment = env_query.single_mut();

    environment.possible_score += 1;

    if environment.max_cat_dist < TARGET_SEPERATION {
        gizmos.circle_2d(environment.center_cat_pos, environment.max_cat_dist / 2.0, Color::GREEN);
        environment.score += 1;
    }

    for (mut text, score, dist) in &mut text_query {
        if let Some(_s) = score {
            text.sections[1].value = environment.score.to_string();
            text.sections[3].value = environment.possible_score.to_string();
            let f_score = environment.score as f32;
            let f_possible = environment.possible_score as f32;
            let percentage = 100.0 * f_score / f_possible;
            text.sections[4].value = format!("  ({percentage:.3}%)");
        } else if let Some(_d) = dist {
            let val = environment.max_cat_dist;
            text.sections[1].value = format!("{val:.1}");
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, ((update_me, stir_cats), update_positions, draw_overlay).chain())
        .run();
}

use bevy::prelude::*;


#[derive(Component)]
struct Position {x : f32, y: f32}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, tartup)
        .add_systems(Update, print_position_system)
        .add_systems(Startup, greetings)
        .run();
    
}

fn tartup(mut commands: Commands) {
    println!("Hello World");
    println!("woo");
   
    commands.spawn((Person, Name("Person 1".to_string())));

    commands.spawn((Person, Name("Furry 1".to_string())));
}


fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("Position: {} {}", position.x, position.y)
    }
}


fn greetings (query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello there {}!", name.0)

    }
}
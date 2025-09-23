use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world,(update_people, greet_people).chain()))
        .run();
}

fn hello_world(){
    println!("Hello, world!");
}


fn add_people(mut commands: Commands){
    commands.spawn((Person,Name(String::from("Juanito"))));
    commands.spawn((Person,Name(String::from("Pedrito"))));
    commands.spawn((Person,Name(String::from("Sutanito"))));
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query{
        println!("Hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>){
    for mut name in &mut query{
        if name.0 == "Juanito" {
            name.0 = String::from("Perenganito");
            break;
        }
    }
}
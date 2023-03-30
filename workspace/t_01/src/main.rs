use bevy::prelude::{App, Commands, Component, Plugin, Query, Without};

fn main() {
    App::new().add_plugin(PersonPlugin).run();
}

pub struct PersonPlugin;
impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_name)
            .add_system(print_name_job)
            .add_system(print_name_no_job);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));

    commands.spawn((
        Person {
            name: "Bob".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));

    commands.spawn((Person {
        name: "Dim".to_string(),
    },));
}

fn print_name(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("name: {}", person.name);
    }
}

fn print_name_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        println!("name: {}, job is : {:?}", person.name, employed.job);
    }
}

fn print_name_no_job(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("name: {} no job", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}

#[derive(Component)]
struct PlayerXp(u32);

#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Friendly;

#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    _p: Player,

    #[bundle]
    sprite: SpriteSheetBundle,
}

#[derive(Resource)]
struct GoalsReached {
    main_goal: bool,
    bonus: bool,
}

#[derive(Resource, Default, Debug)]
struct StartingLevel(usize);

fn debug_start(
    // access resource
    start: Res<StartingLevel>,
) {
    eprintln!("Starting on level {:?}", *start);
}

use rusty_engine::prelude::*;
use rusty_engine::prelude::bevy::utils::HashMap;
use rand::prelude::*;

const PLAYER_LABEL: &str = "player";
const HIGH_SCORE_LABEL: &str = "high_score";
const SCORE_LABEL: &str = "score";

struct GameState {
    high_score: u32,
    current_score: u32,
    spawn_index: u32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            high_score: 0,
            current_score: 0,
            spawn_index: 0,
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.window_settings(WindowDescriptor {
        resizable: false,
        title: String::from("Seems playable :)"),
        ..Default::default()
    });

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.3);

    let player = game.add_sprite(PLAYER_LABEL, SpritePreset::RacingCarRed);
    player.translation = Vec2::new(200.0, -100.0);
    player.rotation = LEFT;
    player.collision = true;
    player.layer = 10.0;

    let score = game.add_text(SCORE_LABEL, "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);
    score.layer = 90.0;

    let high_score = game.add_text(HIGH_SCORE_LABEL, "High score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);
    high_score.layer = 90.0;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // handle collisions
    for event in engine.collision_events.drain(..) {
        println!("{:?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with(PLAYER_LABEL) {
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.4);

            let mut texts = &mut engine.texts;
            game_state.current_score += 1;
            update_score(game_state, &mut texts);

            if game_state.current_score > game_state.high_score {
                game_state.high_score = game_state.current_score;
                update_high_score(game_state, &mut texts);
            }

            for label in event.pair {
                if label != PLAYER_LABEL {
                    engine.sprites.remove(&label);
                }
            }
        }
    }

    // handle movement
    let player = engine.sprites.get_mut(PLAYER_LABEL).unwrap();
    const MOVEMENT_SPEED: f32 = 150.0;
    let keyboard_state = &engine.keyboard_state;
    if keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if keyboard_state.pressed_any(&[KeyCode::Left, KeyCode::A]) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if keyboard_state.pressed_any(&[KeyCode::Right, KeyCode::D]) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // reset score
    if keyboard_state.pressed(KeyCode::R) {
        game_state.current_score = 0;
        update_score(game_state, &mut &mut engine.texts);
    }

    // handle spawn
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ball{}", game_state.spawn_index);
        game_state.spawn_index += 1;
        let ball = engine.add_sprite(label, SpritePreset::RollingBallBlue);
        ball.translation.x = thread_rng().gen_range(-550.0..550.0);
        ball.translation.y = thread_rng().gen_range(-325.0..325.0);
        ball.collision = true;
        ball.layer = 1.0;
    }
}

fn update_score(game_state: &GameState, texts: &mut &mut HashMap<String, Text>) {
    println!("Score: {}", game_state.current_score);
    texts.get_mut(SCORE_LABEL).unwrap().value = format!("Score: {}", game_state.current_score)
}

fn update_high_score(game_state: &GameState, texts: &mut &mut HashMap<String, Text>) {
    println!("High score: {}", game_state.current_score);
    texts.get_mut(HIGH_SCORE_LABEL).unwrap().value = format!("High score: {}", game_state.current_score)
}

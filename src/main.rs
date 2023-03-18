use bevy::prelude::*;
use bevy::window::WindowResolution;

const HEIGHT:f32 = 600.0;
const WIDTH:f32 = 800.0;

const PADDLE_WIDTH:f32 = 10.0;
const PADDLE_HEIGHT:f32 = 100.0;
const PADDLE_VELOCITY:f32 = 180.0;
const PADDLE_MAX_MOVE:f32 = HEIGHT/2.0-PADDLE_HEIGHT/2.0;
const PADDLE_MIN_MOVE:f32 = -HEIGHT/2.0+PADDLE_HEIGHT/2.0;

const BALL_WIDTH:f32 = 10.0;
const BALL_VELOCITY: Vec3 = Vec3::new(160.0,160.0,0.0);

const LINE_WIDTH:f32 = 2.0;

#[derive(Resource)]
struct Score{
    left:u32,
    right:u32
}

#[derive(Component)]
struct Ball{}

#[derive(Component)]
struct Paddle{
    up_key: KeyCode,
    down_key: KeyCode
}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct LeftText;

#[derive(Component)]
struct RightText;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Score{
            left:0,
            right:0
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy ping".to_string(),
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(input)
        .add_system(move_ball)
        .add_system(collides)
        .add_system(score)
        .add_system(show_score)
        .run();
}

fn setup(
    mut commands:Commands,
    asset_server: Res<AssetServer>,
) {
    //camera
    commands
        .spawn(Camera2dBundle::default());

    //ball
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_WIDTH, BALL_WIDTH)),
            ..default()
        },
        ..default()
    })
        .insert(Velocity(BALL_VELOCITY.clone()))
        .insert(Ball{});

    //line
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(LINE_WIDTH, HEIGHT)),
            ..default()
        },
        ..default()
    });

    //paddle_left
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        transform: Transform::from_xyz(-WIDTH/2.0+PADDLE_WIDTH/2.0,0.0,0.0),
        ..default()
    })
        .insert(Paddle{up_key:KeyCode::W, down_key:KeyCode::A});

    //paddle_right
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        transform: Transform::from_xyz(WIDTH/2.0-PADDLE_WIDTH/2.0,0.0,0.0),
        ..default()
    })
        .insert(Paddle{up_key:KeyCode::Up, down_key:KeyCode::Down});

    //score
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 64.0,
                color: Color::WHITE,
            }),
        transform: Transform::from_xyz(-WIDTH/4.0,200.,1.),
        ..Default::default()
    })
        .insert(LeftText{});
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 64.0,
                color: Color::WHITE,
            }),
        transform: Transform::from_xyz(WIDTH/4.0,200.,1.),
        ..Default::default()
    })
        .insert(RightText{});
}

fn input(
    time:Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>
){
    for (paddle,mut transform) in query.iter_mut(){
        let paddle_move = time.delta_seconds()*PADDLE_VELOCITY;
        if keyboard_input.pressed(paddle.up_key){
            transform.translation.y = PADDLE_MAX_MOVE.min(transform.translation.y+paddle_move);
        };
        if keyboard_input.pressed(paddle.down_key) {
            transform.translation.y = PADDLE_MIN_MOVE.max(transform.translation.y - paddle_move);
        }
    }
}

fn move_ball(
    time:Res<Time>,
    mut query: Query<(&mut Transform,&mut Velocity),With<Ball>>
){
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();

        if transform.translation.y - BALL_WIDTH/2.0 < -HEIGHT/2.0 ||
           transform.translation.y + BALL_WIDTH/2.0 > HEIGHT/2.0 {
            velocity.0.y = - velocity.0.y;
        }
    }
}

fn collides(
    mut ball_query: Query<(&Transform, &mut Velocity, &Sprite),(With<Ball>, Without<Paddle>)>,
    paddle_query:Query<(&Transform, &Sprite),(With<Paddle>, Without<Ball>)>
){
    for (ball_transform, mut velocity,ball) in ball_query.iter_mut() {
        let ball_size = ball.custom_size.unwrap();
        for (paddle_transform, paddle) in paddle_query.iter() {
            let paddle_size = paddle.custom_size.unwrap();
            match bevy::sprite::collide_aabb::collide(
                ball_transform.translation,
                 ball_size,
                paddle_transform.translation,
                paddle_size,
            ){
                Some(_collision)=>{
                    velocity.0.x = - velocity.0.x;
                },
                None=>{}
            }
        }
    }
}

fn score(
    mut score:ResMut<Score>,
    mut query: Query<&mut Transform,With<Ball>>,
)
{
    for mut transform in query.iter_mut() {
        if transform.translation.x < -WIDTH / 2.0 {
            score.right += 1;
            transform.translation = Vec3::ZERO;
        }
        if transform.translation.x > WIDTH / 2.0 {
            score.left += 1;
            transform.translation = Vec3::ZERO;
        }
    }
}

fn show_score(
    score:Res<Score>,
    mut left_query: Query<&mut Text,(With<LeftText>,Without<RightText>)>,
    mut right_query: Query<&mut Text,(With<RightText>,Without<LeftText>)>,
){
    for mut text in left_query.iter_mut() {
        text.sections[0].value = format!("{}", score.left);
    }
    for mut text in right_query.iter_mut() {
        text.sections[0].value = format!("{}", score.right);
    }
}

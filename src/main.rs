use glam::{vec3, Vec3};
mod engine;
use engine::raytrace::Transform;
use std::io::{stdout, Result};
use std::time::Instant;

const FPS: f32 = 16.0;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

fn get_input(t: Transform, sec_to_imput: u64) -> Option<Transform> {
    let mut retun_t = t;
    if event::poll(std::time::Duration::from_millis(sec_to_imput)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return None;
            }
            // movment
            let mut moment_vec = vec3(0.0, 0.0, 0.0);
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Left {
                moment_vec += vec3(-1.0, 0.0, 0.0);
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Right {
                moment_vec += vec3(1.0, 0.0, 0.0);
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Up {
                moment_vec += vec3(0.0, 0.0, -1.0);
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
                moment_vec += vec3(0.0, 0.0, 1.0);
            }
            //moment_vec = moment_vec.normalize();
            moment_vec = engine::raytrace::rotate_degrees(moment_vec, retun_t.rotation);
            retun_t.position += moment_vec / 10.0;
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('a') {
                retun_t.rotation.y += 1.0;
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('d') {
                retun_t.rotation.y += -1.0;
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('w') {
                retun_t.rotation.x += 1.0;
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('s') {
                retun_t.rotation.x += -1.0;
            }
            if retun_t.rotation.x > 40.0 {
                retun_t.rotation.x = 40.0;
            }
            if retun_t.rotation.x < -40.0 {
                retun_t.rotation.x = -40.0;
            }
        }
    }
    Some(retun_t)
}

fn test_world_cam(){
    let mut world = engine::raytrace::genrate_test_world();
    engine::raytrace::render_world_mesh(world);
}

fn test_loop_world() -> Result<i32> {
    let mut world = engine::raytrace::genrate_test_world();
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut rotation = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut position = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut framse: i32 = 0;
    let loop_start = Instant::now();
    loop {
        framse += 1;
        let start = Instant::now();
        let print_world = engine::raytrace::render_world(world);
        //let print =format!("{}\n roation(x:{:.2},y:{:.2},z:{:.2}) \n postion(x:{:.2},y:{:.2},z:{:.2} )",print_world.0,rotation.x,rotation.y,rotation.z,position.x,position.y,position.z);
        world = print_world.1;
        let print = print_world.0;
        let cam = world.camera;
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(print, area);
        })?;
        let duration = start.elapsed();
        let duration_seconds = (duration.as_millis() as f32) / 1000.0;
        let seconds_in_fram = 1.0 / FPS;
        let sec_left_in_frame = (seconds_in_fram - duration_seconds) * 1000.0;
        let mut sec_to_imput = sec_left_in_frame as u64;
        if sec_left_in_frame < 1.0 {
            sec_to_imput = 16;
        }

        let t = cam.get_transform();
        let result = get_input(t, sec_to_imput);
        if result.is_some() {
            rotation = result.unwrap().rotation;
            position = result.unwrap().position;
        } else {
            break;
        }
        world.camera.rotation = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        world.camera.origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        world.camera.rotation += rotation;
        world.camera.origin += position;
    }

    // TODO main loop
    let duration_of_loop = loop_start.elapsed();
    let fps = (framse as f32 / (duration_of_loop.as_secs() as f32)) as i32;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(fps)
}

fn main() {
    //test_world_cam();

    
    let fps = test_loop_world();
    if fps.is_ok() {
        println!("fps: {}", fps.unwrap());
    }
    //raytrace::test_render_in_text_mode();
    
}

use macroquad::prelude::*;
use std::time::{Duration, Instant};

#[macroquad::main("snake-android")]
async fn main() -> anyhow::Result<()> {
    loop {
        clear_background(LIGHTGRAY);

        const LM: usize = 40;
        const HM: usize = 20;
        let mut monde = vec![[0; LM]; HM];
        let mut x_player: i32 = 16;
        let mut y_player: i32 = 16;
        let mut vx: i32 = 1;
        let mut vy: i32 = 0;
        let mut instant = Instant::now();
        let mut snake_size = 2;

        let cellule_size = 15.;

        let mut x_appel = macroquad::rand::gen_range(0, LM - 1);
        let mut y_appel = macroquad::rand::gen_range(0, HM - 1);

        'jeux: loop {
            clear_background(BLACK);
            let passe = instant.elapsed();

            if passe > Duration::from_millis(150) {
                x_player += vx;
                y_player += vy;
                println!("{} {} {}", snake_size, x_player, y_player);

                instant = Instant::now();

                for i in &mut monde {
                    for a in i {
                        if *a > 0 {
                            *a -= 1;
                        }
                    }
                }
                if monde[y_player as usize][x_player as usize] > 0 {
                    println!("dd");
                    break 'jeux;
                }

                if y_appel as i32 == y_player && x_appel as i32 == x_player {
                    snake_size += 1;
                    x_appel = macroquad::rand::gen_range(0, LM - 1);
                    y_appel = macroquad::rand::gen_range(0, HM - 1);
                }

                monde[y_player as usize][x_player as usize] = snake_size;
            }

            monde[y_appel as usize][x_appel as usize] = 1;

            if is_key_down(KeyCode::Down) {
                vx = 0;
                vy = 1
            }
            if is_key_down(KeyCode::Up) {
                vx = 0;
                vy = -1;
            }
            if is_key_down(KeyCode::Left) {
                vx = -1;
                vy = 0;
            }
            if is_key_down(KeyCode::Right) {
                vx = 1;
                vy = 0;
            }

            for (idx_y, &i) in &mut monde.iter().enumerate() {
                for (idx_x, a) in &mut i.iter().enumerate() {
                    if *a == 0 {
                        draw_rectangle(
                            idx_x as f32 * cellule_size,
                            idx_y as f32 * cellule_size,
                            cellule_size,
                            cellule_size,
                            WHITE,
                        );
                    }
                    if *a > 0 {
                        draw_rectangle(
                            idx_x as f32 * cellule_size,
                            idx_y as f32 * cellule_size,
                            cellule_size,
                            cellule_size,
                            RED,
                        );
                    }
                }
            }

            let touchet = touches();

            for toouch in touchet {
                draw_text(&format!("touch {:?}", toouch.position), 50., 50., 30., BLUE);
                if 60. < toouch.position[0]
                    && toouch.position[0] < 120.
                    && screen_height() - 120. < toouch.position[1]
                    && toouch.position[1] < screen_height() - 60.
                {
                    vx = -1;
                    vy = 0;
                }
                if 180. < toouch.position[0]
                    && toouch.position[0] < 240.
                    && screen_height() - 120. < toouch.position[1]
                    && toouch.position[1] < screen_height() - 60.
                {
                    vx = 1;
                    vy = 0;
                }
                if 120. < toouch.position[0]
                    && toouch.position[0] < 180.
                    && screen_height() - 120. < toouch.position[1]
                    && toouch.position[1] < screen_height()
                {
                    vx = 0;
                    vy = 1;
                }
                if 120. < toouch.position[0]
                    && toouch.position[0] < 180.
                    && screen_height() - 180. < toouch.position[1]
                    && toouch.position[1] < screen_height() - 120.
                {
                    vx = 0;
                    vy = -1;
                }
            }

            draw_rectangle(60., screen_height() - 120., 60., 60., BLUE);
            draw_rectangle(180., screen_height() - 120., 60., 60., BLUE);
            draw_rectangle(120., screen_height() - 60., 60., 60., BLUE);
            draw_rectangle(120., screen_height() - 180., 60., 60., BLUE);

            next_frame().await
        }
    }
}

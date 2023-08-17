use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut move_x : f32 = 0.0;
    let mut move_y : f32 = 0.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Up){
            move_y -= 4.0;
        }
        else if is_key_down(KeyCode::Down){
            move_y += 4.0;
        }
        else if is_key_down(KeyCode::Left){
            move_x -= 4.0;
        }
        else if is_key_down(KeyCode::Right){
            move_x += 4.0;
        }

        draw_rectangle(move_x , move_y, 60.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_line(100.0, 100.0, 300.0, 300.0, 40.0, PINK);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

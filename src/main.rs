use macroquad::prelude::*;

#[macroquad::main("Black Jack")]
async fn main() {
    set_pc_assets_folder("./src/assets");
    // let font: Font = load_ttf_font("fonts/Space_Mono/SpaceMono-Bold.ttf")
    //     .await
    //     .unwrap();
    let WINDOW_WIDTH: f32 = screen_width();
    let WINDOW_HEIGHT: f32 = screen_height();

    let img = load_texture("images/A.png").await.unwrap();

    const MARGIN_X: f32 = 20.0;
    const MARGIN_Y: f32 = 10.0;
    const FONT_SIZE: f32 = 40.0;

    loop {
        clear_background(GREEN);

        //draw views for dealer and player
        //vertical  line 1

        draw_texture(&img, 2.0 * MARGIN_X, 2.0 * MARGIN_Y, WHITE);
        draw_line(
            MARGIN_X,
            MARGIN_Y,
            MARGIN_X,
            WINDOW_HEIGHT * 0.75,
            4.0,
            BLACK,
        );

        //vertical line 2
        draw_line(
            WINDOW_WIDTH / 2.0 - MARGIN_X,
            MARGIN_Y,
            WINDOW_WIDTH / 2.0 - MARGIN_X,
            WINDOW_HEIGHT * 0.75,
            4.0,
            BLACK,
        );

        //vertical line 3
        draw_line(
            WINDOW_WIDTH - MARGIN_X,
            MARGIN_Y,
            WINDOW_WIDTH - MARGIN_X,
            WINDOW_HEIGHT * 0.75,
            4.0,
            BLACK,
        );

        //top line
        draw_line(
            MARGIN_X,
            MARGIN_Y,
            WINDOW_WIDTH - MARGIN_X,
            MARGIN_Y,
            4.0,
            BLACK,
        );

        //bottom line
        draw_line(
            MARGIN_X,
            WINDOW_HEIGHT * 0.75,
            WINDOW_WIDTH - MARGIN_X,
            WINDOW_HEIGHT * 0.75,
            4.0,
            BLACK,
        );
        //draw the button and actions at the bottom of the screen
        // 1. Draw main dashboard
        draw_rectangle(
            WINDOW_WIDTH * 0.1,
            WINDOW_HEIGHT * 0.8,
            WINDOW_WIDTH * 0.8,
            62.0,
            WHITE,
        );

        //2. Draw Hit Button
        draw_rectangle(
            WINDOW_WIDTH * 0.15,
            WINDOW_HEIGHT * 0.82,
            WINDOW_WIDTH / 5.0,
            40.0,
            GREEN,
        );
        draw_text(
            "HIT",
            WINDOW_WIDTH * 0.2,
            WINDOW_HEIGHT * 0.87,
            FONT_SIZE,
            BLACK,
        );

        //3. Draw Stand Button
        draw_rectangle(
            WINDOW_WIDTH * 0.41,
            WINDOW_HEIGHT * 0.82,
            WINDOW_WIDTH / 5.0,
            40.0,
            YELLOW,
        );
        draw_text(
            "STAND",
            WINDOW_WIDTH * 0.44,
            WINDOW_HEIGHT * 0.87,
            FONT_SIZE,
            BLACK,
        );

        //4. Draw Deal Button
        draw_rectangle(
            WINDOW_WIDTH * 0.65,
            WINDOW_HEIGHT * 0.82,
            WINDOW_WIDTH / 5.0,
            40.0,
            RED,
        );

        draw_text(
            "DEAL",
            WINDOW_WIDTH * 0.7,
            WINDOW_HEIGHT * 0.87,
            FONT_SIZE,
            BLACK,
        );

        // score text
        draw_text(
            format!("Score:  {}", 4).as_str(),
            WINDOW_HEIGHT / 2.0,
            WINDOW_HEIGHT - (FONT_SIZE / 2.0),
            FONT_SIZE,
            WHITE,
        );

        next_frame().await
    }
}

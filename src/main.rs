/*
By: <Abdul Baig>
Date: 2026-05-12
Program Details: <Clearly display who wins each game.

Prevent the player from changing an already selected spot (X or O) on the board.

Implement computer logic that attempts to win or block the player from winning.

Use a single, consistent method to handle moves for all spots on the board (no separate methods per position).

Keep track of and display the running score of wins for both the player and the computer.>
*/

mod modules;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "tictactoes".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}




#[macroquad::main(window_conf)]
async fn main() {



let mut lbl_winner = Label::new("", 250.0, 250.0, 50);
let mut lbl_player_score = Label::new("Player: 0", 350.0, 500.0, 30);
let mut lbl_computer_score = Label::new("Computer: 0", 350.0, 550.0, 30);


let btn_exit = TextButton::new(750.0, 50.0, 50.0, 50.0, "X", RED, BLACK, 50);

 let mut btn_box1 = TextButton::new(200.0, 250.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box1.with_text_color(BLACK);
    btn_box1.with_alignment(modules::text_button::TextAlign::Center);


    let mut btn_box2 = TextButton::new(300.0, 250.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box2.with_text_color(BLACK);
    btn_box2.with_alignment(modules::text_button::TextAlign::Center);


    let mut btn_box3 = TextButton::new(400.0, 250.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box3.with_text_color(BLACK);
    btn_box3.with_alignment(modules::text_button::TextAlign::Center);

    
    let mut btn_box4 = TextButton::new(200.0, 350.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box4.with_text_color(BLACK);
    btn_box4.with_alignment(modules::text_button::TextAlign::Center);

    
    let mut btn_box5 = TextButton::new(300.0, 350.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box5.with_text_color(BLACK);
    btn_box5.with_alignment(modules::text_button::TextAlign::Center);

    
    let mut btn_box6 = TextButton::new(400.0, 350.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box6.with_text_color(BLACK);
    btn_box6.with_alignment(modules::text_button::TextAlign::Center);

    
    let mut btn_box7 = TextButton::new(200.0, 450.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box7.with_text_color(BLACK);
    btn_box7.with_alignment(modules::text_button::TextAlign::Center);

    
    let mut btn_box8 = TextButton::new(300.0, 450.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box8.with_text_color(BLACK);
    btn_box8.with_alignment(modules::text_button::TextAlign::Center);

    
    let mut btn_box9 = TextButton::new(400.0, 450.0, 100.0, 100.0, "", WHITE, GRAY, 60);
    btn_box9.with_text_color(BLACK);
    btn_box9.with_alignment(modules::text_button::TextAlign::Center);











    loop {


        
        clear_background(WHITE);

 draw_grid(50.0, BLACK);
     
     
     
     
     
     lbl_winner.draw();
lbl_computer_score.draw();
lbl_player_score.draw();



     if btn_box1.click() {
  
     };
     
     if btn_box2.click() {

     };
     
     if btn_box3.click() {

     };
     
     if btn_box4.click() {

     };
     
     if btn_box5.click() {

     };
     
     if btn_box6.click() {

     };
     
     if btn_box7.click() {

     };
     
     if btn_box8.click() {

     };
    
     if btn_box9.click() {

     };
     
     
     
     
     if btn_exit.click() {

        break;
    }
     
        next_frame().await;
    }
}

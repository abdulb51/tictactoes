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
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

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


    let mut btn_reset = TextButton::new(550.0, 250.0, 100.0, 50.0, "Reset", RED, BLACK, 30);
    btn_reset.with_alignment(modules::text_button::TextAlign::Center);
    
    let mut lbl_winner = Label::new("", 250.0, 250.0, 50);
   
    let mut lbl_player_score = Label::new("Player: 0", 350.0, 500.0, 30);
    
    let mut lbl_computer_score = Label::new("Computer: 0", 350.0, 550.0, 30);
   
    let mut btn_exit = TextButton::new(550.0, 100.0, 50.0, 50.0, "X", RED, BLACK, 50);
    btn_exit.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box1 = TextButton::new(200.0, 250.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box1.with_text_color(BLACK);
    btn_box1.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box2 = TextButton::new(300.0, 250.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box2.with_text_color(BLACK);
    btn_box2.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box3 = TextButton::new(400.0, 250.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box3.with_text_color(BLACK);
    btn_box3.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box4 = TextButton::new(200.0, 350.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box4.with_text_color(BLACK);
    btn_box4.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box5 = TextButton::new(300.0, 350.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box5.with_text_color(BLACK);
    btn_box5.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box6 = TextButton::new(400.0, 350.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box6.with_text_color(BLACK);
    btn_box6.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box7 = TextButton::new(200.0, 450.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box7.with_text_color(BLACK);
    btn_box7.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box8 = TextButton::new(300.0, 450.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box8.with_text_color(BLACK);
    btn_box8.with_alignment(modules::text_button::TextAlign::Center);

    let mut btn_box9 = TextButton::new(400.0, 450.0, 100.0, 100.0, "", WHITE, GRAY, 100);
    btn_box9.with_text_color(BLACK);
    btn_box9.with_alignment(modules::text_button::TextAlign::Center);

    let mut btnclick = 0;
    let mut playerscore = 0;
    let mut computerscore = 0;

    let mut buttons = vec![btn_box1, btn_box2, btn_box3, btn_box4, btn_box5, btn_box6, btn_box7, btn_box8, btn_box9];

    loop {
        clear_background(WHITE);

        draw_grid(50.0, BLACK);

        for spot in 0..buttons.len() {
            if buttons[spot].click() {
                buttons[spot].set_text("X");
                buttons[spot].enabled = false;
                btnclick += 1;

                if buttons[0].get_text() == "X" && buttons[1].get_text() == "X" && buttons[2].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }

                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore));
                } 
                else if buttons[3].get_text() == "X" && buttons[4].get_text() == "X" && buttons[5].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore));
                } 
                else if buttons[6].get_text() == "X" && buttons[7].get_text() == "X" && buttons[8].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore));
                } 
                else if buttons[0].get_text() == "X" && buttons[3].get_text() == "X" && buttons[6].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore));
                } 
                else if buttons[1].get_text() == "X" && buttons[4].get_text() == "X" && buttons[7].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore));
                } 
                else if buttons[2].get_text() == "X" && buttons[5].get_text() == "X" && buttons[8].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore));
                } 
                else if buttons[0].get_text() == "X" && buttons[4].get_text() == "X" && buttons[8].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore));
                } 
                else if buttons[2].get_text() == "X" && buttons[4].get_text() == "X" && buttons[6].get_text() == "X" {
                    lbl_winner.set_text("X Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    playerscore += 1;
                    lbl_player_score.set_text(format!("Player: {}", playerscore)); 
                }
                
                
                
                  if btnclick <= 9 {
                    let mut comp_turn = false;
                    while comp_turn == false {
                        let randombox = rand::gen_range(0, 9);
                        if buttons[randombox].enabled == true {
                            buttons[randombox].set_text("O");
                            buttons[randombox].enabled = false;
                            btnclick += 1;
                            comp_turn = true;
         
                        }
                
                
          if buttons[0].get_text() == "O" && buttons[1].get_text() == "O" && buttons[2].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                else if buttons[3].get_text() == "O" && buttons[4].get_text() == "O" && buttons[5].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                else if buttons[6].get_text() == "O" && buttons[7].get_text() == "O" && buttons[8].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                else if buttons[0].get_text() == "O" && buttons[3].get_text() == "O" && buttons[6].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                else if buttons[1].get_text() == "O" && buttons[4].get_text() == "O" && buttons[7].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                else if buttons[2].get_text() == "O" && buttons[5].get_text() == "O" && buttons[8].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                else if buttons[0].get_text() == "O" && buttons[4].get_text() == "O" && buttons[8].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                else if buttons[2].get_text() == "O" && buttons[4].get_text() == "O" && buttons[6].get_text() == "O" {
                    lbl_winner.set_text("O Wins!");
                    for buttonsdisable in 0..buttons.len() {
                        buttons[buttonsdisable].enabled = false;
                    }
                    computerscore += 1;
                    lbl_computer_score.set_text(format!("Computer: {}", computerscore));
                } 
                   
                        if buttons[0].enabled == false
                            && buttons[1].enabled == false
                            && buttons[2].enabled == false
                            && buttons[3].enabled == false
                            && buttons[4].enabled == false
                            && buttons[5].enabled == false
                            && buttons[6].enabled == false
                            && buttons[7].enabled == false
                            && buttons[8].enabled == false 
                        {
                            break;
                        }
                    }
                }
                
         
            }
        }


        lbl_winner.draw();
        lbl_computer_score.draw();
        lbl_player_score.draw();

        if btn_reset.click() {
            for enablebuttons in 0..buttons.len() {
                buttons[enablebuttons].set_text("");
                buttons[enablebuttons].enabled = true;
                btnclick = 0;
            }
            lbl_winner.set_text("");
        }

        if btn_exit.click() {
            break;
        }


        
        next_frame().await;
    }
}


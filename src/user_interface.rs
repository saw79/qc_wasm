use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonType {
    WAIT,
    BAG,
    GRAB,
    TARGET,
    ATTACK,
}

#[derive(Debug)]
pub enum ButtonState {
    UP,
    DOWN,
    CHECKED,
}

#[derive(Debug)]
pub struct ButtonInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub state: ButtonState,
    pub skin_img: Option<String>,
}

pub struct UserInterface {
    pub buttons: HashMap<ButtonType, ButtonInfo>,
    pub button_down: Option<ButtonType>,
}

impl UserInterface {
    pub fn new(width: i32, height: i32, tile_pix: i32) -> Self {
        let mut buttons = HashMap::new();

        let mut btn_size = 2*tile_pix;
        let mut btn_pad = tile_pix/2;

        let button_bar_width = 5*(btn_size + btn_pad) + btn_pad;
        if button_bar_width > width {
            let ratio = (width as f32) / (button_bar_width as f32);
            btn_size = (btn_size as f32 * ratio) as i32;
            btn_pad = (btn_pad as f32 * ratio) as i32;
        }

        let ypos = height - btn_size - btn_pad;

        buttons.insert(ButtonType::WAIT, ButtonInfo {
            x: width/2 - btn_size/2 - (btn_size+btn_pad)*2,
            y: ypos,
            width: btn_size,
            height: btn_size,
            state: ButtonState::UP,
            skin_img: Some("btn_skin_wait".to_string()),
        });
        buttons.insert(ButtonType::BAG, ButtonInfo {
            x: width/2 - btn_size/2 - (btn_size+btn_pad)*1,
            y: ypos,
            width: btn_size,
            height: btn_size,
            state: ButtonState::UP,
            skin_img: Some("btn_skin_bag".to_string()),
        });
        buttons.insert(ButtonType::GRAB, ButtonInfo {
            x: width/2 - btn_size/2,
            y: ypos,
            width: btn_size,
            height: btn_size,
            state: ButtonState::UP,
            skin_img: Some("btn_skin_grab".to_string()),
        });
        buttons.insert(ButtonType::TARGET, ButtonInfo {
            x: width/2 - btn_size/2 + (btn_size+btn_pad)*1,
            y: ypos,
            width: btn_size,
            height: btn_size,
            state: ButtonState::UP,
            skin_img: Some("btn_skin_target".to_string()),
        });
        buttons.insert(ButtonType::ATTACK, ButtonInfo {
            x: width/2 - btn_size/2 + (btn_size+btn_pad)*2,
            y: ypos,
            width: btn_size,
            height: btn_size,
            state: ButtonState::UP,
            skin_img: None,
        });

        UserInterface {
            buttons: buttons,
            button_down: None,
        }
    }

    pub fn log_click_down(&mut self, x: i32, y: i32) -> Option<ButtonType> {
        for (bt, bi) in self.buttons.iter_mut() {
            if x >= bi.x && x < bi.x + bi.width && y >= bi.y && y < bi.y + bi.height {
                self.button_down = Some(bt.clone());
                bi.state = ButtonState::DOWN;
                return Some(bt.clone());
            }
        }

        self.button_down = None;
        None
    }

    pub fn log_click_up(&mut self, x: i32, y: i32) -> Option<ButtonType> {
        match self.button_down.clone() {
            Some(bd_type) => {
                self.buttons.get_mut(&bd_type)?.state = ButtonState::UP;
                self.button_down = None;
                let bd = &self.buttons[&bd_type];
                if x >= bd.x && x < bd.x + bd.width && y >= bd.y && y < bd.y + bd.height {
                    Some(bd_type)
                } else {
                    None
                }
            },
            None => None,
        }
    }
}


use std::collections::HashMap;
use num::clamp;
use winit::event::VirtualKeyCode;
use pseudo_term::{
    env::EnvironmentBuilder,
    obj::GameObject
};

const MOVE_SPD: f32 = 5.0;
const ANIM_SPD: f32 = 2.0;
const ANIMATIONS: [(&'static str, &'static [[[char; 3]; 5]]); 2] = [
    (
        "idle", &[
            [
                [ '\0',  '@', '\0' ],
                [  '-',  '+',  '-' ],
                [ '\0',  '^', '\0' ],
                [  '/', '\0', '\\' ],
                [  '|', '\0',  '|' ]
            ]
        ]
    ), (
        "walk", &[
            [
                [ '\0',  '@',  '/' ],
                [ '\0',  '+', '\0' ],
                [  '/',  '^', '\0' ],
                [  '^',  '|', '\0' ],
                [ '\0', '\0',  '-' ]
            ], [
                [ '\\',  '@', '\0' ],
                [ '\0',  '+', '\0' ],
                [ '\0',  '^', '\\' ],
                [ '\0',  '|',  '^' ],
                [  '-', '\0', '\0' ]
            ]
        ]
    )
];

#[derive(Clone)]
struct Player {
    position: (f32, f32),
    velocity: (f32, f32),

    anims: HashMap<&'static str, &'static [[[char; 3]; 5]]>,
    cur_anim: String,
    anim_frame: f32,

    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: (35.0, 12.0),
            velocity: (0.0, 0.0),

            anims: HashMap::from(ANIMATIONS),
            cur_anim: "idle".to_string(),
            anim_frame: 0.0,

            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false
        }
    }
}

impl GameObject for Player {
    fn obj_type(&self) -> String {
        "Player".to_string()
    }

    fn persistant(&self) -> bool {
        false
    }

    fn on_key_pressed(
            &mut self, code: VirtualKeyCode,
            _global_objs: &Vec<Box<dyn GameObject>>,
            _rooms: &HashMap<String, Vec<Box<dyn GameObject>>>,
            _cur_room: &mut String) {
        match code {
            VirtualKeyCode::Up => self.up_pressed = true,
            VirtualKeyCode::Down => self.down_pressed = true,
            VirtualKeyCode::Left => self.left_pressed = true,
            VirtualKeyCode::Right => self.right_pressed = true,
            _ => {}
        }
    }

    fn on_key_released(
            &mut self, code: VirtualKeyCode,
            _global_objs: &Vec<Box<dyn GameObject>>,
            _rooms: &HashMap<String, Vec<Box<dyn GameObject>>>,
            _cur_room: &mut String) {
        match code {
            VirtualKeyCode::Up => self.up_pressed = false,
            VirtualKeyCode::Down => self.down_pressed = false,
            VirtualKeyCode::Left => self.left_pressed = false,
            VirtualKeyCode::Right => self.right_pressed = false,
            _ => {}
        }
    }

    fn update(
            &mut self, delta_time: f32,
            _global_objs: &Vec<Box<dyn GameObject>>,
            _rooms: &HashMap<String, Vec<Box<dyn GameObject>>>,
            _cur_room: &mut String) {
        let last_vel = self.velocity;
        self.velocity = (0.0, 0.0);
        if self.up_pressed {
            self.velocity.1 -= MOVE_SPD;
        }
        if self.down_pressed {
            self.velocity.1 += MOVE_SPD;
        }
        if self.left_pressed {
            self.velocity.0 -= MOVE_SPD;
        }
        if self.right_pressed {
            self.velocity.0 += MOVE_SPD;
        }
        if last_vel == (0.0, 0.0) && self.velocity != (0.0, 0.0) {
            self.cur_anim = "walk".to_string();
            self.anim_frame = 0.0;
        } else if last_vel != (0.0, 0.0) && self.velocity == (0.0, 0.0) {
            self.cur_anim = "idle".to_string();
            self.anim_frame = 0.0;
        }

        self.position.0 += self.velocity.0 * delta_time;
        self.position.1 += self.velocity.1 * delta_time;
        self.position.0 = clamp(self.position.0, 0.0 + 1.0, 79.0 - 1.0);
        self.position.1 = clamp(self.position.1, 0.0 + 2.0, 24.0 - 2.0);

        let anim_frames = self.anims.get(self.cur_anim.as_str()).unwrap();
        self.anim_frame += ANIM_SPD * delta_time;
        if self.anim_frame as usize > anim_frames.len() - 1 {
            self.anim_frame = 0.0;
        }
    }

    fn draw(&self, text_buf: &mut [[char; 81]; 25]) {
        let anim_frames = self.anims.get(self.cur_anim.as_str()).unwrap();
        let pic = anim_frames[self.anim_frame as usize];
        for y in 0..5 {
            for x in 0..3 {
                if pic[y][x] == '\0' {
                    continue;
                }
                text_buf[self.position.1 as usize + y - 2][self.position.0 as usize + x - 1] =
                    pic[y][x];
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = EnvironmentBuilder::new("main")
        .add_room("main", &vec![ Box::new(Player::new()) ])
        .build().await?;
    env.run().await?;
    Ok(())
}


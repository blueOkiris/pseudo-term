// Author(s): Dylan Turner <dylan.turner@tutanota.com>
//! Trait that can be used to define game objects and what they look like/do

use std::collections::HashMap;
use winit::event::VirtualKeyCode;

/// An object with animations, position, and behaviors. Takes an enum as generic arg
pub trait GameObject: GameObjectClone + Sync + Send {
    /// User defined type for classifying objects for help in implementing behaviors
    fn obj_type(&self) -> String where Self: Sized + Clone;

    /// Whether or not an object resets everytime its room is switched to or not
    fn persistant(&self) -> bool where Self: Sized + Clone;

    /// Tell the GameObject what to do when a key is pressed
    fn on_key_pressed(
        &mut self, code: VirtualKeyCode,
        global_objs: &Vec<Box<dyn GameObject>>,
        rooms: &HashMap<String, Vec<Box<dyn GameObject>>>,
        cur_room: &mut String
    );

    /// Same as on_key_pressed, but for released
    fn on_key_released(
        &mut self, code: VirtualKeyCode,
        global_objs: &Vec<Box<dyn GameObject>>,
        rooms: &HashMap<String, Vec<Box<dyn GameObject>>>,
        cur_room: &mut String
    );

    /// How to continuously modify the object
    fn update(
        &mut self, delta_time: f32,
        global_objs: &Vec<Box<dyn GameObject>>,
        rooms: &HashMap<String, Vec<Box<dyn GameObject>>>,
        cur_room: &mut String
    );

    /// Allow drawing to the screen buffer each render frame.
    /// Chars at index 80 are new lines, so change them at your own risk!
    fn draw(&self, text_buf: &mut [[char; 81]; 25]);
}

/// Allows us to store GameObjects in Vecs
pub trait GameObjectClone: {
    fn clone_box(&self) -> Box<dyn GameObject>;
}

impl<T> GameObjectClone for T where T: 'static + GameObject + Clone {
    fn clone_box(&self) -> Box<dyn GameObject> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GameObject> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}


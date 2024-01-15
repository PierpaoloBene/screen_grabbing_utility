use std::ops::Index;
use std::str::FromStr;
use std::string;

use global_hotkey::hotkey::HotKey;
use keyboard_types::Modifiers;
use keyboard_types::Code;

pub struct Hotkeys{
    hotkeys_vector: Vec<HotKey>,
}


impl Hotkeys{
    pub fn new()-> Self{
        Hotkeys{hotkeys_vector: 
            vec![
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyE),  //Exit
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyD),  //Screen
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyS),  //Save
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyC),  //Copy
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyA),  //Save with name
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyG),  //Crop
            ]
        }
      
    }
    
    pub fn get_hotkeys(&self)-> Vec<HotKey>{
        self.hotkeys_vector.clone()
    }
    pub fn update_hotkey(&mut self, id: usize, modifier:String, code: String){
        
        let mut modifier_name: String = "CONTROL".to_string();
 
    
        match modifier.as_str(){
            "alt" => {modifier_name = "ALT".to_string()},
            "ctrl" => {modifier_name = "CONTROL".to_string()},
            "shift" => {modifier_name = "SHIFT".to_string()},
            "mac_cmd" => {modifier_name = "CONTROL".to_string()},
            "command" => {modifier_name = "CONTROL".to_string()},
            _ => {}
        }

        println!("{:?}", self.hotkeys_vector.get(id));
        *self.hotkeys_vector.get_mut(id).unwrap() = HotKey::new(Modifiers::from_name(modifier_name.as_str()), Code::from_str(format!("Key{}",code).as_str()).unwrap());
        
        println!("{:?}",  HotKey::new(Modifiers::from_name(modifier_name.as_str()), Code::from_str(format!("Key{}",code).as_str()).unwrap()));
        println!("{:?}", self.hotkeys_vector.get(id));
    }
}
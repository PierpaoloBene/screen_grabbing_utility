use std::ops::Index;
use std::str::FromStr;
use std::string;

use global_hotkey::hotkey::HotKey;
use keyboard_types::Modifiers;
use keyboard_types::Code;

pub struct CustomizeHotkey{
    id: usize,
    modifier: String,
    code: String,
}

impl Default for CustomizeHotkey{
    fn default() -> Self {
        CustomizeHotkey{
            id: usize::MAX,
            modifier: "modifier".to_string(),
            code:   "Key".to_string(),
        }
    }
}

impl PartialEq for CustomizeHotkey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.modifier == other.modifier && self.code == other.code
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl CustomizeHotkey{
    pub fn new(id: usize,modifier: String,code: String)->Self{
        CustomizeHotkey { id: id, modifier: modifier, code: code }
    }

    
}
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
    pub fn update_hotkey(&mut self, new_hotkey: &CustomizeHotkey){
        
        let mut modifier_name: String = "CONTROL".to_string();
 
    
        match new_hotkey.modifier.as_str(){
            "alt" => {modifier_name = "ALT".to_string()},
            "ctrl" => {modifier_name = "CONTROL".to_string()},
            "shift" => {modifier_name = "SHIFT".to_string()},
            "mac_cmd" => {modifier_name = "CONTROL".to_string()},
            "command" => {modifier_name = "CONTROL".to_string()},
            _ => {}
        }

        println!("{:?}", self.hotkeys_vector.get(new_hotkey.id));
        *self.hotkeys_vector.get_mut(new_hotkey.id).unwrap() = HotKey::new(Modifiers::from_name(modifier_name.as_str()), Code::from_str(format!("Key{}",new_hotkey.code).as_str()).unwrap());
        
        println!("{:?}",  HotKey::new(Modifiers::from_name(modifier_name.as_str()), Code::from_str(format!("Key{}",new_hotkey.code).as_str()).unwrap()));
        println!("{:?}", self.hotkeys_vector.get(new_hotkey.id));
    }
}
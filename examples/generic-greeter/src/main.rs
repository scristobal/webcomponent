extern crate stdweb;
extern crate webcomponent;
use std::collections::HashMap;

use webcomponent::{
    WebComponent,
    define,
    set_inner_html,
    log
};

struct GenericGreeter {
    greeting: String,
    name: String
}

impl Default for GenericGreeter {
    fn default() -> GenericGreeter {
        GenericGreeter {
            greeting: String::from("Hello"),
            name: String::from("World"),
        }
    }
}

impl WebComponent for GenericGreeter {
    fn get_observable_attributes() -> Vec<&'static str> {vec!["greeting","name"]}

    fn created(&mut self){
        self.render();
    }

    fn attribute_changed(&mut self,attribute_name:String,_old_value:stdweb::Value,new_value:stdweb::Value){
        if attribute_name == "greeting"{
            self.greeting = new_value.into_string().unwrap();
        } else if attribute_name == "name" {
            self.name = new_value.into_string().unwrap();
        }
        self.render();
    }
}

impl GenericGreeter {
    fn render(&mut self){
        set_inner_html(&format!("{} {}! ",self.greeting,self.name));
    }
}

fn main() {
    // get std wb started
    stdweb::initialize();

    // define the web components we will use
    define::<GenericGreeter>("generic-greeter");

    // keep std event going
    stdweb::event_loop();
}

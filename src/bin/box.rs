#![allow(dead_code)]

trait View {
    fn render(&self) {
        println!("Rendering component..");
    }
}

struct Button {
    label: String,
}

impl View for Button {
    fn render(&self) {
        println!("[ {} ]", self.label)
    }
}

struct Container {
    name: String,
    child: Option<Box<dyn View>>, // Box adds indirection and breaks the cycle in the recursive type
}

impl Container {
    fn new<V>(name: &str, child: V) -> Self
    where
        V: View + 'static,
    {
        Self {
            name: name.into(),
            child: Some(Box::new(child)),
        }
    }
}

impl View for Container {
    fn render(&self) {
        print!("<container@{}:\n  ", self.name);
        if let Some(child) = &self.child {
            child.render();
        } else {
            println!("None");
        }
        println!(">")
    }
}

fn main() {
    let button_a = Box::new(Button {
        label: "Opt-in for notification".into(),
    });
    let button_b = Button {
        label: "Cancel".into(),
    };
    let container = Box::new(Container::new("Notifications", button_b));

    let views: Vec<Box<dyn View>> = vec![button_a, container];
    views.into_iter().for_each(|view| view.render());
}

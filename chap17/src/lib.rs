// We've seen a big limitation with what we have in our type system rn : collections are homoegeneous (though enums allow
// us to alleviate that a little)
// Let's see how we can get dynamic type dispatch in the following examples

pub mod gui {
// Let's say that we want to make a gui library with extensible drawable components. In pure OO, we would most likely
// define an interface IComponent having a method draw() and then have the other components (Button, Image, SelectBox, ...)
// implement the method. In rust we can't do that because we don't have inheritance strictly speaking, but we can use
// something else.
// The solution? Trait objects. Trait objects can't hold data and only have an interface. They, however, allow dynamic
// dispatch of the method on a concrete type.

pub trait Draw {
    fn draw(&self);
}

// The "dyn" keyword allows us to create a trait object, the Box<T> is also needed (the trait object must be a pointer)
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn render(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// The big difference between a generic with a trait constrain and a trait object is that the concrete type of the generic
// must be known at compile time, the trait object can be substituted for any concrete object implementing the said trait
// at runtime. If we were to do the following :
// pub struct Screen<T: Draw> {
//     pub component: Vec<T>,
// }
// 
// impl<T> Screen<T> {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// Then we would be forced to have a homogeneous collection, with only one type of component, be it TextFields, Buttons,...
// Now let's add some types

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button : {:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw select box : {:?}", self);
    }
}

// To be used as trait objects, traits must be "object-safe", two rules are most relevant :
// - The return type isn't Self
// - There are no generic type parameters
// As trait objects are implemented through type erasure, the Self type (the type of the concrete class)
// is forgotten alongside the generic type parameters. The clone trait is not object-safe for instance.

}

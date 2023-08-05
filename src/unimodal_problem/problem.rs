// Import than rand library for random numbers
use rand::prelude::*;

// define the data structure of the UnimodalProblem struct.
// Note that Rust does not use classes. It has no inheritance, and provides no
// default constructors (nor deconstructors, since Rust is inherently memory safe).
// In order to construct an instance of this class, I will be using the "builder"
// paradigm. See below.
pub struct UnimodalProblem {
    pub x_offset: f32,
    pub scale_factor: f32,
}

// Provide the implementations for the UnimodalProblem struct. These
// implementations provide the functions that act on the data in the struct, as
// well as static functions that are part of the implementation, but can be
// executed without an instance (E.G. if you want to make a constructor)

// Implementations are a super cool system, especially because you can provice implementations for
// specific "Traits", which allows you to use the struct as an argument to any function whose type
// is generic, and only requires that a specific trait to be implemented. This allows different
// structs to be passed to the same function, similar to c++, but without any inheritance.

// Implementations for traits are such that you declare that you are going to implement said trait,
// then provide the functions that the implementation requires (and obeying the necessary function
// signatures), in whichever way makes sense to implement for your specific struct.
impl UnimodalProblem {
    pub fn calc( // "pub" means that this function is available to things outside of this struct.
            &self, // This makes the function an instance method, requiring to be run on an instance.
                   // Specifically, this function takes an immutable reference to the instance it is
                   // being run on, meaning that it does not consume the instance and it cannot
                   // modify it.
            x: f32 // take ownership of an immutable f32
            ) -> f32 {
        self.scale_factor
          * f32::abs(1.0 / (x-self.x_offset)) // the `return` keyword is not necessary if the
                                                   // final expression does not end with a semicolon
                                                   // (and it's the correct return type)
    }
}

// The builder syntax became popular, since a struct cannot be partially constructed, and defaults
// cannot be defined for a struct. To get around this, you make a builder struct that contains
// good defaults, functions to modify the values that will be used to make the struct, and a
// function to actually build and return the struct.
pub struct UnimodalProblemBuilder {
    x_offset: f32,
    scale_factor: f32,
}

// The implementations for the builder
impl UnimodalProblemBuilder {
    pub fn new() -> UnimodalProblemBuilder { // Note no "self" keyword, so this is a static function
                                             // that is executed without an instance.
        UnimodalProblemBuilder { // return an instance of the builder with my default values
            x_offset: 0.0,
            scale_factor: 0.0,
        }
    }

    pub fn randomize(&mut self) -> &UnimodalProblemBuilder { // This is an instance method that
                                                            // borrows (does not consume) the instance,
                                                            // and is allowed to modify it.
        let mut rng = rand::thread_rng();
        self.x_offset = (rng.gen::<f32>() - 0.5) * 100.0;
        self.scale_factor = (rng.gen::<f32>() - 0.5) * 20.0;
        self // Returning itself, so that instance methods can be chained together on a single line.
             // E.G. UnimodalProblemBuilder::new().randomize().do_other_thing().set_val(5.0).build()
    }

    pub fn build(&self) -> UnimodalProblem { // borrow the instance immutably, and use it to construct
                                             // an instance of UnimodalProblem.
        UnimodalProblem {
            x_offset: self.x_offset,
            scale_factor: self.scale_factor,
        }
    }
}

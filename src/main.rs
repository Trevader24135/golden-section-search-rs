// Also note, the official Rust documentation is super freaking good,
// and by far the best resource to learn Rust:
//   https://doc.rust-lang.org/book/title-page.html

// This brings the external (downloaded) lazy_static crate (package) into scope in this file,
// only bringing in the piece I want to use (the lazy_static! macro).
use lazy_static::lazy_static;

use argparse_rs::{ArgParser, ArgType};

// This "imports" the other module that I made for this demo, allowing us to access
// it's contents similar to namespaces or classes in c++ (via ::)
mod unimodal_problem;

// This is similar to "using" in c++, and allows us to access the pieces of the module
// that we are going to use without it's full "path".
use unimodal_problem::{UnimodalProblem, UnimodalProblemBuilder};

// Rust does not currently support calling many types of functions when setting constants,
// but you can get mostly the same behavior by using the lazy static crate. Doing this,
// you need to access their values similarly to dereferencing pointers.
lazy_static! {
    static ref PHI: f32 = (1.0 + 5.0_f32.sqrt()) / 2.0;
    static ref RESPHI: f32 = 2.0 - *PHI;
}

fn golden_section_search(
        problem: &UnimodalProblem,  // This argument is borrowing an immutable reference
        mut lower_bound: f32,       // These arguments are taking ownership of the passed
        mut upper_bound: f32,       //   arguments, and allow mutating their values.
        xtol: f32                   // This argument takes immutable ownership of this parameter
        ) -> (f32, f32) {           // This function returns a tuple of two float 32s

    // variables need declared as mutable (mut) if you intend to change their values later.
    // You actually need to change variables far less than you might think, if you've never
    // thought about it before. This search algorithm is a bit of an exception since it moves
    // the search boundaries inwards each time.

    // Alternatively you can have immutable variables and "overwrite" (shadow) them by using "let"
    // again. Shadowing variables respects scope however, and will not overwrite existing variables
    // in outside scopes, which doesn't work in this case.
    let mut lower_search = lower_bound + *RESPHI * (upper_bound - lower_bound);
    let mut lower_val = problem.calc(lower_search);

    let mut upper_search = upper_bound - *RESPHI * (upper_bound - lower_bound);
    let mut upper_val = problem.calc(upper_search);

    // The conditionals and loops bear far more resemblance to Python than C, with the notable
    // exception of using braces instead of indentation.
    while f32::abs(upper_bound - lower_bound) > xtol {
        if lower_val < upper_val {
            upper_bound = upper_search;
            upper_search = lower_search;
            upper_val = lower_val;

            lower_search = lower_bound + *RESPHI * (upper_bound - lower_bound);
            lower_val = problem.calc(lower_search);
        } else {
            lower_bound = lower_search;
            lower_search = upper_search;
            lower_val = upper_val;

            upper_search = upper_bound - *RESPHI * (upper_bound - lower_bound);
            upper_val = problem.calc(upper_search);
        }
        println!("{} {} {}", lower_bound, upper_bound, f32::abs(upper_bound - lower_bound));
    }
    let final_x = (upper_bound + lower_bound) / 2.0;
    let final_val = problem.calc(final_x);
    (final_x, final_val)
}

// `main` is the entrypoint. It is argumentless. In order to accept commandline arguments, you can
// use std::env::args(), or you can use a purpose built module such as argparse-rs which mimics
// Python's argparse library
fn main() {
    let mut parser = ArgParser::new("Golden Section Search Example".into());

    parser.add_opt(
        "tolerance",
        "2.0".into(),
        't',
        false,
        "The tolerance in the independent variable axis to be achieved before finishing",
        ArgType::Option
    );

    let args: Vec<String> = std::env::args().collect();
    let args = parser.parse(args.iter()).unwrap();

    if args.get::<bool>("help").expect("Error getting 'help' from argument parser!") {
        parser.help();
        std::process::exit(0);
    }

    let problem = UnimodalProblemBuilder::new().randomize().build();

    let (x_min, val) = golden_section_search(
        &problem,            // Pass problem as an immutable reference (the function does not take ownership and cannot modify it)
        -200.0, // Simple float value. Rust can guess whether it should be f32 or f64, or you can specify explicitly
        200.0,  // Notably, Rust will not allow you to pass an integer as a float. They are kept separate, and one
        args.get::<f32>("tolerance") // needs cast explicitly into the other. Hence, I specify 200.0 instead of 200.
            .expect("Error parsing x-tolerance from commandline arguments!")
                             // You can also do 200_f32 to be specific.
                             // Rust also allows numbers with underscore separators, EG 10_000.0 for 10 thousand (huzzah!)
    );

    // The standard print is done with the println! macro. Macros are similar to c/c++ in that they are basically code that expands
    // into other code. Unlike c/c++ they are incredibly powerful and can do some crazy cool things. They are written more like
    // special functions than simple substitutions. They're worth reading up on, because they're dang cool:
    //   https://doc.rust-lang.org/book/ch19-06-macros.html
    println!("Random offset: {} Offset estimate: {} minimum value: {}", problem.x_offset, x_min, val);
}

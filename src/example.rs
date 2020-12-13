trait Jakelike {
    fn jake(&self);
}

struct Person {}

impl Jakelike for Person {
    fn jake(&self) {
        println!("I am jakelike")
    }
}

impl Jakelike for &str {
    fn jake(&self) {
        println!("string are also jakelike");
    }
}

fn main() {
    foo("");
}

fn foo(j: impl Jakelike) {
    j.jake()
}

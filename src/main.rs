// Faust generated code

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
struct Dsp {
    pub a: [f32; 1024 * 1024],
}

impl Dsp {
    pub fn new() -> Self {
        Self {
            a: [0.0; 1024 * 1024],
        }
    }

    #[cfg(feature = "default-boxed")]
    pub fn new_boxed() -> Box<Self> {
        use default_boxed::DefaultBoxed;
        Self::default_boxed()
    }

    #[cfg(not(feature = "default-boxed"))]
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

// Architecture file
fn main() {
    let dsp: Box<Dsp> = Dsp::new_boxed();
    println!("{}", dsp.a[0]);
}

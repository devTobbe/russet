use std::error::Error;

#[derive(Debug)]
pub struct Config {
    from: String,
    to: String,
    format: String,
    input: String,
    output: String,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    from: Option<String>,
    to: Option<String>,
    format: Option<String>,
    input: Option<String>,
    output: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: Add error checking and empty string check
    pub fn from<S: Into<String>>(mut self, from: S) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to<S: Into<String>>(mut self, to: S) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn format<S: Into<String>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn input<S: Into<String>>(mut self, input: S) -> Self {
        self.input = Some(input.into());
        self
    }

    pub fn output<S: Into<String>>(mut self, output: S) -> Self {
        self.output = Some(output.into());
        self
    }

    pub fn build(self) -> Result<Config, Box<dyn Error>> {
        // TODO: Add support for user defaults
        //let conf = load_defaults()
        Ok(Config {
            from: self.from.ok_or("Missing From")?,
            to: self.to.ok_or("Missing To")?,
            format: self.format.ok_or("Missing Format")?,
            input: self.input.ok_or("Missing Input")?,
            output: self.output.ok_or("Missing Output")?,
        })
    }
}

use crossterm::style::{Attribute, Color, Stylize};

#[derive(Clone)]
pub struct StyledPart {
    pub text: String,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub styles: Vec<Attribute>,
    pub fill_bg: bool,
}

impl StyledPart {
    pub fn render(&self, default_fg: Option<Color>, default_bg: Option<Color>) -> String {
        let mut content = self.text.clone();

        if let Some(color) = self.fg.or(default_fg) {
            content = content.with(color).to_string();
        }

        if let Some(bg) = self.bg.or(default_bg) {
            content = content.on(bg).to_string();
        }

        for attr in &self.styles {
            content = apply_attr(content, *attr);
        }

        content
    }
}

fn apply_attr(text: String, attr: Attribute) -> String {
    match attr {
        Attribute::Bold => text.bold().to_string(),
        Attribute::Italic => text.italic().to_string(),
        Attribute::Underlined => text.underlined().to_string(),
        Attribute::CrossedOut => text.crossed_out().to_string(),
        Attribute::SlowBlink => text.slow_blink().to_string(),
        _ => text,
    }
}

pub struct OutputBuilder {
    parts: Vec<StyledPart>,
    default_fg: Option<Color>,
    default_bg: Option<Color>,
}

impl Default for OutputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputBuilder {
    pub fn new() -> Self {
        Self {
            parts: Vec::with_capacity(8),
            default_fg: None,
            default_bg: None,
        }
    }

    pub fn base(self) -> BaseStyleBuilder {
        BaseStyleBuilder { builder: self }
    }

    pub fn part(self, text: &str) -> StyledPartBuilder {
        StyledPartBuilder {
            parent: self,
            current: StyledPart {
                text: text.to_string(),
                fg: None,
                bg: None,
                styles: vec![],
                fill_bg: false,
            },
        }
    }

    pub fn get(mut self) -> String {
        // Estimate capacity: sum of part lengths; styled rendering may expand slightly
        let est: usize = self.parts.iter().map(|p| p.text.len()).sum();
        let mut output = String::with_capacity(est);
        for p in &self.parts {
            output.push_str(&p.render(self.default_fg, self.default_bg));
        }
        self.clear();
        output
    }

    pub fn copy(&self) -> String {
        let est: usize = self.parts.iter().map(|p| p.text.len()).sum();
        let mut output = String::with_capacity(est);
        for p in &self.parts {
            output.push_str(&p.render(self.default_fg, self.default_bg));
        }
        output
    }

    pub fn clear(&mut self) {
        self.parts.clear();
    }

    #[inline(always)]
    pub fn add_part(&mut self, part: StyledPart) {
        self.parts.push(part);
    }
}

pub struct BaseStyleBuilder {
    builder: OutputBuilder,
}

impl BaseStyleBuilder {
    #[inline(always)]
    pub fn color(mut self, color: Color) -> Self {
        self.builder.default_fg = Some(color);
        self
    }

    #[inline(always)]
    pub fn background(mut self, color: Color) -> Self {
        self.builder.default_bg = Some(color);
        self
    }

    #[inline(always)]
    pub fn done(self) -> OutputBuilder {
        self.builder
    }
}
pub struct StyledPartBuilder {
    parent: OutputBuilder,
    current: StyledPart,
}

impl StyledPartBuilder {
    #[inline(always)]
    pub fn new(parent: OutputBuilder) -> Self {
        StyledPartBuilder {
            parent,
            current: StyledPart {
                text: String::new(),
                fg: None,
                bg: None,
                styles: vec![],
                fill_bg: false,
            },
        }
    }

    #[inline(always)]
    pub fn part(self, text: &str) -> Self {
        let mut builder = self.parent;
        builder.add_part(self.current); // store last part
        StyledPartBuilder {
            parent: builder,
            current: StyledPart {
                text: text.to_string(),
                fg: None,
                bg: None,
                styles: vec![],
                fill_bg: false,
            },
        }
    }

    #[inline(always)]
    pub fn color(mut self, color: Color) -> Self {
        self.current.fg = Some(color);
        self
    }

    #[inline(always)]
    pub fn background(mut self, color: Color) -> Self {
        self.current.bg = Some(color);
        self
    }

    #[inline(always)]
    pub fn bold(mut self) -> Self {
        self.current.styles.push(Attribute::Bold);
        self
    }

    #[inline(always)]
    pub fn italic(mut self) -> Self {
        self.current.styles.push(Attribute::Italic);
        self
    }

    #[inline(always)]
    pub fn underline(mut self) -> Self {
        self.current.styles.push(Attribute::Underlined);
        self
    }

    #[inline(always)]
    pub fn strike(mut self) -> Self {
        self.current.styles.push(Attribute::CrossedOut);
        self
    }

    #[inline(always)]
    pub fn blink(mut self) -> Self {
        self.current.styles.push(Attribute::SlowBlink);
        self
    }

    #[inline(always)]
    pub fn space(mut self) -> Self {
        self.current.text.push(' ');
        self
    }

    #[inline(always)]
    pub fn none(mut self) -> Self {
        self.current.styles.clear();
        self
    }

    #[inline(always)]
    pub fn fill_bg(mut self) -> Self {
        self.current.fill_bg = true;
        self
    }

    #[inline(always)]
    pub fn end(self) -> OutputBuilder {
        let mut builder = self.parent;
        builder.add_part(self.current);
        builder
    }

    #[inline(always)]
    pub fn get(self) -> String {
        let builder = self.end();
        builder.get()
    }
}

#[inline(always)]
pub fn build() -> OutputBuilder {
    OutputBuilder::new()
}

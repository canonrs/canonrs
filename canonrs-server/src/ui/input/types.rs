/// Input HTML type attribute
#[derive(Clone, Copy, Default, PartialEq)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTime,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Number => "number",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Search => "search",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTime => "datetime-local",
        }
    }
}

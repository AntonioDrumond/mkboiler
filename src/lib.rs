pub mod c;
pub mod nix;
pub mod python;
pub mod rust;

pub enum IndentStyle {
    Tabs,
    Spaces(u32)
}

pub struct Config<'a> {
    pub target: &'a str,
    pub output: Option<&'a str>,
    pub indent: IndentStyle
}

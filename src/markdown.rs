pub trait Markdown {
    fn bold(&self) -> String;
    fn bullet(&self) -> String;
    fn code(&self) -> String;
    fn header(&self, level: u8) -> String;
    fn image(&self, path: &str) -> String;
    fn indent(&self, level: u8) -> String;
    fn italic(&self) -> String;
    fn link(&self, link: &str) -> String;
    fn quote(&self) -> String;
    fn strike(&self) -> String;
    fn task(&self, is_complete: bool) -> String;
}

impl<T: AsRef<str>> Markdown for T {
    fn bold(&self) -> String {
        decorate_text_with(self.as_ref(), "**")
    }

    fn bullet(&self) -> String {
        let text = self.as_ref();
        format!("- {text}")
    }

    fn code(&self) -> String {
        decorate_text_with(self.as_ref(), "`")
    }

    fn header(&self, level: u8) -> String {
        match level {
            0 => self.as_ref().to_string(),
            _ => {
                let header_string = "#".repeat(level.into());
                let text = self.as_ref();
                format!("{header_string} {text}")
            }
        }
    }

    fn image(&self, path: &str) -> String {
        let text = self.as_ref();
        format!("![{text}]({path})")
    }

    fn indent(&self, level: u8) -> String {
        match level {
            0 => self.as_ref().to_string(),
            _ => {
                let indent_string = "    ".repeat(level.into());
                let text = self.as_ref();
                format!("{indent_string}{text}")
            }
        }
    }

    fn italic(&self) -> String {
        decorate_text_with(self.as_ref(), "*")
    }

    fn link(&self, link: &str) -> String {
        let text = self.as_ref();
        format!("[{text}]({link})")
    }

    fn quote(&self) -> String {
        let text = self.as_ref();
        format!("> {text}")
    }

    fn strike(&self) -> String {
        decorate_text_with(self.as_ref(), "~~")
    }

    fn task(&self, is_complete: bool) -> String {
        let is_complete_symbol = if is_complete { "X" } else { " " };
        let text = self.as_ref();
        format!("- [{is_complete_symbol}] {text}")
    }
}

fn decorate_text_with<T: AsRef<str>>(text: T, decoration: &str) -> String {
    let text = text.as_ref();
    format!("{decoration}{text}{decoration}")
}

pub const DIVIDER: &str = "---";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_text() {
        let text = "Dog";
        assert_eq!(text.bold(), format!("**{text}**"))
    }

    #[test]
    fn bullet_text() {
        let text = "Dog";
        assert_eq!(text.bullet(), format!("- {text}"))
    }

    #[test]
    fn code_text() {
        let text = "Dog";
        assert_eq!(text.code(), format!("`{text}`"))
    }

    #[test]
    fn header_text_level_zero() {
        let text = "Dog";
        assert_eq!(text.header(0), text)
    }

    #[test]
    fn header_text_level_three() {
        let text = "Dog";
        assert_eq!(text.header(3), format!("### {text}"))
    }

    #[test]
    fn image_test() {
        let text = "Dog";
        let path = "/path/to/dog_image.png";
        assert_eq!(text.image(path), format!("![{text}]({path})"))
    }

    #[test]
    fn indent_text_level_zero() {
        let text = "Dog";
        assert_eq!(text.indent(0), text)
    }

    #[test]
    fn indent_text_level_three() {
        let text = "Dog";
        assert_eq!(text.indent(3), format!("            {text}"))
    }

    #[test]
    fn italicize_text() {
        let text = "Dog";
        assert_eq!(text.italic(), format!("*{text}*"))
    }

    #[test]
    fn link_text() {
        let text = "Apple";
        let link = "www.apple.com";
        assert_eq!(text.link(link), format!("[{text}]({link})"))
    }

    #[test]
    fn quote_text() {
        let text = "Not all those who wander are lost";
        assert_eq!(text.quote(), format!("> {text}"))
    }

    #[test]
    fn strike_text() {
        let text = "Dog";
        assert_eq!(text.strike(), format!("~~{text}~~"))
    }

    #[test]
    fn task_complete() {
        let text = "Finish refactor";
        assert_eq!(text.task(true), format!("- [X] {text}"))
    }

    #[test]
    fn task_not_complete() {
        let text = "Finish refactor";
        assert_eq!(text.task(false), format!("- [ ] {text}"))
    }

    #[test]
    fn multiple_decorations_1() {
        let text = "Begin building marko crate";
        let link = "https://github.com/JosephTLyons/marko";
        assert_eq!(
            text.bold().italic().link(link).task(true),
            format!("- [X] [***{text}***]({link})",)
        )
    }

    #[test]
    fn multiple_decorations_2() {
        let text = "The worst of times...";
        assert_eq!(text.italic().header(3), format!("### *{text}*"))
    }

    #[test]
    fn divider_test() {
        assert_eq!(DIVIDER, "---")
    }
}

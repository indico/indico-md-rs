use crate::LinkRule;

#[derive(Clone, Debug)]
pub struct MarkdownOptions {
    pub(crate) unstyled: bool,
    pub(crate) hardbreaks: bool,
    pub(crate) target_blank: bool,
    pub(crate) autolink_rules: Vec<LinkRule>,
}

impl MarkdownOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn unstyled(&mut self, unstyled: bool) -> &mut Self {
        self.unstyled = unstyled;
        self
    }

    pub fn hardbreaks(&mut self, hardbreaks: bool) -> &mut Self {
        self.hardbreaks = hardbreaks;
        self
    }

    pub fn target_blank(&mut self, target_blank: bool) -> &mut Self {
        self.target_blank = target_blank;
        self
    }

    pub fn autolink_rules(&mut self, autolink_rules: &[LinkRule]) -> &mut Self {
        self.autolink_rules = Vec::from(autolink_rules);
        self
    }
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            unstyled: false,
            hardbreaks: false,
            target_blank: true,
            autolink_rules: Vec::new(),
        }
    }
}

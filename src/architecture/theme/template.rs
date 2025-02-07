use crate::config::TemplateConfig;
use std::collections::HashMap;
use upon::Engine;

pub fn init_template_engine<'a>() -> Engine<'a> {
    let syntax = upon::Syntax::default();
    Engine::with_syntax(syntax)
}

pub struct TemplateManager<'a> {
    pub templates: &'a Option<HashMap<String, TemplateConfig>>,
    pub engine: Engine<'a>,
}

impl TemplateManager<'_> {
    pub fn new(templates: &Option<HashMap<String, TemplateConfig>>) -> TemplateManager {
        let engine = init_template_engine();
        TemplateManager { templates, engine }
    }
}

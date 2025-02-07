use crate::config::TemplateConfig;
use std::collections::HashMap;
use upon::Engine;

pub struct TemplateManager {
    pub templates: Option<HashMap<String, TemplateConfig>>,
    pub engine: Engine<'static>,
}

impl TemplateManager {
    pub fn new(templates: Option<HashMap<String, TemplateConfig>>) -> Self {
        let engine = init_template_engine();
        TemplateManager { templates, engine }
    }
}

pub fn init_template_engine() -> Engine<'static> {
    let syntax = upon::Syntax::default();
    Engine::with_syntax(syntax)
}

//! Tool manager — handles loading/unloading of feature-gated tools

#[allow(dead_code)]
pub struct ToolManager {
    enabled_tools: Vec<String>,
}

#[allow(dead_code)]
impl ToolManager {
    pub fn new() -> Self {
        Self {
            enabled_tools: Vec::new(),
        }
    }

    pub fn enable(&mut self, tool: &str) {
        if !self.enabled_tools.contains(&tool.to_string()) {
            self.enabled_tools.push(tool.to_string());
        }
    }

    pub fn disable(&mut self, tool: &str) {
        self.enabled_tools.retain(|t| t != tool);
    }

    pub fn is_enabled(&self, tool: &str) -> bool {
        self.enabled_tools.contains(&tool.to_string())
    }
}

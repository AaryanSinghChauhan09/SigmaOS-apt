#[derive(Debug, Clone, PartialEq)]
pub enum SourceType { Binary, Source }
#[derive(Debug, Clone)]
pub struct AptSource {
    pub url: String,
    pub suite: String,
    pub components: Vec<String>,
    pub source_type: SourceType,
    pub enabled: bool,
}
impl AptSource {
    pub fn new(url: &str, suite: &str, components: &[&str]) -> Self {
        Self {
            url: url.to_string(),
            suite: suite.to_string(),
            components: components.iter().map(|s| s.to_string()).collect(),
            source_type: SourceType::Binary,
            enabled: true,
        }
    }
    pub fn line(&self) -> String {
        format!("deb {} {} {}", self.url, self.suite, self.components.join(" "))
    }
}
pub struct SourceList { sources: Vec<AptSource> }
impl SourceList {
    pub fn new() -> Self { Self { sources: Vec::new() } }
    pub fn add(&mut self, source: AptSource) { self.sources.push(source); }
    pub fn enabled_count(&self) -> usize { self.sources.iter().filter(|s| s.enabled).count() }
    pub fn all(&self) -> &[AptSource] { &self.sources }
}
impl Default for SourceList { fn default() -> Self { Self::new() } }

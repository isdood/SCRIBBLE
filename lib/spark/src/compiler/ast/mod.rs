use super::forge::SafetyLevel;

#[derive(Debug)]
pub struct SparkNode {
    pub kind: NodeKind,
    pub safety: SafetyLevel,
}

#[derive(Debug)]
pub enum NodeKind {
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<SparkNode>,
    },
    // Add other node types as needed
}

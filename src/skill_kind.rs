#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SkillKind {
    Other,
    /// 未対応
    None,

    /// 変身
    /// 1: name(キャラ名)
    Transform(String),
}
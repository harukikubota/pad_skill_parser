use super::schema::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Skill {
    /// スキルの効果
    pub effect: SkillEffect,
    /// スキル発動前後の制約
    pub sub_effects: Option<SubEffect>,
    /// Nターンの間、XXする。ターン数を設定する。
    pub turns_of_apply: Option<usize>,
}

/// スキル効果
#[derive(Clone, Debug, PartialEq)]
pub enum SkillEffect {
    Other,
    /// N色のドロップを(単色|ランダムでN色)に変換する。
    /// 0: from
    /// 1: to
    ChangeDropAToB(Drops, Drops),
    /// 全ドロップを変化
    ChangeAllOfBoard(Drops),
    /// ランダム生成
    /// * 0: FromOtherDrops これに指定されているドロップ以外から生成する
    /// * 1: To 生成するドロップの種類と個数
    GenRandomDrop(Drops, GenDropsWithQty),
    /// ドロップリフレッシュ
    DropRefresh,
}

impl Default for SkillEffect {
  fn default() -> Self {
      Self::Other
  }
}

/// スキルの副次効果
/// 制約、条件による効果追加、スキル進化など
#[derive(Clone, Debug, PartialEq)]
pub enum SubEffect {
    /// バトル(以後|以前)の場合
    /// * pos_int 6  バトル6以降
    /// * neg_int -5 バトル5以前
    Floor(isize, SubEffectAttribute),

    /// HPがN%(以上|以下)
    /// * pos_int 80  80％以上
    /// * neg_int -50 50%以下
    HitPoint(isize, SubEffectAttribute),

    /// Nターン後に発動
    Reserve(usize),
}

/// 副次効果に付加される属性
#[derive(Clone, Debug, PartialEq)]
pub enum SubEffectAttribute {
    /// 条件を満たしている場合にのみ使用可能
    Available,
    /// 条件を満たしている場合に適用される
    IfApply,
}
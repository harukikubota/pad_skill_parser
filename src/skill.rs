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
    DropShapeGen(Vec<ShapeType>),
    // ロック解除
    DropUnLock,
    /// ドロップロック
    DropLock(Drops),
    /// ドロップリフレッシュ
    DropRefresh,
    /// ドロップ強化
    DropPowerUp(Drops),
    /// ドロップ目覚め
    DropFalloff(Drops, VolumeVariation),
    /// 強化ドロップ目覚め
    PowerupDropFalloff(PowerupDropFalloffKind),
    /// ロック目覚め
    FallLockDrop(Drops),
    /// 釘ドロップ目覚め
    FallNailDropEasierToFalloff(VolumeVariation),
    /// 落ちコンなし
    DropsNotFalling,
    /// ルーレット生成
    /// * 0: 個数
    GenRoulette(usize),
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

/// 指定型生成の生成位置を表す。
/// * col.pos: 左からN番目
/// * col.neg: 右からN番目
/// * row.pos: 上からN番目
/// * row.neg: 下からN番目
pub type ShapeGenIdx = isize;

/// 形状生成の種類
#[derive(Clone, Debug, PartialEq)]
pub enum ShapeType {
    /// 縦に生成
    Col(ShapeGenIdx, Drop),
    /// 横に生成
    Row(ShapeGenIdx, Drop),
    /// L字型で生成
    /// * 1: 生成数
    LShape(Drop, usize),
    /// Z字型で生成
    ZShape(Drop),
    /// 十字型(5個)で生成
    /// * 1: 生成数
    SmallCrossShape(Drop, usize),
    /// 十字型(10個)で生成
    CrossShape(Drop),
    /// 正方形で生成
    /// * 1: 1辺のサイズ（現在のところ3のみ）
    /// * 2: 生成数
    Square(Drop, usize, usize),
    /// 盤面外周で生成
    ShapeOfBoardPerimeter(Drop),
    /// 盤面上部で生成
    /// 現在では存在しないが、火ヨグソトースと同じように扱う
    /// * 1: 生成数
    ShapeOfBoardTop(Drop, usize),
    /// 盤面中央で生成
    ShapeOfBoardCenter(Drop),
    /// 盤面下部で生成
    /// * 1: 生成数
    ShapeOfBoardBottom(Drop, usize),
    /// 盤面4隅で生成
    /// * 1: ポイント毎の生成数
    ShapeOfBoardCorners(Drop, usize),
    /// 蜘蛛の巣状で生成
    /// 現状ではスパイダーマンのみ
    ShapeOfSpiderweb(Drop, Drop),
    /// 三日月状で生成
    ShapeOfCrescentMoon(Drop),
    /// 斜めで生成
    ShapeOfOblique(Drop),
    /// `XXX`の形で生成
    /// * 1: 形状
    /// * 2: 生成数
    ShapeOfSomeKind(Drop, String, usize),
}

impl ShapeType {
    /// 生成個数が必要ないバリアント用
    pub(super) fn set_drop(self: Self, drop: Drop) -> Self {
        match self {
            Self::ZShape(_) => Self::ZShape(drop),
            Self::CrossShape(_) => Self::CrossShape(drop),
            Self::ShapeOfBoardPerimeter(_) => Self::ShapeOfBoardPerimeter(drop),
            Self::ShapeOfBoardCenter(_) => Self::ShapeOfBoardCenter(drop),
            Self::ShapeOfCrescentMoon(_) => Self::ShapeOfCrescentMoon(drop),
            Self::ShapeOfOblique(_) => Self::ShapeOfOblique(drop),
            _ => panic!(
                "from ShapeType::set_drop(). This Variant require qty {:?}",
                self
            ),
        }
    }

    /// 生成個数が指定されているバリアント用
    pub(super) fn set_with_qty(self: Self, drop: Drop, qty: usize) -> Self {
        match self {
            Self::LShape(_, _) => Self::LShape(drop, qty),
            Self::ShapeOfBoardCorners(_, _) => Self::ShapeOfBoardCorners(drop, qty),
            // パース時点では盤面最大生成が指定されているので型を変更する
            Self::CrossShape(_) => Self::SmallCrossShape(drop, qty),
            Self::ShapeOfBoardTop(_, _) => Self::ShapeOfBoardTop(drop, qty),
            Self::ShapeOfBoardBottom(_, _) => Self::ShapeOfBoardBottom(drop, qty),
            _ => panic!(
                "from ShapeType::set_with_qty(). This Variant can't have qty {:?}",
                self
            ),
        }
    }

    /// 正方形用
    pub(super) fn set_for_square(self: Self, drop: Drop, size: usize, qty: usize) -> Self {
        Self::Square(drop, size, qty)
    }

    pub(super) fn is_square(self: &Self) -> bool {
        match self {
            Self::Square(_, _, _) => true,
            _ => false,
        }
    }

    /// `XXX`の形用
    pub(super) fn set_for_some_kind(self: Self, drop: Drop, some_kind: String, qty: usize) -> Self {
        Self::ShapeOfSomeKind(drop, some_kind, qty)
    }

    pub(super) fn is_some_kind(self: &Self) -> bool {
        match self {
            Self::ShapeOfSomeKind(_, _, _) => true,
            _ => false,
        }
    }

    pub(super) fn is_spiderweb(self: &Self) -> bool {
        match self {
            Self::ShapeOfSpiderweb(_, _) => true,
            _ => false,
        }
    }
}

impl From<&str> for ShapeType {
    fn from(item: &str) -> Self {
        if item == "L字型" {
            ShapeType::LShape(Drop::default(), 0)
        } else if item == "Z字型" {
            ShapeType::ZShape(Drop::default())
        } else if item == "十字型" {
            ShapeType::CrossShape(Drop::default())
        } else if item == "正方形" {
            ShapeType::Square(Drop::default(), 3, 0)
        } else if item == "盤面外周" {
            ShapeType::ShapeOfBoardPerimeter(Drop::default())
        } else if item == "盤面上部" {
            ShapeType::ShapeOfBoardTop(Drop::default(), 0)
        } else if item == "盤面中央" {
            ShapeType::ShapeOfBoardCenter(Drop::default())
        } else if item == "盤面下部" {
            ShapeType::ShapeOfBoardBottom(Drop::default(), 0)
        } else if item == "盤面4隅" {
            ShapeType::ShapeOfBoardCorners(Drop::default(), 0)
        } else if item == "の形" {
            ShapeType::ShapeOfSomeKind(Drop::default(), "Unknown".to_owned(), 0)
        } else if item == "蜘蛛の巣状" {
            ShapeType::ShapeOfSpiderweb(
                Drop::Colored(Color::Fire),
                Drop::NonColored(NonColoredDrop::Recovery),
            )
        } else if item == "三日月状" {
            ShapeType::ShapeOfCrescentMoon(Drop::default())
        } else if item == "斜め" {
            ShapeType::ShapeOfOblique(Drop::default())
        } else {
            panic!("from Color::from(&str). Expected ShapeType {item}");
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PowerupDropFalloffKind {
    /// N%
    Num(usize),
    /// 少し
    VolumeVariation(VolumeVariation),
}

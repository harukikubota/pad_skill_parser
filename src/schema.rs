/// ドロップリスト
pub type Drops = Vec<Drop>;

pub type GenDropsWithQty = Vec<(Drop, usize)>;

/// 何かのドロップを表す
#[derive(Clone, Debug, PartialEq)]
pub enum Drop {
    Colored(Color),
    NonColored(NonColoredDrop),
}

impl Drop {
    pub(super) fn default() -> Self {
        Self::Colored(Color::Fire)
    }
}

/// 何かの色を表す
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Color {
    #[default]
    Fire,
    Water,
    Wood,
    Lightning,
    Dark,
}

impl From<&str> for Color {
    fn from(item: &str) -> Self {
        if item == "火" {
            Color::Fire
        } else if item == "水" {
            Color::Water
        } else if item == "木" {
            Color::Wood
        } else if item == "光" {
            Color::Lightning
        } else if item == "闇" {
            Color::Dark
        } else {
            panic!("from Color::from(&str). Expected Color {item}");
        }
    }
}

/// 色を持たないドロップ
#[derive(Clone, Debug, Default, PartialEq)]
pub enum NonColoredDrop {
    #[default]
    Recovery,
    Disturb,
    Bomb,
    Poison,
    DeadlyPoison,
}

impl From<&str> for NonColoredDrop {
    fn from(item: &str) -> Self {
        if item == "回復" {
            NonColoredDrop::Recovery
        } else if item == "お邪魔" {
            NonColoredDrop::Disturb
        } else if item == "爆弾" {
            NonColoredDrop::Bomb
        } else if item == "毒" {
            NonColoredDrop::Poison
        } else if item == "猛毒" {
            NonColoredDrop::DeadlyPoison
        } else {
            panic!("from NonColoredDrop::from(&str). Expected Item {item}");
        }
    }
}

/// 位置を表す
#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    Left,
    Right,
    LeftAndRight,
    Top,
    Bottom,
}

impl From<&str> for Position {
    fn from(item: &str) -> Self {
        if item == "左" {
            Position::Left
        } else if item == "右" {
            Position::Right
        } else if item == "両" {
            Position::LeftAndRight
        } else if item == "上" {
            Position::Top
        } else if item == "下" {
            Position::Bottom
        } else {
            panic!("from Position::from(&str). Expected Position {item}");
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenShapeRowCol {
    Row(isize),
    Col(isize),
}

impl GenShapeRowCol {
    pub(super) fn idx(self: &Self) -> isize {
        match self {
            Self::Col(i) => i,
            Self::Row(i) => i,
        }
        .to_owned()
    }

    pub(super) fn update(self: &Self, new_idx: isize) -> Self {
        match self {
            Self::Col(_) => Self::Col(new_idx),
            Self::Row(_) => Self::Row(new_idx),
        }
    }
}

pub type GenPositions = Vec<GenShapeRowCol>;

/// スキルの効果がどの程度及ぼすかの変化量
#[derive(Clone, Debug, PartialEq)]
pub enum VolumeVariation {
    // ほんの少し
    LittleMore,
    // 少し
    Little,
    // 普通
    Normal,
    // のみ
    Only,
}

/// row * col
#[derive(Clone, Debug, PartialEq)]
pub struct Size(pub usize, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum BoardPosition {
    Row(isize),
    Col(isize),
    Random,
}

impl From<Position> for BoardPosition {
    fn from(value: Position) -> Self {
        match value {
            Position::Left => Self::Col(1),
            Position::Right => Self::Col(-1),
            Position::Top => Self::Row(1),
            Position::Bottom => Self::Row(-1),
            _ => panic!("両端は未対応"),
        }
    }
}

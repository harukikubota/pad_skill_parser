/// ドロップリスト
pub type Drops = Vec<Drop>;

/// 何かのドロップを表す
#[derive(Clone, Debug, PartialEq)]
pub enum Drop {
    Colored(Color),
    NonColored(NonColoredDrop),
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

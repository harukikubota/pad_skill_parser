use crate::schema::*;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum StackItem {
    Color(Color),
    Drop(Drop),
    Drops(Drops),
    PosInt(usize),
}

#[allow(dead_code)]
impl StackItem {
    pub(super) fn color(self: Self) -> Color {
        match self {
            Self::Color(elem) => elem,
            _ => panic!("from StackItem::pop(). this Item isn't Color!"),
        }
    }

    pub(super) fn drop(self: Self) -> Drop {
        match self {
            Self::Drop(elem) => elem,
            _ => panic!("from StackItem::pop(). this Item isn't Drop!"),
        }
    }

    pub(super) fn drops(self: Self) -> Drops {
        match self {
            Self::Drops(elem) => elem,
            _ => panic!("from StackItem::pop(). this Item isn't Drops!"),
        }
    }

    pub(super) fn pos_int(self: Self) -> usize {
        match self {
            Self::PosInt(elem) => elem,
            _ => panic!("from StackItem::pop(). this Item isn't PosInt!"),
        }
    }

    pub(super) fn is_color(self: &Self) -> bool {
        match self {
            Self::Color(_) => true,
            _ => false,
        }
    }

    pub(super) fn is_drop(self: &Self) -> bool {
        match self {
            Self::Drop(_) => true,
            _ => false,
        }
    }

    pub(super) fn is_drops(self: &Self) -> bool {
        match self {
            Self::Drops(_) => true,
            _ => false,
        }
    }

    pub(super) fn is_pos_int(self: &Self) -> bool {
        match self {
            Self::PosInt(_) => true,
            _ => false,
        }
    }
}

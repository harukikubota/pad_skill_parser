use super::{schema::*, skill::*};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum StackItem {
    Color(Color),
    Drop(Drop),
    Drops(Drops),
    PosInt(usize),
    Position(Position),
    GenPositions(GenPositions),
    DropShapeGenShapeType(ShapeType),
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

    pub(super) fn position(self: Self) -> Position {
        match self {
            Self::Position(elem) => elem,
            _ => panic!("from StackItem::pop(). this Item isn't Position!"),
        }
    }

    pub(super) fn gen_positions(self: Self) -> GenPositions {
        match self {
            Self::GenPositions(elem) => elem,
            _ => panic!("from StackItem::pop(). this Item isn't GenPositions!"),
        }
    }

    pub(super) fn shape_type(self: Self) -> ShapeType {
        match self {
            Self::DropShapeGenShapeType(elem) => elem,
            _ => panic!("from StackItem::pop(). this Item isn't ShapeType!"),
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

    pub(super) fn is_position(self: &Self) -> bool {
        match self {
            Self::Position(_) => true,
            _ => false,
        }
    }

    pub(super) fn is_gen_positions(self: &Self) -> bool {
        match self {
            Self::GenPositions(_) => true,
            _ => false,
        }
    }

    pub(super) fn is_shape_type(self: &Self) -> bool {
        match self {
            Self::DropShapeGenShapeType(_) => true,
            _ => false,
        }
    }
}

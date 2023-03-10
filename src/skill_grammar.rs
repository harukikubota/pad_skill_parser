use std::marker::PhantomData;
use std::ops::Neg;

use super::schema::*;
use super::skill::*;
use super::skill_grammar_trait::SkillGrammarTrait;
use super::stack_item::*;

#[derive(Debug, Default, PartialEq)]
pub struct SkillGrammar<'t> {
    pub skill_list: Vec<Skill>,
    stack: Vec<StackItem>,
    tmp: TmpItem,
    pd: PhantomData<&'t str>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub(super) enum TmpItem {
    #[default]
    None,
    Stack(StackItem),
    Skill(Skill),
    SkillEffect(SkillEffect),
}

impl<'t> SkillGrammar<'_> {
    pub(super) fn pop(self: &mut Self) -> StackItem {
        self.stack
            .pop()
            .expect("from SkillGrammar::pop. stack is 0.")
    }

    pub(super) fn pop_if<F>(self: &mut Self, fun: F) -> Option<StackItem>
    where
        F: FnOnce(&StackItem) -> bool,
    {
        let last = self.pop();

        if fun(&last) {
            Some(last)
        } else {
            self.push(last);
            None
        }
    }

    pub(super) fn peek_check<F>(self: &mut Self, fun: F) -> bool
    where
        F: FnOnce(&StackItem) -> bool,
    {
        let last = self.stack.pop();

        if last.clone().is_none() {
            false
        } else {
            self.push(last.clone().unwrap());
            fun(&last.unwrap())
        }
    }

    pub(super) fn is_zero(self: &mut Self) -> bool {
        self.stack.len() == 0
    }

    pub(super) fn push(self: &mut Self, item: StackItem) {
        self.stack.push(item);
    }

    pub(super) fn get_tmp(self: &mut Self) -> Option<TmpItem> {
        let tmp = match self.tmp.clone() {
            TmpItem::None => None,
            TmpItem::Stack(item) => Some(TmpItem::Stack(item)),
            TmpItem::Skill(item) => Some(TmpItem::Skill(item)),
            TmpItem::SkillEffect(item) => Some(TmpItem::SkillEffect(item)),
        };

        tmp.map(|item| {
            self.tmp = TmpItem::None;
            item
        })
    }

    #[allow(dead_code)]
    pub(super) fn get_tmp_for_stack_item(self: &mut Self) -> StackItem {
        let item = self.get_tmp().unwrap();

        match item {
            TmpItem::Stack(item) => item,
            _ => panic!("from TmpItem::get_tmp(). this Item isn't StackItem!"),
        }
    }

    #[allow(dead_code)]
    pub(super) fn get_tmp_for_skill(self: &mut Self) -> Skill {
        let item = self.get_tmp().unwrap();

        match item {
            TmpItem::Skill(item) => item,
            _ => panic!("from TmpItem::get_tmp(). this Item isn't Skill!"),
        }
    }

    #[allow(dead_code)]
    pub(super) fn get_tmp_for_skill_effect(self: &mut Self) -> SkillEffect {
        let item = self.get_tmp().unwrap();

        match item {
            TmpItem::SkillEffect(item) => item,
            _ => panic!("from TmpItem::get_tmp(). this Item isn't SkillEffect!"),
        }
    }

    pub(super) fn set_tmp(self: &mut Self, tmp: TmpItem) {
        self.tmp = tmp;
    }
}

impl SkillGrammar<'_> {
    pub fn new() -> Self {
        SkillGrammar {
            skill_list: Vec::new(),
            stack: Vec::new(),
            tmp: TmpItem::default(),
            pd: PhantomData::default(),
        }
    }

    #[allow(dead_code)]
    fn show_stack(self: &Self) {
        self.show_stack_("")
    }

    #[allow(dead_code)]
    fn show_stack_<'a>(self: &Self, from: &'a str) {
        print!("---------------- START ----------------\n");
        print!("from: {from}\n");
        self.stack
            .to_vec()
            .into_iter()
            .enumerate()
            .rev()
            .for_each(|(idx, item)| {
                print!("{idx}  | {:?}\n", item);
            });
        print!("tmp| {:?}\n", self.tmp);
        print!("----------------  END  ----------------\n");
    }
}

/// Helper for SkillGrammarTraitImpl.
impl SkillGrammar<'_> {
    /// stack ??????????????????????????????????????????
    #[allow(dead_code)]
    fn steal<'a>(self: &'a mut Self) -> Vec<StackItem> {
        let ret = self.stack.to_vec();

        self.stack.clear();
        ret
    }

    /// stack ?????????????????????????????????????????????????????????
    #[allow(dead_code)]
    fn steal_rev<'a>(self: &'a mut Self) -> Vec<StackItem> {
        let mut ret = self.steal();
        ret.reverse();
        ret
    }

    /// ???????????????????????????????????????????????????????????????????????????????????????????????????
    fn steal_if<'a, F, M, R>(self: &'a mut Self, type_check_fun: F, map_fun: M) -> Vec<R>
    where
        F: Fn(&StackItem) -> bool,
        M: Fn(&StackItem) -> R,
        R: Clone,
    {
        let ret: &mut Vec<R> = &mut Vec::new();

        let mut result = self.steal_if_(ret, type_check_fun, map_fun).to_vec();
        result.reverse();
        result
    }

    fn steal_if_<'a, F, M, R>(
        self: &'a mut Self,
        list: &'a mut Vec<R>,
        type_check_fun: F,
        map_fun: M,
    ) -> &'a mut Vec<R>
    where
        F: Fn(&StackItem) -> bool,
        M: Fn(&StackItem) -> R,
    {
        if self.is_zero() {
            // ??????????????????
            list
        } else {
            let last = self.pop();

            if type_check_fun(&last) {
                list.push(map_fun(&last));
                self.steal_if_(list, type_check_fun, map_fun)
            } else {
                self.push(last);
                list
            }
        }
    }

    /// ???????????????
    fn build_change_drop_a_to_b<'a>(self: &'a mut Self, a: Drops, b: Drops) -> Skill {
        Skill {
            effect: SkillEffect::ChangeDropAToB(a, b),
            ..Default::default()
        }
    }

    /// ?????????????????????????????????????????????
    fn build_gen_drop_and_qty_list(to: Drops, qty: usize) -> GenDropsWithQty {
        to.into_iter().map(|drop| (drop, qty)).collect()
    }

    /// ??????????????????????????????????????????????????????????????????????????????
    /// * specified: ????????????????????????????????????????????????????????????????????????
    /// * to: D1, D2, ...Dx ????????????????????????
    /// * ?????????4?????????: ?????????????????????????????????
    /// * ?????????5?????????: ???????????????????????????
    /// * ????????????30  : ??????
    fn build_gen_random_drop_exc_from(specified: &mut Drops, to: &mut GenDropsWithQty) -> Drops {
        let to_drops: &mut Drops = &mut to.into_iter().map(|e| e.clone().0).collect();
        let gen_drop_sum: usize = to.into_iter().map(|e| e.1).sum();

        print!("sum:{}\n", gen_drop_sum);

        if dbg!((to_drops.to_vec().len() < 5) && (gen_drop_sum != 30)) {
            // 4?????????????????? & ????????????30????????????
            specified.append(to_drops);
        }
        specified.to_vec()
    }

    fn push_gen_drop_and_qty_list<'a>(
        self: &'a mut Self,
        from: Drops,
        gen_drop_and_qty_list: GenDropsWithQty,
    ) {
        let se = SkillEffect::GenRandomDrop(from, gen_drop_and_qty_list);

        let skill = Skill {
            effect: se,
            ..Default::default()
        };

        self.skill_list.push(skill);
    }

    fn g_s_s_p_side_<'a>(self: &'a mut Self, gen_count: usize, position: Position) -> GenPositions {
        let mut result: GenPositions = Vec::new();

        // N??????????????????
        for idx in 1..=gen_count {
            match position {
                Position::Left => result.push(GenShapeRowCol::Col(idx as isize)),
                Position::Right => result.push(GenShapeRowCol::Col((idx as isize).neg())),
                Position::LeftAndRight => {
                    result.push(GenShapeRowCol::Col(idx as isize));
                    result.push(GenShapeRowCol::Col((idx as isize).neg()))
                }
                Position::Top => result.push(GenShapeRowCol::Row(idx as isize)),
                Position::Bottom => result.push(GenShapeRowCol::Row((idx as isize).neg())),
            }
        }

        result
    }

    /// 5???????????????????????????????????????????????????
    fn all_drops_10() -> Drops {
        vec![
            Drop::Colored(Color::Fire),
            Drop::Colored(Color::Water),
            Drop::Colored(Color::Wood),
            Drop::Colored(Color::Lightning),
            Drop::Colored(Color::Dark),
            Drop::NonColored(NonColoredDrop::Recovery),
            Drop::NonColored(NonColoredDrop::Disturb),
            Drop::NonColored(NonColoredDrop::Bomb),
            Drop::NonColored(NonColoredDrop::Poison),
            Drop::NonColored(NonColoredDrop::DeadlyPoison),
        ]
    }
}

impl<'t> SkillGrammarTrait<'t> for SkillGrammar<'t> {
    fn starts_with_drop_line(
        &mut self,
        _arg: &crate::skill_grammar_trait::StartsWithDropLine<'t>,
    ) -> miette::Result<()> {
        let item = self.pop();

        if item.is_drop() {
            // ????????????
            let to = item.drop();
            let from = self.pop().drops();

            let skill = self.build_change_drop_a_to_b(from, vec![to]);
            self.skill_list.push(skill);

            // ????????????????????????????????????????????????????????????????????????
            if let Some(maybe_skill) = self.get_tmp() {
                match maybe_skill {
                    TmpItem::Skill(skill) => {
                        self.skill_list.push(skill);
                    }
                    other => self.set_tmp(other),
                }
            }
        } else if item.is_drops() {
            // ?????????????????????????????????
            let from = self.pop().drops();

            let skill = self.build_change_drop_a_to_b(from, item.drops());
            self.skill_list.push(skill);
        } else if item.is_gen_drops_with_qty() {
            let mut list: Vec<GenDropsWithQty> = self.steal_if(
                |e| e.is_gen_drops_with_qty(),
                |e| e.clone().gen_drop_with_qty(),
            );

            list.push(item.gen_drop_with_qty());

            let first: &mut GenDropsWithQty = &mut Vec::new();
            let exc: &mut Drops = &mut Vec::new();
            let qty_or_drops = self.pop();

            if qty_or_drops.is_pos_int() {
                let qty = qty_or_drops.pos_int();
                let drops = self.pop().drops();

                first.append(&mut Self::build_gen_drop_and_qty_list(drops, qty))
            } else {
                exc.append(&mut qty_or_drops.drops());
            };

            first.append(&mut list.concat());

            let exc_from_drops = Self::build_gen_random_drop_exc_from(exc, first);

            self.push_gen_drop_and_qty_list(exc_from_drops, first.to_owned());
        } else if item.is_drop_powerup() {
            let drops = self.pop().drops();

            let skill = Skill {
                effect: SkillEffect::DropPowerUp(drops),
                ..Default::default()
            };

            self.skill_list.push(skill);
        } else {
            // ??????????????????
            let qty = item.pos_int();
            let drops = self.pop().drops();

            let to = &mut Self::build_gen_drop_and_qty_list(drops, qty);

            let exc: &mut Drops = &mut if !self.is_zero() {
                self.pop().drops()
            } else {
                vec![]
            };

            let exc_from_drops = Self::build_gen_random_drop_exc_from(exc, to);

            self.push_gen_drop_and_qty_list(exc_from_drops, to.to_owned());
        }
        Ok(())
    }

    fn change_drop_with_drop_unlock_line(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeDropWithDropUnlockLine<'t>,
    ) -> miette::Result<()> {
        // ???????????????????????????????????????????????????????????????????????????????????????????????????0?????????
        if !self.is_zero() {
            let item = self.pop();

            if item.is_drops() {
                // N????????????
                let drops = item.drops();

                let skill = Skill {
                    effect: SkillEffect::ChangeAllOfBoard(drops),
                    ..Default::default()
                };

                self.skill_list.push(skill);
            } else if item.is_drop_powerup() {
                // ?????????????????????
                let drops = vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Wood),
                    Drop::Colored(Color::Lightning),
                    Drop::Colored(Color::Dark),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ];

                let skill = Skill {
                    effect: SkillEffect::DropPowerUp(drops),
                    ..Default::default()
                };

                self.skill_list.push(skill);
            } else {
                // ????????????????????????
                let gen_drops_with_qty = item.gen_drop_with_qty();
                let drops = self.pop().drops();

                let skill = Skill {
                    effect: SkillEffect::ChangeAllOfBoard(drops),
                    ..Default::default()
                };

                self.skill_list.push(skill);
                self.push_gen_drop_and_qty_list(vec![], gen_drops_with_qty);
            }
        }
        Ok(())
    }

    fn drop_un_lock_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::DropUnLockStmt<'t>,
    ) -> miette::Result<()> {
        // Stack??????????????????
        let _ = self.pop();
        let skill = Skill {
            effect: SkillEffect::DropUnLock,
            ..Default::default()
        };

        self.skill_list.push(skill);
        Ok(())
    }

    fn drop_refresh_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::DropRefreshStmt<'t>,
    ) -> miette::Result<()> {
        let skill = Skill {
            effect: SkillEffect::DropRefresh,
            ..Default::default()
        };

        self.skill_list.push(skill);
        Ok(())
    }

    fn towards_the_enemy_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::TowardsTheEnemyStmt<'t>,
    ) -> miette::Result<()> {
        let se = self.pop().apply_in_turns_skill();
        let skill = Skill {
            effect: se,
            turns_of_apply: Some(999),
            ..Default::default()
        };
        self.skill_list.push(skill);

        Ok(())
    }

    fn change_enemy_attribute_block(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeEnemyAttributeBlock<'t>,
    ) -> miette::Result<()> {
        let se = SkillEffect::EnemyAttributeChange(self.pop().color());
        self.push(StackItem::ApplyInTurnsSkill(se));

        Ok(())
    }

    fn turns_of_apply_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::TurnsOfApplyStmt<'t>,
    ) -> miette::Result<()> {
        // ????????????????????????????????????Some
        let qty = self.pop_if(|i| i.is_pos_int()).map(|i| i.pos_int());

        let se_list = self.steal_if(
            |i| i.is_apply_in_turns_skill(),
            |i| i.clone().apply_in_turns_skill(),
        );

        let turn = self.pop().pos_int();

        se_list.into_iter().for_each(|se| {
            let se = match se {
                SkillEffect::GenRoulette(_) => SkillEffect::GenRoulette(qty.unwrap()),
                other => other,
            };

            let skill = Skill {
                turns_of_apply: Some(turn),
                effect: se,
                ..Default::default()
            };

            self.skill_list.push(skill);
        });

        Ok(())
    }

    /// A ???????????? ??? B ???????????? ??? 2????????????????????????????????????
    /// ?????????????????????3?????????????????????????????????????????????????????????
    fn change_drop_block_other_first(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeDropBlockOtherFirst<'t>,
    ) -> miette::Result<()> {
        let to = self.pop().drop();
        let mut from = self.pop().drops();

        // 0??????????????????1??????????????????????????????????????????????????????????????????
        from.reverse();
        let for_before_group_drop = from.pop().unwrap();
        self.push(StackItem::Drop(for_before_group_drop));
        from.reverse();

        let skill = self.build_change_drop_a_to_b(from, vec![to]);
        self.set_tmp(TmpItem::Skill(skill));
        Ok(())
    }

    fn gen_random_drop_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::GenRandomDropStmt<'t>,
    ) -> miette::Result<()> {
        let list = &mut self
            .steal_if(
                |e| e.is_gen_drops_with_qty(),
                |e| e.clone().gen_drop_with_qty(),
            )
            .concat();

        let exc_from_drops = Self::build_gen_random_drop_exc_from(&mut vec![], list);

        let se = SkillEffect::GenRandomDrop(exc_from_drops, list.to_owned());

        let skill = Skill {
            effect: se,
            ..Default::default()
        };
        self.skill_list.push(skill);
        Ok(())
    }

    fn gen_random_drop_block(
        &mut self,
        _arg: &crate::skill_grammar_trait::GenRandomDropBlock<'t>,
    ) -> miette::Result<()> {
        let gen_quantity = self.pop().pos_int();
        let gen_drops = self.pop().drops();

        let gen_drop_qty_list: &mut GenDropsWithQty =
            &mut Self::build_gen_drop_and_qty_list(gen_drops, gen_quantity);

        self.push(StackItem::GenDropsWithQty(gen_drop_qty_list.to_owned()));

        Ok(())
    }

    fn gen_shape_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::GenShapeStmt<'t>,
    ) -> miette::Result<()> {
        let shape_type_list = self.steal();

        let drop_shape_gen_list: Vec<ShapeType> = shape_type_list
            .into_iter()
            .map(|se| se.shape_type())
            .collect();

        let se = SkillEffect::DropShapeGen(drop_shape_gen_list);

        let skill = Skill {
            effect: se,
            ..Default::default()
        };
        self.skill_list.push(skill);
        Ok(())
    }

    fn gen_shape_block_row_col(
        &mut self,
        _arg: &crate::skill_grammar_trait::GenShapeBlockRowCol<'t>,
    ) -> miette::Result<()> {
        let drop = self.pop().drop();
        let positions = self.pop().gen_positions();

        positions.into_iter().for_each(|p| {
            let shape_type = match p {
                GenShapeRowCol::Row(idx) => ShapeType::Row(idx, drop.clone()),
                GenShapeRowCol::Col(idx) => ShapeType::Col(idx, drop.clone()),
            };

            self.push(StackItem::DropShapeGenShapeType(shape_type));
        });
        Ok(())
    }

    /// ?????????????????????????????????
    fn gen_shape_block_other_row_col(
        &mut self,
        _arg: &crate::skill_grammar_trait::GenShapeBlockOtherRowCol<'t>,
    ) -> miette::Result<()> {
        self.show_stack_("gen_shape_block_other_row_col");
        if self.stack.len() == 2 {
            // ???????????????????????????
            let drop = self.pop().drops().pop().unwrap();
            let shape = self.pop().shape_type();

            if shape.is_spiderweb() {
                // ????????????????????????????????????????????????????????????????????????
                self.push(StackItem::DropShapeGenShapeType(shape))
            } else {
                self.push(StackItem::DropShapeGenShapeType(shape.set_drop(drop)));
            }
        } else {
            let qty = self.pop().pos_int();
            let drop = self.pop().drops().pop().unwrap();
            let shape = self.pop().shape_type();

            let shape = if shape.is_square() {
                // 1??????????????????????????????????????????????????????
                let size = self.pop().size().0;

                shape.set_for_square(drop, size, qty)
            } else if shape.is_some_kind() {
                // ????????????`7`??????????????????????????????
                // UnsignedInt ????????????????????????????????????????????????????????????

                let some_kind = self.pop().pos_int().to_string();
                shape.set_for_some_kind(drop, some_kind, qty)
            } else {
                shape.set_with_qty(drop, qty)
            };

            self.push(StackItem::DropShapeGenShapeType(shape));
        }
        Ok(())
    }

    /// ???N????????????
    fn g_s_s_p_side(
        &mut self,
        _arg: &crate::skill_grammar_trait::GSSPSide<'t>,
    ) -> miette::Result<()> {
        let mut gen_positions_list: Vec<GenPositions> = Vec::new();

        while self.peek_check(|i| i.is_pos_int()) {
            let gen_count = self.pop().pos_int();
            let position = self.pop().position();

            gen_positions_list.push(self.g_s_s_p_side_(gen_count, position));
        }

        // ???????????????????????????????????????????????????????????????????????????
        gen_positions_list.reverse();
        let gen_positions = gen_positions_list.concat();

        self.push(StackItem::GenPositions(gen_positions));
        Ok(())
    }

    fn g_s_s_p_center(
        &mut self,
        _arg: &crate::skill_grammar_trait::GSSPCenter<'t>,
    ) -> miette::Result<()> {
        let gen_count = self.pop().pos_int();
        let gen_positions = self.pop().gen_positions();

        let mut new_gen_positions: GenPositions = Vec::new();

        gen_positions.into_iter().for_each(|gp| {
            // ????????????????????????????????????
            for idx in 0..gen_count {
                let base_idx = gp.idx();

                let new_idx = if base_idx.is_positive() {
                    base_idx + (idx as isize)
                } else {
                    base_idx - (idx as isize)
                };

                new_gen_positions.push(gp.update(new_idx));
            }
        });

        self.push(StackItem::GenPositions(new_gen_positions));
        Ok(())
    }

    fn g_s_s_p_center_blocks(
        &mut self,
        _arg: &crate::skill_grammar_trait::GSSPCenterBlocks<'t>,
    ) -> miette::Result<()> {
        if self.stack.len() > 1 {
            // 2????????????????????????1??????????????????????????????
            let list: GenPositions = self
                .steal_if(|i| i.is_gen_positions(), |i| i.to_owned().gen_positions())
                .concat();

            self.push(StackItem::GenPositions(list));
        }
        Ok(())
    }

    fn g_s_s_p_center_block(
        &mut self,
        _arg: &crate::skill_grammar_trait::GSSPCenterBlock<'t>,
    ) -> miette::Result<()> {
        let gen_idx = self.pop().pos_int();
        let position = self.pop().position();

        let item = match position {
            Position::Left => GenShapeRowCol::Col(gen_idx as isize),
            Position::Right => GenShapeRowCol::Col((gen_idx as isize).neg()),
            Position::Top => GenShapeRowCol::Row(gen_idx as isize),
            Position::Bottom => GenShapeRowCol::Row((gen_idx as isize).neg()),
            _ => panic!("from g_s_s_p_center_block. unexpected pattern!"),
        };

        self.push(StackItem::GenPositions(vec![item]));
        Ok(())
    }

    fn drop_unlock_block(
        &mut self,
        _arg: &crate::skill_grammar_trait::DropUnlockBlock<'t>,
    ) -> miette::Result<()> {
        // Stack??????????????????
        let _ = self.pop();
        let skill = Skill {
            effect: SkillEffect::DropUnLock,
            ..Default::default()
        };

        self.skill_list.push(skill);
        Ok(())
    }

    /// N?????????????????????????????????????????????????????????????????????????????????????????????
    fn five_attribute(
        &mut self,
        _arg: &crate::skill_grammar_trait::FiveAttribute<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let colors = vec![
            Color::Fire,
            Color::Water,
            Color::Wood,
            Color::Lightning,
            Color::Dark,
        ];

        colors.into_iter().for_each(|color| {
            self.push(StackItem::Drop(Drop::Colored(color)));
        });
        Ok(())
    }

    fn drops(
        &mut self,
        _arg: &crate::skill_grammar_trait::Drops<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drops: Drops = self
            .steal_if(|i| i.is_drop(), |i| i.clone().drop())
            .to_vec();

        self.push(StackItem::Drops(drops));
        Ok(())
    }

    fn drop(
        &mut self,
        _arg: &crate::skill_grammar_trait::Drop<'t>,
    ) -> parol_runtime::miette::Result<()> {
        // ?????????????????????????????????????????????????????????????????????
        let drop = self
            .pop_if(|i| i.is_color())
            .map_or_else(|| self.pop().drop(), |item| Drop::Colored(item.color()));

        self.push(StackItem::Drop(drop));
        Ok(())
    }

    /// ??????????????????
    fn recovery(
        &mut self,
        arg: &crate::skill_grammar_trait::Recovery<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.recovery.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// ?????????????????????
    fn disturb(
        &mut self,
        arg: &crate::skill_grammar_trait::Disturb<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.disturb.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// ??????????????????
    fn bomb(
        &mut self,
        arg: &crate::skill_grammar_trait::Bomb<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.bomb.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// ???????????????
    fn poison(
        &mut self,
        arg: &crate::skill_grammar_trait::Poison<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.poison.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// ??????????????????
    fn deadly_poison(
        &mut self,
        arg: &crate::skill_grammar_trait::DeadlyPoison<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.deadly_poison.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    fn fire(
        &mut self,
        arg: &crate::skill_grammar_trait::Fire<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.fire.text());

        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn water(
        &mut self,
        arg: &crate::skill_grammar_trait::Water<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.water.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn wood(
        &mut self,
        arg: &crate::skill_grammar_trait::Wood<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.wood.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn lightning(
        &mut self,
        arg: &crate::skill_grammar_trait::Lightning<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.lightning.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn dark(
        &mut self,
        arg: &crate::skill_grammar_trait::Dark<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.dark.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn shape_of_l(&mut self, arg: &crate::skill_grammar_trait::ShapeOfL<'t>) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_l.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_z(&mut self, arg: &crate::skill_grammar_trait::ShapeOfZ<'t>) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_z.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    /// ???????????????????????????????????????????????????????????????????????????
    /// ????????????????????????????????????????????????????????????????????????
    fn shape_of_cross(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfCross<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_cross.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_square(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfSquare<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_square.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_board_perimeter(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfBoardPerimeter<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_board_perimeter.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_board_corners(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfBoardCorners<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_board_corners.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_board_top(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfBoardTop<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_board_top.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_board_center(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfBoardCenter<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_board_center.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_board_bottom(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfBoardBottom<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_board_bottom.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_spiderweb(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfSpiderweb<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_spiderweb.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_crescent_moon(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfCrescentMoon<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_crescent_moon.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_oblique(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfOblique<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_oblique.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    fn shape_of_some_kind(
        &mut self,
        arg: &crate::skill_grammar_trait::ShapeOfSomeKind<'t>,
    ) -> miette::Result<()> {
        let shape = ShapeType::from(arg.shape_of_some_kind.text());
        self.stack.push(StackItem::DropShapeGenShapeType(shape));
        Ok(())
    }

    /// ???????????????????????????????????????
    /// ????????????????????????????????????
    /// ???????????????????????????????????????????????????
    fn drops_easier_to_falloff_or_fall_lock_drop(
        &mut self,
        _arg: &crate::skill_grammar_trait::DropsEasierToFalloffOrFallLockDrop<'t>,
    ) -> miette::Result<()> {
        if self.pop_if(|i| i.is_drop_lock()).is_some() {
            // ??????????????????
            let drops = self.pop().drops();
            let se = SkillEffect::FallLockDrop(drops);

            self.push(StackItem::ApplyInTurnsSkill(se));
        } else {
            let volume = self
                .pop_if(|i| i.is_volume_variation())
                .map_or_else(|| VolumeVariation::Normal, |i| i.volume_variation());

            // ?????????????????????????????????????????????Some
            let powerup_drop = self.pop_if(|i| i.is_drop_powerup());

            let drops = self.pop().drops();

            let effect = SkillEffect::DropFalloff(drops, volume.clone());

            self.push(StackItem::ApplyInTurnsSkill(effect));

            if powerup_drop.is_some() {
                let effect = SkillEffect::PowerupDropFalloff(
                    PowerupDropFalloffKind::VolumeVariation(volume),
                );
                self.push(StackItem::ApplyInTurnsSkill(effect));
            }
        }
        Ok(())
    }

    /// ???????????????????????????
    fn powerup_drops_easier_to_falloff(
        &mut self,
        _arg: &crate::skill_grammar_trait::PowerupDropsEasierToFalloff<'t>,
    ) -> miette::Result<()> {
        // ????????????????????????????????????????????????Some
        let mut drop_falloff: Option<Drops> = None;

        let kind = if let Some(percent) = self.pop_if(|i| i.is_pos_int()) {
            let _ = self.pop(); // WordPowerup

            PowerupDropFalloffKind::Num(percent.pos_int())
        } else {
            let _ = self.pop(); // WordLittle
            drop_falloff = self.pop_if(|i| i.is_drops()).map(|i| i.drops());
            let _ = self.pop(); // WordPowerup

            PowerupDropFalloffKind::VolumeVariation(VolumeVariation::Little)
        };

        self.push(StackItem::ApplyInTurnsSkill(
            SkillEffect::PowerupDropFalloff(kind),
        ));

        if let Some(drops) = drop_falloff {
            // ?????????????????????`??????`??????
            let se = SkillEffect::DropFalloff(drops, VolumeVariation::Little);
            self.push(StackItem::ApplyInTurnsSkill(se));
        }

        self.show_stack();
        Ok(())
    }

    fn fall_lock_drop_of_all(
        &mut self,
        _arg: &crate::skill_grammar_trait::FallLockDropOfAll<'t>,
    ) -> miette::Result<()> {
        let _ = self.pop(); // WordLock
        let se = SkillEffect::FallLockDrop(Self::all_drops_10());

        self.push(StackItem::ApplyInTurnsSkill(se));
        Ok(())
    }

    fn fall_nail_drop_easier_to_falloff(
        &mut self,
        _arg: &crate::skill_grammar_trait::FallNailDropEasierToFalloff<'t>,
    ) -> miette::Result<()> {
        let volume = self
            .pop_if(|i| i.is_volume_variation())
            .map_or_else(|| VolumeVariation::Normal, |i| i.volume_variation());

        let se = SkillEffect::FallNailDropEasierToFalloff(volume);
        self.push(StackItem::ApplyInTurnsSkill(se));
        Ok(())
    }

    fn board_change(
        &mut self,
        _arg: &crate::skill_grammar_trait::BoardChange<'t>,
    ) -> miette::Result<()> {
        let se = self.pop().apply_in_turns_skill();
        let size = self.pop_if(|i| i.is_size()).map(|i| i.size());

        if size.is_none() {
            let _ = self.pop();
        } // PosInt N???

        let position = self.pop_if(|i| i.is_position()).map_or_else(
            || BoardPosition::Random,
            |i| BoardPosition::from(i.position()),
        );

        let se = match se {
            SkillEffect::GenCloud(_, _) => SkillEffect::GenCloud(position, size.unwrap()),
            SkillEffect::GenTeap(_) => SkillEffect::GenTeap(position),
            SkillEffect::ChangeBoardSize(_) => SkillEffect::ChangeBoardSize(size.unwrap()),
            _ => todo!(),
        };

        self.push(StackItem::ApplyInTurnsSkill(se));
        Ok(())
    }

    fn size(&mut self, _arg: &crate::skill_grammar_trait::Size<'t>) -> miette::Result<()> {
        let col = self.pop().pos_int();
        let row = self.pop().pos_int();

        self.push(StackItem::Size(Size(row, col)));

        Ok(())
    }

    fn nullification_damage_absorption(
        &mut self,
        _arg: &crate::skill_grammar_trait::NullificationDamageAbsorption<'t>,
    ) -> miette::Result<()> {
        self.push(StackItem::ApplyInTurnsSkill(
            SkillEffect::NullificationDamageAbsorption,
        ));
        Ok(())
    }

    fn nullification_attribute_absorption(
        &mut self,
        _arg: &crate::skill_grammar_trait::NullificationAttributeAbsorption<'t>,
    ) -> miette::Result<()> {
        self.push(StackItem::ApplyInTurnsSkill(
            SkillEffect::NullificationAttributeAbsorption,
        ));
        Ok(())
    }

    fn penetration_damage_nullification(
        &mut self,
        _arg: &crate::skill_grammar_trait::PenetrationDamageNullification<'t>,
    ) -> miette::Result<()> {
        self.push(StackItem::ApplyInTurnsSkill(
            SkillEffect::PenetrationDamageNullification,
        ));
        Ok(())
    }

    fn word_left(&mut self, arg: &crate::skill_grammar_trait::WordLeft<'t>) -> miette::Result<()> {
        let position = Position::from(arg.word_left.text());

        self.stack.push(StackItem::Position(position));
        Ok(())
    }

    fn word_right(
        &mut self,
        arg: &crate::skill_grammar_trait::WordRight<'t>,
    ) -> miette::Result<()> {
        let position = Position::from(arg.word_right.text());

        self.stack.push(StackItem::Position(position));
        Ok(())
    }

    fn word_left_and_right(
        &mut self,
        arg: &crate::skill_grammar_trait::WordLeftAndRight<'t>,
    ) -> miette::Result<()> {
        let position = Position::from(arg.word_left_and_right.text());

        self.stack.push(StackItem::Position(position));
        Ok(())
    }

    fn word_top(&mut self, arg: &crate::skill_grammar_trait::WordTop<'t>) -> miette::Result<()> {
        let position = Position::from(arg.word_top.text());

        self.stack.push(StackItem::Position(position));
        Ok(())
    }

    fn word_bottom(
        &mut self,
        arg: &crate::skill_grammar_trait::WordBottom<'t>,
    ) -> miette::Result<()> {
        let position = Position::from(arg.word_bottom.text());

        self.stack.push(StackItem::Position(position));
        Ok(())
    }

    fn word_lock(&mut self, _arg: &crate::skill_grammar_trait::WordLock<'t>) -> miette::Result<()> {
        self.stack.push(StackItem::DropLock);
        Ok(())
    }

    fn word_power_up(
        &mut self,
        _arg: &crate::skill_grammar_trait::WordPowerUp<'t>,
    ) -> miette::Result<()> {
        self.stack.push(StackItem::DropPowerUp);
        Ok(())
    }

    fn word_little_more(
        &mut self,
        _arg: &crate::skill_grammar_trait::WordLittleMore<'t>,
    ) -> miette::Result<()> {
        self.stack
            .push(StackItem::VolumeVariation(VolumeVariation::LittleMore));
        Ok(())
    }

    fn word_little(
        &mut self,
        _arg: &crate::skill_grammar_trait::WordLittle<'t>,
    ) -> miette::Result<()> {
        self.stack
            .push(StackItem::VolumeVariation(VolumeVariation::Little));
        Ok(())
    }

    fn word_only(&mut self, _arg: &crate::skill_grammar_trait::WordOnly<'t>) -> miette::Result<()> {
        self.stack
            .push(StackItem::VolumeVariation(VolumeVariation::Only));
        Ok(())
    }

    fn word_not_falling(
        &mut self,
        _arg: &crate::skill_grammar_trait::WordNotFalling<'t>,
    ) -> miette::Result<()> {
        self.stack
            .push(StackItem::ApplyInTurnsSkill(SkillEffect::DropsNotFalling));
        Ok(())
    }

    fn word_roulette(
        &mut self,
        _arg: &crate::skill_grammar_trait::WordRoulette<'t>,
    ) -> miette::Result<()> {
        self.stack
            .push(StackItem::ApplyInTurnsSkill(SkillEffect::GenRoulette(0)));
        Ok(())
    }

    fn word_cloud(
        &mut self,
        _arg: &crate::skill_grammar_trait::WordCloud<'t>,
    ) -> miette::Result<()> {
        self.stack
            .push(StackItem::ApplyInTurnsSkill(SkillEffect::GenCloud(
                BoardPosition::Random,
                Size(0, 0),
            )));
        Ok(())
    }

    fn word_cant_be_operated(
        &mut self,
        _arg: &crate::skill_grammar_trait::WordCantBeOperated<'t>,
    ) -> miette::Result<()> {
        self.stack
            .push(StackItem::ApplyInTurnsSkill(SkillEffect::GenTeap(
                BoardPosition::Random,
            )));
        Ok(())
    }

    /// ????????????????????????????????????????????????????????????`SkillEffect::ChangeBoardSize`????????????????????????
    fn word_mass(&mut self, _arg: &crate::skill_grammar_trait::WordMass<'t>) -> miette::Result<()> {
        self.stack
            .push(StackItem::ApplyInTurnsSkill(SkillEffect::ChangeBoardSize(
                Size(0, 0),
            )));
        Ok(())
    }

    fn pos_int(&mut self, arg: &crate::skill_grammar_trait::PosInt<'t>) -> miette::Result<()> {
        let text = arg.pos_int.text();
        let num = text.parse::<usize>().unwrap();

        self.push(StackItem::PosInt(num));

        Ok(())
    }

    fn on_board(&mut self, _arg: &crate::skill_grammar_trait::OnBoard<'t>) -> miette::Result<()> {
        // ??????????????????????????????????????????????????????????????????????????????
        let _ = self.pop();

        Ok(())
    }
}

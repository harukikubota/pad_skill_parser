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
    /// stack の中身を全て取得し、空にする
    #[allow(dead_code)]
    fn steal<'a>(self: &'a mut Self) -> Vec<StackItem> {
        let ret = self.stack.to_vec();

        self.stack.clear();
        ret
    }

    /// stack の中身を逆順にして全て取得し、空にする
    #[allow(dead_code)]
    fn steal_rev<'a>(self: &'a mut Self) -> Vec<StackItem> {
        let mut ret = self.steal();
        ret.reverse();
        ret
    }

    /// 条件に合う要素を終端から取得し、取得した要素はスタックから取り除く
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
            // 全て取得した
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

    /// 単色変換用
    fn build_change_drop_a_to_b<'a>(self: &'a mut Self, a: Drops, b: Drops) -> Skill {
        Skill {
            effect: SkillEffect::ChangeDropAToB(a, b),
            ..Default::default()
        }
    }

    /// ドロップ全てに生成数を付与する
    fn build_gen_drop_and_qty_list(to: Drops, qty: usize) -> GenDropsWithQty {
        to.into_iter().map(|drop| (drop, qty)).collect()
    }

    /// ランダム生成の生成元から除外するドロップリストを返す
    /// * specified: ○ドロップ以外からで指定されているドロップリスト
    /// * to: D1, D2, ...Dx 生成するドロップ
    /// * 生成が4色以下: 指定している色＋生成色
    /// * 生成が5色以上: 指定している色のみ
    /// * 生成数が30  : なし
    fn build_gen_random_drop_exc_from(specified: &mut Drops, to: &mut GenDropsWithQty) -> Drops {
        let to_drops: &mut Drops = &mut to.into_iter().map(|e| e.clone().0).collect();
        let gen_drop_sum: usize = to.into_iter().map(|e| e.1).sum();

        print!("sum:{}\n", gen_drop_sum);

        if dbg!((to_drops.to_vec().len() < 5) && (gen_drop_sum != 30)) {
            // 4色以下の生成 & 生成数が30ではない
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

        // N列分繰り返す
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

    /// 5属性＋回復、お邪魔、毒、猛毒、爆弾
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
            // 単色変換
            let to = item.drop();
            let from = self.pop().drops();

            let skill = self.build_change_drop_a_to_b(from, vec![to]);
            self.skill_list.push(skill);

            // ２色目の変換があるならスキルリストへプッシュする
            if let Some(maybe_skill) = self.get_tmp() {
                match maybe_skill {
                    TmpItem::Skill(skill) => {
                        self.skill_list.push(skill);
                    }
                    other => self.set_tmp(other),
                }
            }
        } else if item.is_drops() {
            // ランダムで複数色に変換
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
            // ランダム生成
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
        // 指定型生成はすでにスキルリストにプッシュされているため、スタックは0となる
        if !self.is_zero() {
            let item = self.pop();

            if item.is_drops() {
                // N色陣のみ
                let drops = item.drops();

                let skill = Skill {
                    effect: SkillEffect::ChangeAllOfBoard(drops),
                    ..Default::default()
                };

                self.skill_list.push(skill);
            } else if item.is_drop_powerup() {
                // 全ドロップ強化
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
                // 陣→ランダム生成
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
        // Stackから削除する
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

    fn turns_of_apply_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::TurnsOfApplyStmt<'t>,
    ) -> miette::Result<()> {
        let se_list = self.steal_if(
            |i| i.is_apply_in_turns_skill(),
            |i| i.clone().apply_in_turns_skill(),
        );

        let turn = self.pop().pos_int();

        se_list.into_iter().for_each(|se| {
            let skill = Skill {
                turns_of_apply: Some(turn),
                effect: se,
                ..Default::default()
            };

            self.skill_list.push(skill);
        });

        Ok(())
    }

    /// A ドロップ を B ドロップ に 2回目の出現時のみ呼ばれる
    /// 現在のところ、3色目の変換は別の行に別れるため問題なし
    fn change_drop_block_other_first(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeDropBlockOtherFirst<'t>,
    ) -> miette::Result<()> {
        let to = self.pop().drop();
        let mut from = self.pop().drops();

        // 0要素目は変換1色目で扱うドロップのため、スタックに積み直す
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

    /// 縦、横以外の指定型生成
    fn gen_shape_block_other_row_col(
        &mut self,
        _arg: &crate::skill_grammar_trait::GenShapeBlockOtherRowCol<'t>,
    ) -> miette::Result<()> {
        self.show_stack_("gen_shape_block_other_row_col");
        if self.stack.len() == 2 {
            // 形状とドロップのみ
            let drop = self.pop().drops().pop().unwrap();
            let shape = self.pop().shape_type();

            if shape.is_spiderweb() {
                // 蜘蛛の巣状はデフォルト値で設定しているので未処理
                self.push(StackItem::DropShapeGenShapeType(shape))
            } else {
                self.push(StackItem::DropShapeGenShapeType(shape.set_drop(drop)));
            }
        } else {
            let qty = self.pop().pos_int();
            let drop = self.pop().drops().pop().unwrap();
            let shape = self.pop().shape_type();

            let shape = if shape.is_square() {
                // 1辺だけ分かればいいので、片方は捨てる
                let size = self.pop().pos_int();
                let _ = self.pop();

                shape.set_for_square(drop, size, qty)
            } else if shape.is_some_kind() {
                // 現在では`7`の形のみなので通る。
                // UnsignedInt 以外でスキルが追加された場合は対応が必要

                let some_kind = self.pop().pos_int().to_string();
                shape.set_for_some_kind(drop, some_kind, qty)
            } else {
                shape.set_with_qty(drop, qty)
            };

            self.push(StackItem::DropShapeGenShapeType(shape));
        }
        Ok(())
    }

    /// 端N列の生成
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

        // 右側から解決されているため、左からになるようにする
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
            // 生成する列数だけ繰り返す
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
            // 2要素以上の場合、1つのリストにまとめる
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
        // Stackから削除する
        let _ = self.pop();
        let skill = Skill {
            effect: SkillEffect::DropUnLock,
            ..Default::default()
        };

        self.skill_list.push(skill);
        Ok(())
    }

    /// N色陣、ランダム生成でのみ使用する文言なのでドロップまで生成する
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
        // 色ならドロップに変換、色なしドロップはそのまま
        let drop = self
            .pop_if(|i| i.is_color())
            .map_or_else(|| self.pop().drop(), |item| Drop::Colored(item.color()));

        self.push(StackItem::Drop(drop));
        Ok(())
    }

    /// 回復ドロップ
    fn recovery(
        &mut self,
        arg: &crate::skill_grammar_trait::Recovery<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.recovery.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// お邪魔ドロップ
    fn disturb(
        &mut self,
        arg: &crate::skill_grammar_trait::Disturb<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.disturb.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// 爆弾ドロップ
    fn bomb(
        &mut self,
        arg: &crate::skill_grammar_trait::Bomb<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.bomb.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// 毒ドロップ
    fn poison(
        &mut self,
        arg: &crate::skill_grammar_trait::Poison<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.poison.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// 猛毒ドロップ
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

    /// デフォルトでは盤面一杯に生成する十字型として扱う。
    /// 個数が指定されている場合に小型にキャストされる。
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

    /// ○ドロップが落ちやすくなる
    /// ○ドロップのみ落ちてくる
    /// ○ドロップがロック状態で落ちてくる
    fn drops_easier_to_falloff_or_fall_lock_drop(
        &mut self,
        _arg: &crate::skill_grammar_trait::DropsEasierToFalloffOrFallLockDrop<'t>,
    ) -> miette::Result<()> {
        if self.pop_if(|i| i.is_drop_lock()).is_some() {
            // ロック目覚め
            let drops = self.pop().drops();
            let se = SkillEffect::FallLockDrop(drops);

            self.push(StackItem::ApplyInTurnsSkill(se));
        } else {
            let volume = self
                .pop_if(|i| i.is_volume_variation())
                .map_or_else(|| VolumeVariation::Normal, |i| i.volume_variation());

            // 強化ドロップが複合しているならSome
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

    /// 強化ドロップ目覚め
    fn powerup_drops_easier_to_falloff(
        &mut self,
        _arg: &crate::skill_grammar_trait::PowerupDropsEasierToFalloff<'t>,
    ) -> miette::Result<()> {
        // ドロップ目覚めが複合しているならSome
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
            // 複合する場合は`少し`のみ
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

    fn pos_int(&mut self, arg: &crate::skill_grammar_trait::PosInt<'t>) -> miette::Result<()> {
        let text = arg.pos_int.text();
        let num = text.parse::<usize>().unwrap();

        self.push(StackItem::PosInt(num));

        Ok(())
    }

    fn on_board(&mut self, _arg: &crate::skill_grammar_trait::OnBoard<'t>) -> miette::Result<()> {
        // パターンを引っ掛けたいだけなのでスタックから削除する
        let _ = self.pop();

        Ok(())
    }
}

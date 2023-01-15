use std::marker::PhantomData;

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
        self.show_stack_("");
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

    /// stack から StackItem::Drop を再起的に取得する
    fn drops_<'a>(self: &'a mut Self, result: &'a mut Drops) -> &'a mut Drops {
        // 全部Drop
        if self.stack.len() == 0 {
            result
        } else {
            let last = self.pop();

            match last {
                StackItem::Drop(drop) => {
                    result.push(drop);
                    self.drops_(result)
                }
                _ => {
                    self.push(last);
                    result
                }
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
    fn build_gen_random_drop_exc_from(specified: &mut Drops, to: &mut Drops) -> Drops {
        if to.to_vec().len() < 5 {
            // 4色以下の生成
            specified.append(to);
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
}

impl<'t> SkillGrammarTrait<'t> for SkillGrammar<'t> {
    fn change_drop_stmt_inc_gen_random_drop(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeDropStmtIncGenRandomDrop<'t>,
    ) -> parol_runtime::miette::Result<()> {
        self.show_stack_("change_drop_stmt_inc_gen_random_drop s");

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
        } else {
            // ランダム生成
            let qty = item.pos_int();
            let drops = self.pop().drops();

            let exc: &mut Drops = &mut if !self.is_zero() {
                self.pop().drops()
            } else {
                vec![]
            };

            let exc_from_drops = Self::build_gen_random_drop_exc_from(exc, &mut drops.to_vec());

            let to = Self::build_gen_drop_and_qty_list(drops, qty);

            self.push_gen_drop_and_qty_list(exc_from_drops, to);
        }
        Ok(())
    }

    fn change_all_of_borad_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeAllOfBoradStmt<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drops = self.pop().drops();

        let skill = Skill {
            effect: SkillEffect::ChangeAllOfBoard(drops),
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

    fn gen_random_drop_stmt1(
        &mut self,
        _arg: &crate::skill_grammar_trait::GenRandomDropStmt1<'t>,
    ) -> miette::Result<()> {
        let gen_quantity = self.pop().pos_int();
        let gen_drops = self.pop().drops();

        let gen_drop_qty_list: GenDropsWithQty = gen_drops
            .to_vec()
            .into_iter()
            .map(|drop| (drop, gen_quantity))
            .collect();

        let se = SkillEffect::GenRandomDrop(gen_drops, gen_drop_qty_list);

        let skill = Skill {
            effect: se,
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
        let mut drops = self.drops_(&mut Vec::new()).to_vec();
        drops.reverse();

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

    fn pos_int(&mut self, arg: &crate::skill_grammar_trait::PosInt<'t>) -> miette::Result<()> {
        let text = arg.pos_int.text();
        let num = text.parse::<usize>().unwrap();

        self.push(StackItem::PosInt(num));

        Ok(())
    }
}

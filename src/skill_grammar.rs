use std::marker::PhantomData;

use super::schema::*;
use super::skill_grammar_trait::SkillGrammarTrait;
use super::stack_item::*;

#[derive(Debug, Default, PartialEq)]
pub struct SkillGrammar<'t> {
    pub skill_list: Vec<Skill>,
    stack: Vec<StackItem>,
    pd: PhantomData<&'t str>,
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

    pub(super) fn push(self: &mut Self, item: StackItem) {
        self.stack.push(item);
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Skill {
    /// スキル発動前後の制約
    pub sub_effects: Option<SubEffect>,
    /// Nターンの間、XXする。ターン数を設定する。
    pub turns_of_apply: Option<usize>,
    /// スキルの効果
    pub effect: SkillEffect,
}

impl Default for SkillEffect {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(Debug, PartialEq)]
/// スキルの副次効果
/// 制約、条件による効果追加、スキル進化など
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
#[derive(Debug, PartialEq)]
pub enum SubEffectAttribute {
    /// 条件を満たしている場合にのみ使用可能
    Available,
    /// 条件を満たしている場合に適用される
    IfApply,
}

#[derive(Debug, PartialEq)]
/// スキル効果
pub enum SkillEffect {
    Other,
    /// N色のドロップを単色に変換する。
    /// 0: from
    /// 1: to
    ChangeDropAToB(Drops, Drop),
}

impl SkillGrammar<'_> {
    pub fn new() -> Self {
        SkillGrammar {
            skill_list: Vec::new(),
            stack: Vec::new(),
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
                print!("{idx}| {:?}\n", item);
            });
        print!("----------------  END  ----------------\n");
    }
}

/// Helper for SkillGrammarTraitImpl.
impl SkillGrammar<'_> {
    /// stack から StackItem::Drop を再起的に取得する
    fn drops_<'a>(self: &'a mut Self, result: &'a mut Vec<Drop>) -> &'a mut Vec<Drop> {
        self.show_stack_("drops_ start");

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
                _ => result,
            }
        }
    }
}

impl<'t> SkillGrammarTrait<'t> for SkillGrammar<'t> {
    /// たぶんいらない
    fn change_drop_stmt(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeDropStmt<'t>,
    ) -> parol_runtime::miette::Result<()> {
        Ok(())
    }

    // A ドロップ を B ドロップ に
    fn change_drop_block(
        &mut self,
        _arg: &crate::skill_grammar_trait::ChangeDropBlock<'t>,
    ) -> parol_runtime::miette::Result<()> {
        self.show_stack_("change_drop_block");
        let to = self.pop().drop();
        let from = self.pop().drops();

        let se = SkillEffect::ChangeDropAToB(from, to);

        let skill = Skill {
            effect: se,
            ..Default::default()
        };

        self.skill_list.push(skill);
        Ok(())
    }

    fn drops(
        &mut self,
        _arg: &crate::skill_grammar_trait::Drops<'t>,
    ) -> parol_runtime::miette::Result<()> {
        self.show_stack_("drops start");

        let reversed = self.drops_(&mut Vec::new()).to_vec();

        let drops: Vec<Drop> = reversed.into_iter().rev().collect();

        self.push(StackItem::Drops(drops));

        self.show_stack_("drops end");
        Ok(())
    }

    fn drop(
        &mut self,
        _arg: &crate::skill_grammar_trait::Drop<'t>,
    ) -> parol_runtime::miette::Result<()> {
        self.show_stack_("drop");

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
        let drop = NonColoredDrop::from(arg.clone().recovery.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// お邪魔ドロップ
    fn disturb(
        &mut self,
        arg: &crate::skill_grammar_trait::Disturb<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.clone().disturb.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// 爆弾ドロップ
    fn bomb(
        &mut self,
        arg: &crate::skill_grammar_trait::Bomb<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.clone().bomb.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// 毒ドロップ
    fn poison(
        &mut self,
        arg: &crate::skill_grammar_trait::Poison<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.clone().poison.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    /// 猛毒ドロップ
    fn deadly_poison(
        &mut self,
        arg: &crate::skill_grammar_trait::DeadlyPoison<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = NonColoredDrop::from(arg.clone().deadly_poison.text());
        self.stack.push(StackItem::Drop(Drop::NonColored(drop)));
        Ok(())
    }

    fn fire(
        &mut self,
        arg: &crate::skill_grammar_trait::Fire<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.clone().fire.text());

        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn water(
        &mut self,
        arg: &crate::skill_grammar_trait::Water<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.clone().water.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn wood(
        &mut self,
        arg: &crate::skill_grammar_trait::Wood<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.clone().wood.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn lightning(
        &mut self,
        arg: &crate::skill_grammar_trait::Lightning<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.clone().lightning.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }

    fn dark(
        &mut self,
        arg: &crate::skill_grammar_trait::Dark<'t>,
    ) -> parol_runtime::miette::Result<()> {
        let drop = Color::from(arg.clone().dark.text());
        self.stack.push(StackItem::Color(drop));
        Ok(())
    }
}

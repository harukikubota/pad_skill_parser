#[cfg(test)]
mod parser_test {
    use pad_skill_parser::schema::*;
    use pad_skill_parser::skill_grammar::*;
    use pad_skill_parser::skill_parser::parse;

    const FILE_NAME: &str = "parser_test.rs";

    fn new<'t>(s: Vec<Skill>) -> SkillGrammar<'t> {
        let mut g = SkillGrammar::new();
        g.skill_list = s;
        g
    }

    #[test]
    fn change_drop_a_to_b_1() {
        let input = "木ドロップを水ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeDropAToB(
                vec![Drop::Colored(Color::Wood)],
                Drop::Colored(Color::Water),
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_drop_a_to_b_2() {
        let input = "回復を水に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeDropAToB(
                vec![Drop::NonColored(NonColoredDrop::Recovery)],
                Drop::Colored(Color::Water),
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_drop_many_to_b() {
        let input = "お邪魔、爆弾、毒、猛毒を光に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeDropAToB(
                vec![
                    Drop::NonColored(NonColoredDrop::Disturb),
                    Drop::NonColored(NonColoredDrop::Bomb),
                    Drop::NonColored(NonColoredDrop::Poison),
                    Drop::NonColored(NonColoredDrop::DeadlyPoison),
                ],
                Drop::Colored(Color::Lightning),
            ),
        }]);

        assert_eq!(except, grammar);
    }
}

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

    #[test]
    fn change_drop_a_to_b_and_c_to_d() {
        let input = "火を水に、光を回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except =
            &mut new(vec![
                Skill {
                    sub_effects: None,
                    turns_of_apply: None,
                    effect: SkillEffect::ChangeDropAToB(
                        vec![Drop::Colored(Color::Fire),],
                        Drop::Colored(Color::Water),
                    ),
                },
                Skill {
                    sub_effects: None,
                    turns_of_apply: None,
                    effect: SkillEffect::ChangeDropAToB(
                        vec![Drop::Colored(Color::Lightning),],
                        Drop::NonColored(NonColoredDrop::Recovery),
                    ),
                }
            ]);

        assert_eq!(except, grammar);
    }

    /*
    //#[test]
    fn change_all_of_board_five_att() {
        let input = "全ドロップを5属性に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Wood),
                    Drop::Colored(Color::Lightning),
                    Drop::Colored(Color::Dark),
                ]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn change_all_of_board_six_and_other_att() {
        let input = "全ドロップを5属性ドロップ+回復+毒+猛毒に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Wood),
                    Drop::Colored(Color::Lightning),
                    Drop::Colored(Color::Dark),
                    Drop::NonColored(NonColoredDrop::Recovery),
                    Drop::NonColored(NonColoredDrop::Poison),
                    Drop::NonColored(NonColoredDrop::DeadlyPoison),
                ]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn change_all_of_board_four_att() {
        let input = "全ドロップを火、水、木、回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Wood),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn change_all_of_board_two_att() {
        let input = "全ドロップを火と光に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Lightning),
                ]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn change_all_of_board_one_att() {
        let input = "全ドロップを回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(
                vec![
                    Drop::NonColored(NonColoredDrop::Recovery),
                ]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn gen_random_drop_color_of_1() {
        let input = "ランダムで火ドロップを1個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![Drop::Colored(Color::Fire)],
                vec![(Drop::Colored(Color::Fire), 1)]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn gen_random_drop_color_of_2() {
        let input = "ランダムで火と水を2個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                ],
                vec![
                    (Drop::Colored(Color::Fire), 2),
                    (Drop::Colored(Color::Water), 2),
                ]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn gen_random_drop_atori() {
        let input = "5属性+回復を1個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                ],
                vec![
                    (Drop::Colored(Color::Fire), 2),
                    (Drop::Colored(Color::Water), 2),
                ]
            ),
        }]);

        assert_eq!(except, grammar);
    }

    */
    /*
    #[test]
    fn drop_refresh() {
        let input = "ランダムでドロップを入れ替える。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropRefresh,
        }]);

        assert_eq!(except, grammar);
    }
    */
}

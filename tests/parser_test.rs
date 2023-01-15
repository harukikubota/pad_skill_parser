#[cfg(test)]
mod parser_test {
    use pad_skill_parser::schema::*;
    use pad_skill_parser::skill::*;
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
                vec![Drop::Colored(Color::Water)],
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
                vec![Drop::Colored(Color::Water)],
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
                vec![Drop::Colored(Color::Lightning)],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_drop_a_to_b_and_c_to_d() {
        let input = "火を水に、光を回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::ChangeDropAToB(
                    vec![Drop::Colored(Color::Fire)],
                    vec![Drop::Colored(Color::Water)],
                ),
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::ChangeDropAToB(
                    vec![Drop::Colored(Color::Lightning)],
                    vec![Drop::NonColored(NonColoredDrop::Recovery)],
                ),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_all_of_board_five_att() {
        let input = "全ドロップを5属性に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(vec![
                Drop::Colored(Color::Fire),
                Drop::Colored(Color::Water),
                Drop::Colored(Color::Wood),
                Drop::Colored(Color::Lightning),
                Drop::Colored(Color::Dark),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_all_of_board_six_and_other_att() {
        let input = "全ドロップを5属性ドロップ+回復+毒+猛毒に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(vec![
                Drop::Colored(Color::Fire),
                Drop::Colored(Color::Water),
                Drop::Colored(Color::Wood),
                Drop::Colored(Color::Lightning),
                Drop::Colored(Color::Dark),
                Drop::NonColored(NonColoredDrop::Recovery),
                Drop::NonColored(NonColoredDrop::Poison),
                Drop::NonColored(NonColoredDrop::DeadlyPoison),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_all_of_board_four_att() {
        let input = "全ドロップを火、水、木、回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(vec![
                Drop::Colored(Color::Fire),
                Drop::Colored(Color::Water),
                Drop::Colored(Color::Wood),
                Drop::NonColored(NonColoredDrop::Recovery),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_all_of_board_two_att() {
        let input = "全ドロップを火と光に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(vec![
                Drop::Colored(Color::Fire),
                Drop::Colored(Color::Lightning),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_all_of_board_one_att() {
        let input = "全ドロップを回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeAllOfBoard(vec![Drop::NonColored(NonColoredDrop::Recovery)]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_color_of_1() {
        let input = "ランダムで火ドロップを1個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![Drop::Colored(Color::Fire)],
                vec![(Drop::Colored(Color::Fire), 1)],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_color_of_2() {
        let input = "ランダムで火と水を2個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![Drop::Colored(Color::Fire), Drop::Colored(Color::Water)],
                vec![
                    (Drop::Colored(Color::Fire), 2),
                    (Drop::Colored(Color::Water), 2),
                ],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_atori() {
        let input = "5属性+回復を4個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![],
                vec![
                    (Drop::Colored(Color::Fire), 4),
                    (Drop::Colored(Color::Water), 4),
                    (Drop::Colored(Color::Wood), 4),
                    (Drop::Colored(Color::Lightning), 4),
                    (Drop::Colored(Color::Dark), 4),
                    (Drop::NonColored(NonColoredDrop::Recovery), 4),
                ],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_color_1_exc_1() {
        let input = "回復ドロップ以外から火ドロップを6個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![
                    Drop::NonColored(NonColoredDrop::Recovery),
                    Drop::Colored(Color::Fire),
                ],
                vec![(Drop::Colored(Color::Fire), 6)],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_color_2_exc_2() {
        let input = "火とお邪魔以外から水と木を2個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::NonColored(NonColoredDrop::Disturb),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Wood),
                ],
                vec![
                    (Drop::Colored(Color::Water), 2),
                    (Drop::Colored(Color::Wood), 2),
                ],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_leftside_1() {
        let input = "左端1列を光ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Col(
                1,
                Drop::Colored(Color::Lightning),
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_rightside_1() {
        let input = "右端1列を回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Col(
                -1,
                Drop::NonColored(NonColoredDrop::Recovery),
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_left_and_rightside_1() {
        let input = "両端1列を闇ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(1, Drop::Colored(Color::Dark)),
                ShapeType::Col(-1, Drop::Colored(Color::Dark)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_left_1_right_1() {
        let input = "左端1列を闇ドロップに、右端1列を光ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(1, Drop::Colored(Color::Dark)),
                ShapeType::Col(-1, Drop::Colored(Color::Lightning)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_leftside_2() {
        let input = "左端縦2列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(1, Drop::Colored(Color::Wood)),
                ShapeType::Col(2, Drop::Colored(Color::Wood)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_rightside_2() {
        let input = "右端縦2列をお邪魔に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(-1, Drop::NonColored(NonColoredDrop::Disturb)),
                ShapeType::Col(-2, Drop::NonColored(NonColoredDrop::Disturb)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_left_and_rightside_2() {
        let input = "両端縦2列を火ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(1, Drop::Colored(Color::Fire)),
                ShapeType::Col(-1, Drop::Colored(Color::Fire)),
                ShapeType::Col(2, Drop::Colored(Color::Fire)),
                ShapeType::Col(-2, Drop::Colored(Color::Fire)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_left_and_rightside_2_split() {
        let input = "左端2列と右端2列を水ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(1, Drop::Colored(Color::Water)),
                ShapeType::Col(2, Drop::Colored(Color::Water)),
                ShapeType::Col(-1, Drop::Colored(Color::Water)),
                ShapeType::Col(-2, Drop::Colored(Color::Water)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_leftside_3() {
        let input = "左縦3列を火ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(1, Drop::Colored(Color::Fire)),
                ShapeType::Col(2, Drop::Colored(Color::Fire)),
                ShapeType::Col(3, Drop::Colored(Color::Fire)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_left_center_3_1() {
        let input = "左から3列目縦1列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Col(3, Drop::Colored(Color::Wood))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_right_center_2_1() {
        let input = "右から2列目縦1列を火ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Col(-2, Drop::Colored(Color::Fire))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_left_center_3_2() {
        let input = "左から3列目縦2列を光ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(3, Drop::Colored(Color::Lightning)),
                ShapeType::Col(4, Drop::Colored(Color::Lightning)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_right_center_2_3() {
        let input = "右から2列目縦3列を光ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(-2, Drop::Colored(Color::Lightning)),
                ShapeType::Col(-3, Drop::Colored(Color::Lightning)),
                ShapeType::Col(-4, Drop::Colored(Color::Lightning)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_col_lcenter_2_rcenter_2() {
        let input = "左から2列目と右から2列目縦1列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(2, Drop::Colored(Color::Wood)),
                ShapeType::Col(-2, Drop::Colored(Color::Wood)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    /// 存在しないスキルパターンだが、ロジックのテスト用として書く
    #[test]
    fn gen_shape_col_lcenter_2_2_rcenter_2_2() {
        let input = "左から2列目と右から2列目縦2列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Col(2, Drop::Colored(Color::Wood)),
                ShapeType::Col(3, Drop::Colored(Color::Wood)),
                ShapeType::Col(-2, Drop::Colored(Color::Wood)),
                ShapeType::Col(-3, Drop::Colored(Color::Wood)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_top_1() {
        let input = "最上段横1列を水ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Row(1, Drop::Colored(Color::Water))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_bottom_1() {
        let input = "最下段横1列を闇ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Row(-1, Drop::Colored(Color::Dark))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_top_2() {
        let input = "最上段横2列を水ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Row(1, Drop::Colored(Color::Water)),
                ShapeType::Row(2, Drop::Colored(Color::Water)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_bottom_2() {
        let input = "最下段横2列を火ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Row(-1, Drop::Colored(Color::Fire)),
                ShapeType::Row(-2, Drop::Colored(Color::Fire)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_top_1_bottom_1() {
        let input = "最上段横1列と最下段横1列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Row(1, Drop::Colored(Color::Wood)),
                ShapeType::Row(-1, Drop::Colored(Color::Wood)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_tcenter_2_1() {
        let input = "上から2段目横1列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Row(2, Drop::Colored(Color::Wood))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_bcenter_2_2() {
        let input = "下から2段目横2列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Row(-2, Drop::Colored(Color::Wood)),
                ShapeType::Row(-3, Drop::Colored(Color::Wood)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_tcenter_2_bcenter_2() {
        let input = "上から2段目と下から2段目横1列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Row(2, Drop::Colored(Color::Wood)),
                ShapeType::Row(-2, Drop::Colored(Color::Wood)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_row_top_1_bcenter_2() {
        let input = "最上段横1列を火に、下から2段目横1列を木ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![
                ShapeType::Row(1, Drop::Colored(Color::Fire)),
                ShapeType::Row(-2, Drop::Colored(Color::Wood)),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

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
}

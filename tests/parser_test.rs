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
    fn change_drops_rand_colors_1() {
        let input = "火と木をランダムで光と回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeDropAToB(
                vec![Drop::Colored(Color::Fire), Drop::Colored(Color::Wood)],
                vec![
                    Drop::Colored(Color::Lightning),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_drops_rand_colors_2() {
        let input = "光と回復をランダムで火、水、木に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::ChangeDropAToB(
                vec![
                    Drop::Colored(Color::Lightning),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ],
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Wood),
                ],
            ),
        }]);

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
    fn gen_random_drop_15_2() {
        let input = "ランダムで火と光を15個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![],
                vec![
                    (Drop::Colored(Color::Fire), 15),
                    (Drop::Colored(Color::Lightning), 15),
                ],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_10_3() {
        let input = "ドロップのロックを解除し、水、光、回復を10個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::GenRandomDrop(
                    vec![],
                    vec![
                        (Drop::Colored(Color::Water), 10),
                        (Drop::Colored(Color::Lightning), 10),
                        (Drop::NonColored(NonColoredDrop::Recovery), 10),
                    ],
                ),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drops_diff_num_1() {
        let input = "ランダムで木ドロップを11個、闇ドロップを7個、回復ドロップを5個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::GenRandomDrop(
                vec![
                    Drop::Colored(Color::Wood),
                    Drop::Colored(Color::Dark),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ],
                vec![
                    (Drop::Colored(Color::Wood), 11),
                    (Drop::Colored(Color::Dark), 7),
                    (Drop::NonColored(NonColoredDrop::Recovery), 5),
                ],
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drops_diff_num_2() {
        let input = "ドロップのロックを解除し、水を3個、回復を9個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::GenRandomDrop(
                    vec![
                        Drop::Colored(Color::Water),
                        Drop::NonColored(NonColoredDrop::Recovery),
                    ],
                    vec![
                        (Drop::Colored(Color::Water), 3),
                        (Drop::NonColored(NonColoredDrop::Recovery), 9),
                    ],
                ),
            },
        ]);

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
    fn gen_shape_l_1() {
        let input = "L字型に光を1つ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::LShape(
                Drop::Colored(Color::Lightning),
                1,
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_l_2() {
        let input = "L字型に闇を2つ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::LShape(
                Drop::Colored(Color::Dark),
                2,
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_z() {
        let input = "盤面上にZ字型に水を生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ZShape(Drop::Colored(Color::Water))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_smallcloss_1() {
        let input = "十字型に火を1つ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::SmallCrossShape(
                Drop::Colored(Color::Fire),
                1,
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_bigcloss_1() {
        let input = "十字型に光ドロップを生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::CrossShape(Drop::Colored(
                Color::Lightning,
            ))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_square() {
        let input = "3×3の正方形に木ドロップを1つ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::Square(
                Drop::Colored(Color::Wood),
                3,
                1,
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_board_perimeter() {
        let input = "盤面外周を火ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfBoardPerimeter(
                Drop::Colored(Color::Fire),
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_board_top() {
        let input = "盤面上部に闇ドロップを12個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfBoardTop(
                Drop::Colored(Color::Dark),
                12,
            )]),
        }]);
        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_board_center() {
        let input = "盤面中央を闇ドロップに変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfBoardCenter(Drop::Colored(
                Color::Dark,
            ))]),
        }]);
        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_board_bottom() {
        let input = "盤面下部に回復ドロップを12個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfBoardBottom(
                Drop::NonColored(NonColoredDrop::Recovery),
                12,
            )]),
        }]);
        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_board_corners() {
        let input = "盤面4隅に水ドロップを1個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfBoardCorners(
                Drop::Colored(Color::Water),
                1,
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_spiderweb() {
        let input = "蜘蛛の巣状に火と回復ドロップを生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfSpiderweb(
                Drop::Colored(Color::Fire),
                Drop::NonColored(NonColoredDrop::Recovery),
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_crescent_moon() {
        let input = "三日月状に光ドロップを生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfCrescentMoon(Drop::Colored(
                Color::Lightning,
            ))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_oblique() {
        let input = "盤面上に斜めに木ドロップを生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfOblique(Drop::Colored(
                Color::Wood,
            ))]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_of_some_kind() {
        let input = "7の形に火ドロップを1個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropShapeGen(vec![ShapeType::ShapeOfSomeKind(
                Drop::Colored(Color::Fire),
                "7".to_owned(),
                1,
            )]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_unlock() {
        let input = "ドロップのロック状態を解除。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropUnLock,
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_change_with_drop_unlock() {
        let input = "ドロップのロック状態を解除。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropUnLock,
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_drop_with_drop_unlock_1() {
        let input = "全ドロップのロックを解除し、右端1列を光に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropShapeGen(vec![ShapeType::Col(
                    -1,
                    Drop::Colored(Color::Lightning),
                )]),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_drop_with_drop_unlock_2() {
        let input = "ドロップのロックを解除し、右端1列を光に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropShapeGen(vec![ShapeType::Col(
                    -1,
                    Drop::Colored(Color::Lightning),
                )]),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_shape_drop_with_drop_unlock_3() {
        let input = "ロックを解除し、右端1列を光に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropShapeGen(vec![ShapeType::Col(
                    -1,
                    Drop::Colored(Color::Lightning),
                )]),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_with_drop_unlock_1() {
        let input = "ドロップのロックを解除し、火と回復を6個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::GenRandomDrop(
                    vec![
                        Drop::Colored(Color::Fire),
                        Drop::NonColored(NonColoredDrop::Recovery),
                    ],
                    vec![
                        (Drop::Colored(Color::Fire), 6),
                        (Drop::NonColored(NonColoredDrop::Recovery), 6),
                    ],
                ),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_random_drop_with_drop_unlock_2() {
        let input = "ドロップのロックを解除し、回復以外から火と闇を3個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::GenRandomDrop(
                    vec![
                        Drop::NonColored(NonColoredDrop::Recovery),
                        Drop::Colored(Color::Fire),
                        Drop::Colored(Color::Dark),
                    ],
                    vec![
                        (Drop::Colored(Color::Fire), 3),
                        (Drop::Colored(Color::Dark), 3),
                    ],
                ),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_drop_with_drop_unlock_1() {
        let input = "ドロップのロックを解除し、光を闇に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::ChangeDropAToB(
                    vec![Drop::Colored(Color::Lightning)],
                    vec![Drop::Colored(Color::Dark)],
                ),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_all_of_board_with_drop_unlock_1() {
        let input = "全ドロップのロックを解除し、5属性+回復に変化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::DropUnLock,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::ChangeAllOfBoard(vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Wood),
                    Drop::Colored(Color::Lightning),
                    Drop::Colored(Color::Dark),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ]),
            },
        ]);

        assert_eq!(except, grammar);
    }

    /// 図鑑No.9223 極醒の蒼龍契士・ティフォン
    #[test]
    fn blue_typhon() {
        let input = "全ドロップを回復に変化し、火、水、光を9個ずつ生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::ChangeAllOfBoard(vec![Drop::NonColored(
                    NonColoredDrop::Recovery,
                )]),
            },
            Skill {
                sub_effects: None,
                turns_of_apply: None,
                effect: SkillEffect::GenRandomDrop(
                    vec![],
                    vec![
                        (Drop::Colored(Color::Fire), 9),
                        (Drop::Colored(Color::Water), 9),
                        (Drop::Colored(Color::Lightning), 9),
                    ],
                ),
            },
        ]);

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

    #[test]
    fn drop_powerup_all() {
        let input = "全ドロップを強化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropPowerUp(vec![
                Drop::Colored(Color::Fire),
                Drop::Colored(Color::Water),
                Drop::Colored(Color::Wood),
                Drop::Colored(Color::Lightning),
                Drop::Colored(Color::Dark),
                Drop::NonColored(NonColoredDrop::Recovery),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_powerup_color_1() {
        let input = "木ドロップを強化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropPowerUp(vec![Drop::Colored(Color::Wood)]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_powerup_color_2() {
        let input = "水と回復ドロップを強化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: None,
            effect: SkillEffect::DropPowerUp(vec![
                Drop::Colored(Color::Water),
                Drop::NonColored(NonColoredDrop::Recovery),
            ]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_falloff_color_1() {
        let input = "1ターンの間、火ドロップが落ちやすくなる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::DropFalloff(
                vec![Drop::Colored(Color::Fire)],
                VolumeVariation::Normal,
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_falloff_color_2() {
        let input = "3ターンの間、水と回復ドロップが少し落ちやすくなる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(3),
            effect: SkillEffect::DropFalloff(
                vec![
                    Drop::Colored(Color::Water),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ],
                VolumeVariation::Little,
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_falloff_color_3() {
        let input = "99ターンの間、光、闇、回復ドロップがほんの少し落ちやすくなる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(99),
            effect: SkillEffect::DropFalloff(
                vec![
                    Drop::Colored(Color::Lightning),
                    Drop::Colored(Color::Dark),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ],
                VolumeVariation::LittleMore,
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_falloff_only_colors() {
        let input = "1ターンの間、火、水、光、回復ドロップのみ落ちてくる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::DropFalloff(
                vec![
                    Drop::Colored(Color::Fire),
                    Drop::Colored(Color::Water),
                    Drop::Colored(Color::Lightning),
                    Drop::NonColored(NonColoredDrop::Recovery),
                ],
                VolumeVariation::Only,
            ),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn powerup_drop_falloff_1() {
        let input = "1ターンの間、強化ドロップが少し落ちやすくなる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::PowerupDropFalloff(PowerupDropFalloffKind::VolumeVariation(
                VolumeVariation::Little,
            )),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn powerup_drop_falloff_2() {
        let input = "2ターンの間、強化ドロップが25%の確率で落ちてくる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(2),
            effect: SkillEffect::PowerupDropFalloff(PowerupDropFalloffKind::Num(25)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn powerup_drop_falloff_3() {
        let input = "4ターンの間、強化ドロップが50%の確率で落ちてくる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(4),
            effect: SkillEffect::PowerupDropFalloff(PowerupDropFalloffKind::Num(50)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn powerup_drop_falloff_4() {
        let input = "6ターンの間、強化ドロップが100%の確率で落ちてくる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(6),
            effect: SkillEffect::PowerupDropFalloff(PowerupDropFalloffKind::Num(100)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_and_powerup_drop_falloff_1() {
        let input = "1ターンの間、水と光ドロップ、強化ドロップが少し落ちやすくなる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: Some(1),
                effect: SkillEffect::DropFalloff(
                    vec![Drop::Colored(Color::Water), Drop::Colored(Color::Lightning)],
                    VolumeVariation::Little,
                ),
            },
            Skill {
                sub_effects: None,
                turns_of_apply: Some(1),
                effect: SkillEffect::PowerupDropFalloff(PowerupDropFalloffKind::VolumeVariation(
                    VolumeVariation::Little,
                )),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drop_and_powerup_drop_falloff_2() {
        let input = "1ターンの間、強化ドロップと火ドロップが少し落ちやすくなる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: Some(1),
                effect: SkillEffect::PowerupDropFalloff(PowerupDropFalloffKind::VolumeVariation(
                    VolumeVariation::Little,
                )),
            },
            Skill {
                sub_effects: None,
                turns_of_apply: Some(1),
                effect: SkillEffect::DropFalloff(
                    vec![Drop::Colored(Color::Fire)],
                    VolumeVariation::Little,
                ),
            },
        ]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn lock_drop_fall() {
        let input = "1ターンの間、火ドロップがロック状態で落ちてくる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::FallLockDrop(vec![Drop::Colored(Color::Fire)]),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn lock_drop_fall_all() {
        let input = "10ターンの間、全ドロップがロック状態で落ちてくる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = match grammar.skill_list.pop().unwrap().effect {
            SkillEffect::FallLockDrop(drops) => drops.len(),
            _ => 0,
        };

        assert_eq!(except, 10);
    }

    #[test]
    fn fall_nail_drop() {
        let input = "3ターンの間、釘ドロップが落ちやすくなる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(3),
            effect: SkillEffect::FallNailDropEasierToFalloff(VolumeVariation::Normal),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn drops_not_falling() {
        let input = "1ターンの間、落ちコンなし。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::DropsNotFalling,
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_roulette_1() {
        let input = "1ターンの間、ランダムでルーレットを1個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::GenRoulette(1),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_roulette_2() {
        let input = "3ターンの間、ランダムでルーレットを2個生成。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(3),
            effect: SkillEffect::GenRoulette(2),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_cloud_1() {
        let input = "1ターンの間、盤面に2×2の雲が発生。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::GenCloud(BoardPosition::Random, Size(2, 2)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_cloud_2() {
        let input = "4ターンの間、最上段に5×1の雲が発生。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(4),
            effect: SkillEffect::GenCloud(BoardPosition::Row(1), Size(5, 1)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_teap_1() {
        let input = "1ターンの間、左端1列が操作不可になる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::GenTeap(BoardPosition::Col(1)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn gen_teap_2() {
        // 該当スキルは存在しないが、ロジックの検証のためのテストケース
        let input = "1ターンの間、最下段横1列が操作不可になる。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::GenTeap(BoardPosition::Row(-1)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_board_size_76() {
        let input = "1ターンの間、盤面を7×6マスにする。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::ChangeBoardSize(Size(7, 6)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_board_size_65() {
        let input = "3ターンの間、盤面を6×5マスにする。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(3),
            effect: SkillEffect::ChangeBoardSize(Size(6, 5)),
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn change_board_size_54() {
        let input = "5ターンの間、盤面を5×4マスにする。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(5),
            effect: SkillEffect::ChangeBoardSize(Size(5, 4)),
        }]);

        assert_eq!(except, grammar);
    }
}

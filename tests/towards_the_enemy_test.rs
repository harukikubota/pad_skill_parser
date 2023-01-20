#[cfg(test)]
mod parser_test {
    use pad_skill_parser::schema::*;
    use pad_skill_parser::skill::*;
    use pad_skill_parser::skill_grammar::*;
    use pad_skill_parser::skill_parser::parse;

    const FILE_NAME: &str = "towards_the_enemy_test.rs";

    fn new<'t>(s: Vec<Skill>) -> SkillGrammar<'t> {
        let mut g = SkillGrammar::new();
        g.skill_list = s;
        g
    }

    #[test]
    fn nullification_damage_absorption() {
        let input = "1ターンの間、ダメージ吸収を無効化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::NullificationDamageAbsorption,
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn nullification_attribute_absorption() {
        let input = "1ターンの間、属性吸収を無効化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::NullificationAttributeAbsorption,
        }]);

        assert_eq!(except, grammar);
    }

    #[test]
    fn nullification_w_absorption() {
        let input = "1ターンの間、ダメージ吸収と属性吸収を無効化。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![
            Skill {
                sub_effects: None,
                turns_of_apply: Some(1),
                effect: SkillEffect::NullificationDamageAbsorption,
            },
            Skill {
                sub_effects: None,
                turns_of_apply: Some(1),
                effect: SkillEffect::NullificationAttributeAbsorption,
            },
        ]);

        assert_eq!(except, grammar);
    }

    //#[test]
    fn penetrate_damage_nullification() {
        let input = "1ターンの間、ダメージ無効を貫通。";
        let grammar = &mut SkillGrammar::new();
        let _parsed = parse(input, FILE_NAME, grammar).unwrap();

        let except = &mut new(vec![Skill {
            sub_effects: None,
            turns_of_apply: Some(1),
            effect: SkillEffect::NullificationAttributeAbsorption,
        }]);

        assert_eq!(except, grammar);
    }
}

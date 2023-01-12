use parol::build::Builder;

fn main() {
    Builder::with_explicit_output_dir("src")
        .grammar_file("src/skill.par")
        .expanded_grammar_output_file("skill-exp.par")
        .parser_output_file("skill_parser.rs")
        .actions_output_file("skill_grammar_trait_tmp.rs")
        .enable_auto_generation()
        .user_type_name("SkillGrammar")
        .user_trait_module_name("skill_grammar")
        .generate_parser()
        .unwrap();

    replace()
}

fn replace() {
    use std::fs::File;
    use std::io::prelude::*;

    let mut grammar_trait = File::open("./src/skill_grammar_trait_tmp.rs").unwrap();

    let mut contents = String::new();
    grammar_trait.read_to_string(&mut contents).unwrap();

    // re-exportが効いてないからクレートルートから参照するように変更する
    contents = contents.replace("parol_runtime::derive_builder", "derive_builder");

    contents = contents.replace("use parol_runtime::parol_macros", "use parol_macros");

    let mut new_file = File::create("./src/skill_grammar_trait.rs").unwrap();

    new_file.write_all(contents.as_bytes()).unwrap();

    std::process::Command::new("rm")
        .arg("./src/skill_grammar_trait_tmp.rs")
        .spawn()
        .unwrap();
    ()
}

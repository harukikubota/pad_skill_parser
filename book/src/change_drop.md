# 変換スキル
## 単色変換
指定されているドロップを別のドロップへ変化させる。
以下のようなスキルが該当する。

* 火ドロップを木ドロップに変化。
* 水ドロップを光ドロップに、闇ドロップを回復に変化。
* 火、回復ドロップを水ドロップに変化。
* 火と木をランダムで光と回復に変化。

``` rust
let input = "火ドロップを木ドロップに変化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    // 単色変換
    effect: SkillEffect::ChangeDropAToB(
        // from: 変換元
        vec![Drop::Colored(Color::Fire)],
        // to:   変換先
        vec![Drop::Colored(Color::Wood)],
    ),
}
```

----

``` rust
let input = "水ドロップを光ドロップに、闇ドロップを回復に変化。";
//           ①~~~~~~~~~~~~~~~~~~  ②~~~~~~~~~~~~~
[
    // ①
    Skill {
        sub_effects: None,
        turns_of_apply: None,
        effect: SkillEffect::ChangeDropAToB(
            vec![Drop::Colored(Color::Water)],
            vec![Drop::Colored(Color::Lightning)],
        ),
    },
    // ②
    Skill {
        sub_effects: None,
        turns_of_apply: None,
        effect: SkillEffect::ChangeDropAToB(
            vec![Drop::Colored(Color::Dark)],
            vec![Drop::NonColored(NonColoredDrop::Recovery)],
        ),
    },
]
```

複数回変換を行う場合、スキルは２つに別れる。

----

``` rust
let input = "火と木をランダムで光と回復に変化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::ChangeDropAToB(
        vec![
            Drop::Colored(Color::Fire),
            Drop::Colored(Color::Wood)
        ],
        vec![
            Drop::Colored(Color::Lightning),
            Drop::NonColored(NonColoredDrop::Recovery)
        ]
    ),
}
```

上記スキルパターンは`NO. 5361 シロオンマル`、`NO. 5362 クロオンマル`のみが持つ。(23/01/17)

## 全ドロップ変化
盤面全てを特定の色のみに変換するスキル群。所謂N色陣や花火。

以下のようなスキルが該当する。
* 全ドロップを闇ドロップに変化。
* 全ドロップを火と光ドロップに変化。
* 全ドロップを木、光、回復ドロップに変化。
* 全ドロップを5属性に変化。
* 全ドロップを5属性+回復に変化。

``` rust
let input = "全ドロップを闇ドロップに変化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::ChangeAllOfBoard(
        // 変化先
        vec![Drop::Colored(Color::Dark)]
    ),
}
```

----

``` rust
let input = "全ドロップを5属性+回復に変化。";
Skill {
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
        ]
    ),
}
```

## ランダム生成
特定のドロップを指定個数生成するスキル群。
以下のようなスキルが該当する。

* ランダムで火ドロップを1個生成。
* ランダムで火と光ドロップを5個ずつ生成。
* 回復以外から火ドロップを6個生成。
* ランダムで木ドロップを11個、闇ドロップを7個、回復ドロップを5個生成。
* ランダムで火と光を15個ずつ生成。
* 5属性+回復を4個ずつ生成。

``` rust
let input = "ランダムで火ドロップを1個生成。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::GenRandomDrop(
        // ①
        vec![Drop::Colored(Color::Fire)],
        // ②
        vec![
            (
                // ③
                Drop::Colored(Color::Fire)
                // ④
                ,1
            )
        ],
    ),
}
```

①. ここに指定されているドロップ以外から、②を生成する
②. 生成するドロップの種類と個数のリスト
③. ドロップの種類
④. ③の生成数。

### ①を設定する基準
* 明記されていない場合は、生成するドロップと等しい
* 明記されている場合は、生成するドロップと合わせる
* 生成するドロップが5種類以上または生成するドロップの合計が30の場合、盤面を上書きする仕様となるため空になる。

----

``` rust
let input = "ランダムで火と光ドロップを5個ずつ生成。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::GenRandomDrop(
        vec![
            Drop::Colored(Color::Fire),
            Drop::Colored(Color::Lightning),
        ],
        vec![
            (Drop::Colored(Color::Fire), 5),
            (Drop::Colored(Color::Lightning), 5),
        ],
    ),
}
```

----

``` rust
let input = "回復以外から火ドロップを6個生成。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::GenRandomDrop(
        vec![
            Drop::NonColored(NonColoredDrop::Recovery),
            Drop::Colored(Color::Fire)
        ],
        vec![(Drop::Colored(Color::Fire), 6)],
    ),
}
```

----

``` rust
let input = "ランダムで木ドロップを11個、闇ドロップを7個、回復ドロップを5個生成。";
Skill {
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
}
```

----

``` rust
let input = "ランダムで火と光を15個ずつ生成。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::GenRandomDrop(
        // 生成するドロップの合計が30のため、空になる
        vec![],
        vec![
            (Drop::Colored(Color::Fire), 15),
            (Drop::Colored(Color::Lightning), 15),
        ],
    ),
}
```

----

``` rust
let input = "5属性+回復を4個ずつ生成。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::GenRandomDrop(
        vec![],
        vec![
            (Drop::Colored(Color::Fire),4),
            (Drop::Colored(Color::Water),4),
            (Drop::Colored(Color::Wood),4),
            (Drop::Colored(Color::Lightning),4),
            (Drop::Colored(Color::Dark),4),
            (Drop::NonColored(NonColoredDrop::Recovery)4)
        ],
    ),
}
```

## 指定型生成
形状に沿ってドロップを生成するスキル群。
以下のようなスキルが該当する。

* 左端1列を光ドロップに変化。
* 左端2列と右端2列を水ドロップに変化。
* 最上段横1列を水ドロップに変化。
* L字型に光を1つ生成。

`ShapeType::Row`、`ShapeType::Col`の1要素目は以下のようにして設定される。

* Row@Pos: 上からN番目
* Row@Neg: 下からN番目
* Col@Pos: 左からN番目
* Col@Neg: 右からN番目

``` rust
let input = "左端1列を光ドロップに変化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropShapeGen(
        vec![
            // どのような形状を生成するか
            // 詳細は`skill::ShapeType`を参照
            ShapeType::Col(1, Drop::Colored(Color::Lightning))
        ]
    ),
}
```

----

``` rust
let input = "左端2列と右端2列を水ドロップに変化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropShapeGen(
        vec![
            // 生成する行、列毎に要素が生成される
            ShapeType::Col(1, Drop::Colored(Color::Water)),
            ShapeType::Col(2, Drop::Colored(Color::Water)),
            ShapeType::Col(-1, Drop::Colored(Color::Water)),
            ShapeType::Col(-2, Drop::Colored(Color::Water))
        ]
    ),
}
```

----

``` rust
let input = "最上段横1列を水ドロップに変化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropShapeGen(
        vec![ShapeType::Row(1, Drop::Colored(Color::Water))]
    ),
}
```

----

``` rust
let input = "L字型に光を1つ生成。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropShapeGen(
        vec![ShapeType::LShape(Drop::Colored(Color::Lightning), 1)]
    ),
}
```

L字型以外にも形状生成はあるが、特に変わりはないのでどのようなデータか列挙する。

### 生成する数量が設定されている
* L字型
* 十字型(5個生成)
* 正方形(3×3)
* 7の形

### 盤面の特定の位置にドロップを生成する
* 縦列、横列
* Z字型
* 十字型(10個生成)
* 盤面外周
* 盤面上部(該当スキルなし(23/01/17 現在))
* 盤面中央
* 盤面下部
* 盤面4隅
* 蜘蛛の巣状
* 三日月状
* 斜め

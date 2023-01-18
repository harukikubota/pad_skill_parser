# 盤面干渉スキル

変換スキル以外の、盤面に干渉するスキル群。

## ロック解除

* ドロップのロック状態を解除。
* 全ドロップのロックを解除し、右端1列を光に変化。(ロック解除が複合する変換スキル)

``` rust
let input = "ドロップのロック状態を解除。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropUnLock,
}
```

## ドロップリフレッシュ

* ランダムでドロップを入れ替える。

``` rust
let input = "ランダムでドロップを入れ替える。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropRefresh,
}
```

## 落ちコンなし

* 1ターンの間、落ちコンなし。

``` rust
let input = "1ターンの間、落ちコンなし。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::DropsNotFalling,
}
```

## ドロップ強化

* 木ドロップを強化。
* 全ドロップを強化。

``` rust
let input = "木ドロップを強化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropPowerUp(vec![
        // 強化するドロップのリスト
        Drop::Colored(Color::Wood)
    ]),
}
```

----

``` rust
let input = "全ドロップを強化。";
Skill {
    sub_effects: None,
    turns_of_apply: None,
    effect: SkillEffect::DropPowerUp(vec![
        Drop::Colored(Color::Fire),
        Drop::Colored(Color::Water),
        Drop::Colored(Color::Wood),
        Drop::Colored(Color::Lightning),
        Drop::Colored(Color::Dark),
        Drop::NonColored(NonColoredDrop::Recovery)
    ]),
}
```

## ルーレット生成

* 1ターンの間、ランダムでルーレットを1個生成。

``` rust
let input = "1ターンの間、ランダムでルーレットを1個生成。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::DropPowerUp(vec![SkillEffect::GenRoulette(1)]),
}
```

## 雲発生

* 1ターンの間、盤面に2×2の雲が発生。
* 4ターンの間、最上段に5×1の雲が発生。

``` rust
let input = "1ターンの間、盤面に2×2の雲が発生。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::GenCloud(BoardPosition::Random, Size(2, 2)),
}
```

----

``` rust
let input = "4ターンの間、最上段に5×1の雲が発生。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    // 生成位置については、`change_drop.md/##指定型生成`で解説している
    effect: SkillEffect::GenCloud(BoardPosition::Row(1), Size(5, 1)),
}
```

## テープ発生

* 1ターンの間、左端1列が操作不可になる。

``` rust
let input = "1ターンの間、左端1列が操作不可になる。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::GenTeap(BoardPosition::Col(1)),
}
```

## 盤面サイズ変更

* 1ターンの間、盤面を7×6マスにする。

``` rust
let input = "1ターンの間、盤面を7×6マスにする。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::ChangeBoardSize(Size(7, 6)),
}
```

## ドロップ目覚め
指定したドロップが盤面に落ちやすくなる、指定したドロップのみが落ちるスキル群。

* 1ターンの間、火ドロップが落ちやすくなる。
* 1ターンの間、火、水、光、回復ドロップのみ落ちてくる。

``` rust
let input = "1ターンの間、火ドロップが落ちやすくなる。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::DropFalloff(
        // 落ちやすくなるドロップリスト
        vec![Drop::Colored(Color::Fire)],
        // 落ちやすくなる度合い
        VolumeVariation::Normal,
    ),
}
```

----

``` rust
let input = "1ターンの間、火、水、光、回復ドロップのみ落ちてくる。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::DropFalloff(
        vec![
            Drop::Colored(Color::Fire),
            Drop::Colored(Color::Water),
            Drop::Colored(Color::Lightning),
            Drop::NonColored(NonColoredDrop::Recovery)
        ],
        VolumeVariation::Only,
    ),
}
```

## 強化ドロップ目覚め

* 1ターンの間、強化ドロップが25%の確率で落ちてくる。
* 1ターンの間、強化ドロップが少し落ちやすくなる。

``` rust
let input = "1ターンの間、強化ドロップが25%の確率で落ちてくる。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::PowerupDropFalloff(
        // 実数値が指定されている場合
        PowerupDropFalloffKind::Num(25)
    ),
}
```

----

``` rust
let input = "1ターンの間、強化ドロップが少し落ちやすくなる。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::PowerupDropFalloff(
        // 実数値が指定されていない場合
        PowerupDropFalloffKind::VolumeVariation(VolumeVariation::Little)
    ),
}
```

## ロックドロップ目覚め

* 1ターンの間、火ドロップがロック状態で落ちてくる。

``` rust
let input = "1ターンの間、火ドロップがロック状態で落ちてくる。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::FallLockDrop(vec![Drop::Colored(Color::Fire)]),
}
```

## 釘ドロップ目覚め

* 3ターンの間、釘ドロップが落ちやすくなる。

``` rust
let input = "1ターンの間、釘ドロップが落ちやすくなる。";
Skill {
    sub_effects: None,
    turns_of_apply: Some(1),
    effect: SkillEffect::FallNailDropEasierToFalloff(VolumeVariation::Normal),
}
```

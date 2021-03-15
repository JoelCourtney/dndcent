crate::name!("Variant Human");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VariantHuman {
    abilities: [Ability; 2],
    skill: Skill,
    feat: Box<dyn Feat>,
    language: Language
}

#[content]
impl Race for VariantHuman {
    properties! {}

    fn declare(&self, c: &mut Character) {
        common_race_rules::declare(c, self);

        c.size.declare_initializer(NAME);
        c.languages.declare_initializer(NAME);

        for ability in &self.abilities {
            match c.get_mut_ability(*ability) {
                Some(a) => a.declare_modifier(NAME),
                None => {}
            }
        }

        match c.get_mut_skill_proficiency(self.skill) {
            Some(s) => s.declare_initializer(NAME),
            None => {}
        }

        self.feat.declare(c);
    }
    fn iterate(&self, c: &mut Character) {
        common_race_rules::iterate(c, self);

        if c.size.initialize(NAME) {
            *c.size = CreatureSize::Medium;
        }

        if c.languages.initialize(NAME) {
            (*c.languages).push(Language::Common);
            if self.language != Language::Unknown {
                (*c.languages).push(self.language);
            }
        }

        for ability in &self.abilities {
            match c.get_mut_ability(*ability) {
                Some(a) => {
                    if a.modify(NAME) {
                        **a += 1;
                    }
                }
                None => {}
            }
        }

        match c.get_mut_skill_proficiency(self.skill) {
            Some(s) => {
                if s.initialize(NAME) {
                    **s = ProficiencyType::Single;
                }
            }
            None => {}
        }

        self.feat.iterate(c);
    }
    fn last(&mut self, c: &mut Character) {
        c.race_traits.extend(vec! [
            Feature (
                "**Ability Score Increase:** Two different ability scores of your choice increase by 1.",
                Any(&mut self.abilities)
            ),
            Feature (
                "**Skills:** You gain proficiency in one skill of your choice.",
                Any(&mut self.skill)
            ),
            Feature (
                "**Languages:** You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                Any(&mut self.language)
            ),
            Feature (
                "**Feat:** You gain one feat of your choice.",
                Any(&mut self.feat)
            )
        ]);

        self.feat.last(c);
    }

    description! { r#"
        # Variant Human

        If your campaign uses the optional feat rules from the Player’s Handbook, your Dungeon Master might allow these variant traits, all of which replace the human’s Ability Score Increase trait.

        ## Variant Human Traits

        ### Ability Score Increase

        Two different ability scores of your choice increase by 1.

        ### Skills

        You gain proficiency in one skill of your choice.

        ### Feat

        You gain one feat of your choice.
    "#}
}


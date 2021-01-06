use serde::{Deserialize, Serialize};

use crate::modify::*;
use crate::feature::Feature;

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    name: String,

    health: u64,
    temp_health: u64,

    base_strength: u8,
    base_dexterity: u8,
    base_constitution: u8,
    base_intelligence: u8,
    base_wisdom: u8,
    base_charisma: u8,

    alignment: Alignment,

    race: Box<dyn Race>
}

impl StoredCharacter {
    pub fn resolve(&mut self) -> Character {
        let mut char = Character {
            name: self.name.clone(),
            health: self.health,
            temp_health: self.temp_health,

            strength: self.base_strength,
            dexterity: self.base_dexterity,
            constitution: self.base_constitution,
            intelligence: self.base_intelligence,
            wisdom: self.base_wisdom,
            charisma: self.base_charisma,

            alignment: self.alignment,

            ..Default::default()
        };
        self.race.initialize(&mut char);
        self.race.modify(&mut char);
        self.race.finalize(&mut char);
        char.traits.extend(self.race.features());
        char
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Character<'a> {
    pub name: String,

    // HEALTH
    pub health: u64,
    pub temp_health: u64,
    pub max_health: u64,

    // ABILITIES
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,

    pub strength_modifier: i8,
    pub dexterity_modifier: i8,
    pub constitution_modifier: i8,
    pub intelligence_modifier: i8,
    pub wisdom_modifier: i8,
    pub charisma_modifier: i8,

    // SIZE
    pub size: CreatureSize,

    // ALIGNMENT
    pub alignment: Alignment,

    // PROFICIENCIES AND LANGUAGES
    pub skill_proficiencies: Vec<(Skill, ProficiencyType)>,
    pub languages: Vec<Language>,

    // SPEED
    pub walking_speed: u8,
    pub flying_speed: u8,
    pub climbing_speed: u8,
    pub swimming_speed: u8,
    pub burrowing_speed: u8,

    // FEATURES AND TRAITS
    pub traits: Vec<Feature<'a>>,
    pub features: Vec<Feature<'a>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CreatureSize {
    Unspecified,
    Fine,
    Diminutive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal
}

impl Default for CreatureSize {
    fn default() -> Self { CreatureSize::Unspecified }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
    Unspecified,
    ItsComplicated
}

impl Default for Alignment {
    fn default() -> Self { Alignment::Unspecified }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SavingThrow {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    Death
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProficiencyType {
    NONE,
    HALF,
    SINGLE,
    DOUBLE
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Language {
    Abyssal,
    Aquan,
    Auran,
    Celestial,
    Common,
    DeepSpeech,
    Draconic,
    Druidic,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Gnoll,
    Halfling,
    Ignan,
    Infernal,
    Orc,
    Primordial,
    Sylvan,
    Terran,
    Undercommon,
    Unspecified,
    Other(String)
}

impl Default for Language {
    fn default() -> Self {
        Language::Unspecified
    }
}
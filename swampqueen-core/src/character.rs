//! This module contains functionality related to characters. The principal type of this module is
//! [`Character`], around which all other functionality revolves.

use std::ops::{Add, AddAssign};

// TODO better docs
/// A character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character {
    name: String,
    class: Class,
    race: Race,
    stats: StatValues,
}
impl Character {
    // TODO better docs
    /// Create a new character.
    #[must_use]
    pub fn new(
        chosen_name: &str,
        chosen_class: Class,
        chosen_race: Race,
        bonus_stat_1: Stat,
        bonus_stat_2: Stat,
    ) -> Self {
        let mut stats = StatValues::from(chosen_class, chosen_race);
        stats.add_to_stat(bonus_stat_1, 1);
        stats.add_to_stat(bonus_stat_2, 1);

        Self {
            name: String::from(chosen_name),
            class: chosen_class,
            race: chosen_race,
            stats,
        }
    }
}

// TODO better docs
/// A class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Class {
    /// TODO
    Soothsayer,
    /// TODO
    Hunter,
    /// TODO
    Trespasser,
    /// TODO
    Warden,
    /// TODO
    Bastion,
}
impl Class {}
impl TryFrom<&str> for Class {
    type Error = std::io::Error;
    // TODO make this not suck
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "soothsayer" => Ok(Self::Soothsayer),
            "hunter" => Ok(Self::Hunter),
            "trespasser" => Ok(Self::Trespasser),
            "warden" => Ok(Self::Warden),
            "bastion" => Ok(Self::Bastion),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "unrecognized value!!!",
            )),
        }
    }
}

// TODO better docs
/// A race.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Race {
    /// TODO
    AlligatorFolk,
    /// TODO
    InsectoidFae,
    /// TODO
    GoblinoidFae,
}
impl TryFrom<&str> for Race {
    type Error = std::io::Error;
    // TODO make this not suck
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "alligator_folk" => Ok(Self::AlligatorFolk),
            "insectoid_fae" => Ok(Self::InsectoidFae),
            "goblinoid_fae" => Ok(Self::GoblinoidFae),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "unrecognized value!!!",
            )),
        }
    }
}

// TODO better docs
/// All the different stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stat {
    /// TODO
    Cunning,
    /// TODO
    Slipperiness,
    /// TODO
    Bulk,
    /// TODO
    Backbone,
    /// TODO
    TheSight,
}
impl TryFrom<&str> for Stat {
    type Error = std::io::Error;
    // TODO make this not suck
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cunning" => Ok(Self::Cunning),
            "slipperiness" => Ok(Self::Slipperiness),
            "bulk" => Ok(Self::Bulk),
            "backbone" => Ok(Self::Backbone),
            "the_sight" => Ok(Self::TheSight),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "unrecognized value!!!",
            )),
        }
    }
}

/// The stat values of a given [`Character`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct StatValues {
    cunning: isize,
    slipperiness: isize,
    bulk: isize,
    backbone: isize,
    the_sight: isize,
}
impl StatValues {
    /// Create [`StatValues`] based on the chosen [`Race`] and [`Class`].
    fn from(chosen_class: Class, chosen_race: Race) -> Self {
        let mut values = Self::default();

        values.add_class_bonus(chosen_class);
        values.add_race_bonus(chosen_race);

        values
    }

    fn add_to_stat(&mut self, stat: Stat, value: isize) {
        match stat {
            Stat::Cunning => self.cunning += value,
            Stat::Slipperiness => self.slipperiness += value,
            Stat::Bulk => self.bulk += value,
            Stat::Backbone => self.backbone += value,
            Stat::TheSight => self.the_sight += value,
        };
    }

    fn add_class_bonus(&mut self, class: Class) {
        *self += match class {
            Class::Soothsayer => Self {
                cunning: 3,
                slipperiness: 3,
                bulk: 0,
                backbone: 1,
                the_sight: 5,
            },
            Class::Hunter => Self {
                cunning: 3,
                slipperiness: 1,
                bulk: 3,
                backbone: 5,
                the_sight: 0,
            },
            Class::Trespasser => Self {
                cunning: 3,
                slipperiness: 5,
                bulk: 0,
                backbone: 3,
                the_sight: 1,
            },
            Class::Warden => Self {
                cunning: 5,
                slipperiness: 1,
                bulk: 0,
                backbone: 3,
                the_sight: 3,
            },
            Class::Bastion => Self {
                cunning: 3,
                slipperiness: 1,
                bulk: 5,
                backbone: 3,
                the_sight: 0,
            },
        };
    }

    fn add_race_bonus(&mut self, race: Race) {
        *self += match race {
            Race::AlligatorFolk => Self {
                cunning: 0,
                slipperiness: 0,
                bulk: 1,
                backbone: 1,
                the_sight: 0,
            },
            Race::InsectoidFae => Self {
                cunning: 1,
                slipperiness: 0,
                bulk: 0,
                backbone: 0,
                the_sight: 1,
            },
            Race::GoblinoidFae => Self {
                cunning: 1,
                slipperiness: 1,
                bulk: 0,
                backbone: 0,
                the_sight: 0,
            },
        };
    }
}
impl<'a> Add<&'a StatValues> for &StatValues {
    type Output = StatValues;

    fn add(self, other: &'a StatValues) -> StatValues {
        StatValues {
            cunning: self.cunning + other.cunning,
            slipperiness: self.slipperiness + other.slipperiness,
            bulk: self.bulk + other.bulk,
            backbone: self.backbone + other.backbone,
            the_sight: self.the_sight + other.the_sight,
        }
    }
}
impl AddAssign for StatValues {
    fn add_assign(&mut self, rhs: Self) {
        *self = &*self + &rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_class_race_stats() {
        let stats = StatValues::from(Class::Bastion, Race::InsectoidFae);
        assert_eq!(
            stats,
            StatValues {
                cunning: 4,
                slipperiness: 1,
                backbone: 3,
                bulk: 5,
                the_sight: 1,
            }
        );
    }

    #[test]
    fn zero_in_stat() {
        let stats = StatValues::from(Class::Soothsayer, Race::GoblinoidFae);
        assert_eq!(
            stats,
            StatValues {
                cunning: 4,
                slipperiness: 4,
                backbone: 1,
                bulk: 0,
                the_sight: 5,
            }
        );
    }

    #[test]
    fn add_to_stats() {
        let character = Character::new(
            "Mr. Test",
            Class::Bastion,
            Race::InsectoidFae,
            Stat::Slipperiness,
            Stat::Bulk,
        );
        let expected = StatValues {
            cunning: 4,
            slipperiness: 2,
            backbone: 3,
            bulk: 6,
            the_sight: 1,
        };

        assert_eq!(character.stats, expected);
    }

    #[test]
    fn add_to_same_stat_twice() {
        let character = Character::new(
            "Mistah Beefcake",
            Class::Bastion,
            Race::InsectoidFae,
            Stat::Bulk,
            Stat::Bulk,
        );
        let expected = StatValues {
            cunning: 4,
            slipperiness: 1,
            backbone: 3,
            bulk: 7,
            the_sight: 1,
        };

        assert_eq!(character.stats, expected);
    }
}

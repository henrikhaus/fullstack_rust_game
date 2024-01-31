use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{Error, FromRow};
use std::fmt::Debug;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct Item {
    id: Uuid,
    name: String,
    description: String,
    owner_id: Uuid,
    item_type: String,
    item_rarity: String,
    season: i32,
    price: Option<i64>,
}

pub enum ItemType {
    Chip,
    Avatar,
    Emote,
    Effect,
}

pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Divine,
}

impl Item {
    pub fn new(
        name: String,
        description: String,
        owner_id: Uuid,
        item_type: ItemType,
        item_rarity: ItemRarity,
        season: i32,
        price: Option<i64>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            owner_id,
            item_type: item_type.into(),
            item_rarity: item_rarity.into(),
            season,
            price,
        }
    }

    /// The row id in the database
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// The name of the item
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The description of the item
    pub fn description(&self) -> &String {
        &self.description
    }

    /// The id of the item owner
    pub fn owner_id(&self) -> &Uuid {
        &self.id
    }

    /// The item type
    pub fn item_type(&self) -> ItemType {
        self.item_type.try_into().expect("Invalid enum value")
    }

    /// The item rarity
    pub fn item_rarity(&self) -> ItemRarity {
        self.item_rarity.try_into().expect("Invalid enum value")
    }

    /// The season the item was released
    pub fn season(&self) -> i32 {
        self.season
    }

    /// The optional price of the item
    pub fn price(&self) -> Option<i64> {
        self.price
    }
}

struct InvalidVariant;

impl TryFrom<&str> for ItemType {
    type Error = InvalidVariant;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "chip" => Ok(ItemType::Chip),
            "avatar" => Ok(ItemType::Avatar),
            "emote" => Ok(ItemType::Emote),
            "effect" => Ok(ItemType::Effect),
            _ => Err(Self::Error),
        }
    }
}

impl TryFrom<&str> for ItemRarity {
    type Error = InvalidVariant;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "common" => Ok(ItemRarity::Common),
            "uncommon" => Ok(ItemRarity::Uncommon),
            "rare" => Ok(ItemRarity::Rare),
            "epic" => Ok(ItemRarity::Epic),
            "legendary" => Ok(ItemRarity::Legendary),
            "divine" => Ok(ItemRarity::Divine),
            _ => Err(Self::Error),
        }
    }
}

impl From<ItemType> for &str {
    fn from(value: ItemType) -> &'static str {
        match value {
            ItemType::Chip => "chip",
            ItemType::Avatar => "avatar",
            ItemType::Emote => "emote",
            ItemType::Effect => "effect",
        }
    }
}

impl From<ItemRarity> for &str {
    fn from(value: ItemRarity) -> &'static str {
        match value {
            ItemRarity::Common => "common",
            ItemRarity::Uncommon => "uncommon",
            ItemRarity::Rare => "rare",
            ItemRarity::Epic => "epic",
            ItemRarity::Legendary => "legendary",
            ItemRarity::Divine => "divine",
        }
    }
}

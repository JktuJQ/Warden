//! `models` module provides Rust representation of database tables.
//!

use serenity::model::id::{ChannelId, GuildId};
use sqlx::FromRow;
use std::{
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

/// `Id` newtype encapsulates discord API id's that are represented by `u64`, but stored in database as `i64`.
///
#[derive(Copy, Clone, Debug, Default)]
pub struct Id(pub u64);
impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
impl FromStr for Id {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Id(u64::from_str(s)?))
    }
}
impl TryFrom<i64> for Id {
    type Error = TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Id(u64::try_from(value)?))
    }
}
impl From<ChannelId> for Id {
    fn from(value: ChannelId) -> Self {
        Id(value.0)
    }
}
impl From<Id> for ChannelId {
    fn from(value: Id) -> Self {
        ChannelId(value.0)
    }
}
impl From<GuildId> for Id {
    fn from(value: GuildId) -> Self {
        Id(value.0)
    }
}
impl From<Id> for GuildId {
    fn from(value: Id) -> Self {
        GuildId(value.0)
    }
}

/// `ForeignId` newtype encapsulates discord API id's that are represented by `u64`,
/// but stored in database as `Option<i64>` due to possible nullability.
///
#[derive(Copy, Clone, Debug, Default)]
pub struct ForeignId(pub Option<u64>);
impl TryFrom<Option<i64>> for ForeignId {
    type Error = TryFromIntError;

    fn try_from(value: Option<i64>) -> Result<Self, Self::Error> {
        Ok(ForeignId(match value {
            Some(id) => Some(u64::try_from(id)?),
            None => None,
        }))
    }
}

#[derive(Debug, FromRow)]
pub struct Channel {
    #[sqlx(try_from = "i64", default)]
    pub discord_id: Id,
}

#[derive(Debug, FromRow)]
pub struct Guild {
    #[sqlx(try_from = "i64", default)]
    pub discord_id: Id,

    #[sqlx(try_from = "i64", default)]
    pub settings_id: u64,
}

#[derive(Debug, FromRow)]
pub struct Setting {
    #[sqlx(try_from = "i64", default)]
    pub id: u64,

    #[sqlx(try_from = "Option<i64>", default)]
    pub log_channel_id: ForeignId,

    #[sqlx(try_from = "Option<i64>", default)]
    pub moderation_channel_id: ForeignId,

    #[sqlx(try_from = "Option<i64>", default)]
    pub music_order_channel_id: ForeignId,

    #[sqlx(try_from = "Option<i64>", default)]
    pub music_log_channel_id: ForeignId,
}

//! Implements the `GetOwnedGames` endpoint.

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, PlayerServiceError},
    macros::{do_http, gen_args, optional_argument},
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetOwnedGames";
const VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Wrapper {
    response: OwnedGames,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct OwnedGames {
    pub game_count: u64,
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    pub appid: u32,
    pub name: Option<String>,
    pub playtime_2weeks: Option<u64>,
    pub playtime_forever: u64,
    pub img_icon_url: Option<String>,
    pub img_logo_url: Option<String>,
    pub capsule_filename: Option<String>,
}

impl Steam {
    /// Return a list of games owned by the player.
    ///
    /// # Arguments
    ///
    /// * `steamid` - The SteamID of the player we're asking about.
    /// * `include_appinfo` - True if we want additional details (name, icon) about each game.
    /// * `include_played_free_games` - Free games are excluded by default. If this is set, free games the user has played will be returned.
    /// * `appids_filter` - If set, restricts result set to the passed in apps.
    /// * `include_free_sub` - Some games are in the free sub, which are excluded by default.
    /// * `skip_unvetted_apps` - If set, skip unvetted store apps.
    /// * `language` - Will return data in this language (english, french, etc.).
    /// * `include_extended_appinfo` - True if we want even more details (capsule, sortas, and capabilities) about each game. include_appinfo must also be true.
    pub async fn get_owned_games(
        // TODO: Extensive testing for each argument
        &self,
        steamid: SteamId,
        include_appinfo: bool,
        include_played_free_games: bool,
        appids_filter: Vec<u32>,
        include_extended_appinfo: bool,
    ) -> Result<OwnedGames, PlayerServiceError> {
        let key = &self.api_key.clone();
        let steamid = steamid.into_u64();
        let mut args = gen_args!(
            key,
            steamid,
            include_appinfo,
            include_played_free_games,
            include_extended_appinfo
        );

        if !appids_filter.is_empty() {
            let mut appids_filter_arg = serde_json::Map::new();
            appids_filter_arg.insert(
                "appids_filter".to_string(),
                serde_json::to_value(appids_filter).unwrap(),
            );
            args.push_str(&format!(
                "&{}",
                serde_json::to_string(&appids_filter_arg).unwrap()
            ));
        }

        let url = format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?{args}");
        let wrapper = do_http!(url, Wrapper, ErrorHandle, PlayerServiceError::GetOwnedGames);
        Ok(wrapper.response)
    }
}

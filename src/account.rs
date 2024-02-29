use core::panic;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{FullTrack, PlayableItem, PlaylistItem, SavedTrack};

use rspotify::model::SimplifiedPlaylist;

use rspotify::{self, ClientResult};

use crate::data_structs as data;

pub async fn get_motherlist(
    spotify: &rspotify::AuthCodeSpotify,
) -> Result<Vec<data::BetterSavedTrack>> {
    let market = rspotify::model::Market::Country(rspotify::model::Country::UnitedStates);

    let motherlist_id = get_parent_playlist_id(spotify)
        .await
        .context("Failed to get playlist id")?;
    get_parent_playlist_tracks(spotify, motherlist_id, market).await
}

pub async fn get_user_acct() -> Result<rspotify::AuthCodeSpotify> {
    match Path::new(".env").try_exists() {
        Err(_) => panic!("I honestly don't know what could cause this error but the condition apparently triggered."),
        Ok(false) =>  {
            create_env_credentials().context("Failed to create env file")?;
        },
        Ok(true) => {
            println!("You have account details already entered. Would you like to use those or enter a new client ID and client secret? [y/n]");
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).context("Failed to read line for choosing to create new account details file")?;
                input = input.trim().to_lowercase();
                println!("Input was: {}", input);
                if &input[..] == "y" {
                    create_env_credentials().context("Failed to create env file")?;
                    break;
                }
                else if &input[..] != "n" {
                    println!("Please input either \'y\' or \'n\'.");
                    continue;
                }
                break;
            }
        }
    }

    let creds = match rspotify::Credentials::from_env() {
        Some(x) => x,
        None => panic!("Unable to get credentials with given client ID and secret in env file"),
    };

    let oauth = match rspotify::OAuth::from_env(rspotify::scopes!(
        "playlist-modify-private,playlist-read-private,user-library-read"
    )) {
        Some(x) => x,
        None => panic!("Unable to get account with given client ID and secret in env file"),
    };

    let spotify = rspotify::AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).context(
        "Failed to get authorization url
Fai",
    )?;

    spotify
        .prompt_for_token(&url)
        .await
        .context("Error in parsing account token")?;

    Ok(spotify)
}

fn create_env_credentials() -> Result<()> {
    let mut client_id = "RSPOTIFY_CLIENT_ID=".to_string();
    let mut client_secret = "RSPOTIFY_CLIENT_SECRET=".to_string();
    let mut redirect_url = String::new();
    // let mut redirect_url_temp = String::new();

    println!("Please paste your spotify developer app client id here:");
    std::io::stdin()
        .read_line(&mut client_id)
        .context("Error in reading client ID")?;

    println!("Please paste your spotify developer app client secret here:");
    std::io::stdin()
        .read_line(&mut client_secret)
        .context("Error in reading client secret")?;

    println!("Please paste your spotify redirect url here:\n(If the url is \"http://localhost:8888/callback\", leave this field blank)");
    std::io::stdin()
        .read_line(&mut redirect_url)
        .context("Error in reading redirect url")?;
    println!("{redirect_url}");
    if redirect_url.len() == 1 {
        redirect_url.push_str("RSPOTIFY_REDIRECT_URI=http://localhost:8888/callback");
    } else {
        redirect_url = "RSPOTIFY_REDIRECT_URI=".to_owned() + &redirect_url;
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(".env")
        .context("Error in opening .env")?;
    let mut writer = BufWriter::new(&file);
    write!(&mut writer, "{client_id}\n{client_secret}\n{redirect_url}")
        .context("Error in writing to .env")?;

    Ok(())
}

//TODO: Figure out how to anyhow these ClientResults
async fn get_parent_playlist_id(
    spotify: &rspotify::AuthCodeSpotify,
) -> Result<Option<SimplifiedPlaylist>> {
    let playlists_result: Vec<ClientResult<SimplifiedPlaylist>> = spotify
        .current_user_playlists()
        .collect::<Vec<ClientResult<SimplifiedPlaylist>>>()
        .await;
    let playlists: Vec<SimplifiedPlaylist> = playlists_result
        .into_iter()
        .collect::<ClientResult<Vec<SimplifiedPlaylist>>>()?;

    println!("Available playlists associated with account:");
    let mut i = 0;
    for x in playlists.iter() {
        i += 1;
        println!("{i} {:?}", x.name);
    }
    i += 1;
    println!("{i} Liked Songs");
    println!("Please select a playlist as the motherlist by inputting its corresponding number");

    let mut playlist_index: i64;
    let playlists_len = playlists.len();

    loop {
        let mut playlists_index_temp = String::new();
        std::io::stdin().read_line(&mut playlists_index_temp)?;
        match playlists_index_temp.trim().parse() {
            Ok(x) => playlist_index = x,
            Err(_) => {
                println!("Could not parse your input, please enter a number.");
                continue;
            }
        };

        if playlist_index < 0
            || std::convert::TryInto::<usize>::try_into(playlist_index)
                .expect("subplaylist_index really shouldn't be greater than max of usize")
                > playlists_len + 1
        {
            println!("Please enter a number corresponding to one of the playlists.");
            continue;
        }
        break;
    }

    let playlist_index: usize = playlist_index
        .try_into()
        .expect("playlist_index really shouldn't bhe greater than max of usize");

    Ok(playlists.into_iter().nth(playlist_index - 1))
}

async fn get_parent_playlist_tracks(
    spotify: &rspotify::AuthCodeSpotify,
    playlist_id: Option<SimplifiedPlaylist>,
    market: rspotify::model::Market,
) -> Result<Vec<data::BetterSavedTrack>> {
    let (is_liked_songs, playlist_id) = match playlist_id {
        Some(x) => (false, Some(x.id)),
        None => (true, None),
    };

    if is_liked_songs {
        let playlist = spotify.current_user_saved_tracks(Some(market));
        let playlist = playlist.collect::<Vec<ClientResult<SavedTrack>>>().await;
        let playlist: Vec<SavedTrack> = playlist
            .into_iter()
            .collect::<ClientResult<Vec<SavedTrack>>>()?;
        Ok(playlist
            .into_iter()
            .map(|x| data::BetterSavedTrack {
                added_at: x.added_at.timestamp(),
                track: x.track,
            })
            .collect())
    } else {
        let playlist = spotify
            .playlist_items(playlist_id.expect("playlist id should be type Some(SimplifiedPlaylist) since playlist isn't liked songs"), None, Some(market))
            .collect::<Vec<ClientResult<PlaylistItem>>>()
            .await;
        let playlist: Vec<PlaylistItem> = playlist
            .into_iter()
            .collect::<ClientResult<Vec<PlaylistItem>>>()?;
        Ok(playlist.into_iter().map(|x| {
            let added_at = if let Some(time) = x.added_at {
                time.timestamp()
            } else {
                -62167201438
            };
            let playable_item = x
                .track
                .expect("Should be able to get a track for current playlist.");
            let track: FullTrack;
            if let PlayableItem::Track(x) = playable_item {
                track = x;
            } else {
                panic!("We should only have tracks in the playlist. This program does not have the ability to sort Episodes");
            }
            data::BetterSavedTrack { added_at, track }
        }).collect())
    }
}

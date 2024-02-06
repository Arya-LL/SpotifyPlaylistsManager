use rspotify::{clients::BaseClient, model::FullTrack};
use serde::{Deserialize, Serialize};

use crate::misc_helpers;

use anyhow::{Context, Result};

pub struct BetterSavedTrack {
    pub added_at: i64,
    pub track: FullTrack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrimmedTrack {
    pub track_name: String,
    added_at: i64,
    duration: f32,
    explicit: bool,
    album_name: String,
    album_artists: Vec<String>,
    album_release_date: i64,
    artists: Vec<String>,
    acousticness: f32,
    danceability: f32,
    energy: f32,
    liveness: f32,
    mode: i32,
    speechiness: f32,
    valence: f32,
    sections_duration: Vec<f32>,
    sections_confidence: Vec<f32>,
    sections_loudness: Vec<f32>,
    sections_tempo: Vec<f32>,
    sections_tempo_confidence: Vec<f32>,
    sections_key: Vec<i32>,
    sections_key_confidence: Vec<f32>,
    sections_mode: Vec<i32>,
    sections_mode_confidence: Vec<f32>,
    sections_time_signature: Vec<i32>,
    sections_time_signature_confidence: Vec<f32>,
    segments_duration: Vec<f32>,
    segments_duration_confidence: Vec<f32>,
    segments_loudness_start: Vec<f32>,
    segments_loudness_max_time: Vec<f32>,
    segments_loudness_max: Vec<f32>,
    // For some reason an option in rspotify docs
    segments_pitches: Vec<Vec<f32>>,
    segments_timbre: Vec<Vec<f32>>,
    end_of_fade_in: f32,
    start_of_fade_out: f32,
    loudness: f32,
    tempo: f32,
    tempo_confidence: f32,
    //Should just need to be a u8
    time_signature: i32,
    time_signature_confidence: f32,
    //rspotify_model::audio::AudioAnalysisTrack
    //key is u32 in documentation
    key: u32,
    key_confidence: f32,
    mode_confidence: f32,
    codestring: String,
    code_version: f32,
    echoprintstring: String,
    echoprint_version: f32,
    synchstring: String,
    synch_version: f32,
    rhythmstring: String,
    rhythm_version: f32,
}

impl TrimmedTrack {
    /// Creates a new [`TrimmedTrack`].
    pub async fn new(
        spotify: &rspotify::AuthCodeSpotify,
        saved_track: BetterSavedTrack,
    ) -> Result<Self> {
        let track = saved_track.track;
        let analysis = spotify
            .track_analysis(track.id.clone().expect("Track should have track id"))
            .await
            .context("Error getting track analysis")?;
        let features = spotify
            .track_features(track.id.clone().expect("Track should have track id"))
            .await
            .context("Error getting track features")?;

        let best_track = TrimmedTrack {
            track_name: track.name,
            added_at: saved_track.added_at,
            duration: analysis.track.duration,
            explicit: track.explicit,
            album_name: track.album.name,
            album_artists: track.album.artists.into_iter().map(|x| x.name).collect(),
            album_release_date: misc_helpers::convert_to_parsable_date(
                track.album.release_date.unwrap_or("----".to_string()),
            ),
            artists: track.artists.into_iter().map(|x| x.name).collect(),
            acousticness: features.acousticness,
            danceability: features.danceability,
            energy: features.energy,
            liveness: features.liveness,
            mode: misc_helpers::convert_mode_to_int(features.mode),
            speechiness: features.speechiness,
            valence: features.valence,
            sections_duration: analysis
                .sections
                .iter()
                .map(|x| x.time_interval.duration)
                .collect(),
            sections_confidence: analysis
                .sections
                .iter()
                .map(|x| x.time_interval.confidence)
                .collect(),
            sections_loudness: analysis.sections.iter().map(|x| x.loudness).collect(),
            sections_tempo: analysis.sections.iter().map(|x| x.tempo).collect(),
            sections_tempo_confidence: analysis
                .sections
                .iter()
                .map(|x| x.tempo_confidence)
                .collect(),
            sections_key: analysis.sections.iter().map(|x| x.key).collect(),
            sections_key_confidence: analysis.sections.iter().map(|x| x.key_confidence).collect(),
            sections_mode: analysis
                .sections
                .iter()
                .map(|x| misc_helpers::convert_mode_to_int(x.mode))
                .collect(),
            sections_mode_confidence: analysis
                .sections
                .iter()
                .map(|x| x.mode_confidence)
                .collect(),
            sections_time_signature: analysis.sections.iter().map(|x| x.time_signature).collect(),
            sections_time_signature_confidence: analysis
                .sections
                .iter()
                .map(|x| x.time_signature_confidence)
                .collect(),
            segments_duration: analysis
                .segments
                .iter()
                .map(|x| x.time_interval.duration)
                .collect(),
            segments_duration_confidence: analysis
                .segments
                .iter()
                .map(|x| x.time_interval.confidence)
                .collect(),
            segments_loudness_start: analysis.segments.iter().map(|x| x.loudness_start).collect(),
            segments_loudness_max_time: analysis
                .segments
                .iter()
                .map(|x| x.loudness_max_time)
                .collect(),
            segments_loudness_max: analysis.segments.iter().map(|x| x.loudness_max).collect(),
            // For some reason an option in rspotify docs
            segments_pitches: analysis
                .segments
                .iter()
                .map(|x| x.pitches.clone())
                .collect(),
            segments_timbre: analysis.segments.iter().map(|x| x.timbre.clone()).collect(),
            end_of_fade_in: analysis.track.end_of_fade_in,
            start_of_fade_out: analysis.track.start_of_fade_out,
            loudness: analysis.track.loudness,
            tempo: analysis.track.tempo,
            tempo_confidence: analysis.track.tempo_confidence,
            time_signature: analysis.track.time_signature,
            time_signature_confidence: analysis.track.time_signature_confidence,
            key: analysis.track.key,
            key_confidence: analysis.track.key_confidence,
            mode_confidence: analysis.track.mode_confidence,
            codestring: analysis.track.codestring,
            code_version: analysis.track.code_version,
            echoprintstring: analysis.track.echoprintstring,
            echoprint_version: analysis.track.echoprint_version,
            synchstring: analysis.track.synchstring,
            synch_version: analysis.track.synch_version,
            rhythmstring: analysis.track.rhythmstring,
            rhythm_version: analysis.track.rhythm_version,
        };
        Ok(best_track)
    }
}

// pub async fn get_tracks_details(
//     spotify: &rspotify::AuthCodeSpotify,
//     motherlist: &[FullTrack],
// ) -> ClientResult<(
//     Vec<rspotify::model::AudioFeatures>,
//     Vec<rspotify::model::AudioAnalysis>,
// )> {
//     let features_list = join_all(
//         motherlist
//             .iter()
//             .map(|track| spotify.track_features(track.id.clone().expect("Should have track id")))
//             .collect::<Vec<_>>(),
//     )
//     .await
//     .into_iter()
//     .collect::<ClientResult<Vec<_>>>()?;
//     let analysis_list = join_all(
//         motherlist
//             .iter()
//             .map(|track| spotify.track_analysis(track.id.clone().expect("Should have track id")))
//             .collect::<Vec<_>>(),
//     )
//     .await
//     .into_iter()
//     .collect::<ClientResult<Vec<_>>>()?;
//     Ok((features_list, analysis_list))
// }
//
// pub fn trim_tracks(
//     spotify: &rspotify::AuthCodeSpotify,
//     motherlist: &[FullTrack],
//     seed_list: Vec<&rspotify::model::FullTrack>,
//     // mutual: bool,
//     // auto_handle_outliers: bool,
// ) {
//     let (train, test): (Vec<_>, Vec<_>) = motherlist
//         .clone()
//         .into_iter()
//         .partition(|track| seed_list.contains(&track));
//
//     //Turn the thingies into dataset with labeled features.
//
//     // To manual handle outliers, fit with only data above x% in gaussian clustering models around
//     // both modes
// }

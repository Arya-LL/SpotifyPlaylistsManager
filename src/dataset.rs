use std::{collections::HashMap, path::Path};

use burn::data::dataset::{Dataset, SqliteDataset, SqliteDatasetError, SqliteDatasetStorage};
use derive_new::new;

use anyhow::{Context, Result};

use crate::data_structs::{self, TrimmedTrack};

pub async fn write_to_db(
    motherlist: &[data_structs::TrimmedTrack],
    labels: &[Option<u32>],
) -> Result<()> {
    let labels_string: Vec<&str> = labels
        .iter()
        .map(|label| match label {
            Some(_) => "train",
            None => "test",
        })
        .collect();
    let items: Vec<(&str, &TrimmedTrack)> = motherlist
        .iter()
        .enumerate()
        .map(|(i, x)| (labels_string[i], x))
        .collect();
    let dataset = SqliteDatasetStorage::from_name("data/track_classification.db")
        .with_base_dir(Path::new("./"));
    // TODO: Find a better way to remove songs that were removed from liked songs than overwriting
    // the dataset and re-adding all songs.
    let mut writer = dataset
        .writer(true)
        .context("Error in opening database writer")?;

    items
        .into_iter()
        .map(|item| writer.write(item.0, item.1))
        .collect::<Result<Vec<usize>, SqliteDatasetError>>()
        .context("Error in writing to database")?;
    writer
        .set_completed()
        .context("Error in closing database writer")?;
    Ok(())
}

#[derive(new, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackClassificationItem {
    pub track: data_structs::TrimmedTrack, // The text for classification
    pub label: usize,                      // The label of the text (classification category)
}

pub trait TrackClassificationDatasetTrait: Dataset<TrackClassificationItem> {
    fn number_of_classes(&self, map: HashMap<usize, String>) -> usize;
    fn class_name(&self, label: usize, map: HashMap<usize, String>) -> String;
}

pub struct TrackClassificationDatasetStruct {
    dataset: SqliteDataset<TrackClassificationItem>,
}

impl Dataset<TrackClassificationItem> for TrackClassificationDatasetStruct {
    fn get(&self, index: usize) -> Option<TrackClassificationItem> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}

impl TrackClassificationDatasetStruct {
    pub fn train() -> Self {
        Self::new("train")
    }

    pub fn test() -> Self {
        Self::new("test")
    }

    pub fn new(split: &str) -> Self {
        Self {
            dataset: SqliteDataset::from_db_file("data/track_classification.db", split).unwrap(),
        }
    }
}

impl TrackClassificationDatasetTrait for TrackClassificationDatasetStruct {
    fn number_of_classes(&self, map: HashMap<usize, String>) -> usize {
        map.len()
    }

    fn class_name(&self, label: usize, map: HashMap<usize, String>) -> String {
        match map.get(&label) {
            Some(x) => x.to_string(),
            None => panic!("Invalid class id"),
        }
    }
}

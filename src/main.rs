pub mod account;
pub mod batcher;
pub mod data_structs;
pub mod dataset;
pub mod labels;
pub mod misc_helpers;
pub mod tokenizer;

use futures_util::future::join_all;

use anyhow::{Context, Result};

// #[derive(Debug)]
// enum CustomError {
//     ClientError(ClientError),
//     SqliteDatasetError(SqliteDatasetError),
// }

#[tokio::main]
async fn main() -> Result<()> {
    // All actions relating to account pre-analysis
    let (motherlist, labels) = account_details()
        .await
        .context("Error in the account details pre-analysis pipeline")?;
    //Create database
    database_pipeline(&motherlist, &labels)
        .await
        .context("Error in the database pipeline")?;

    // Print the members of motherlist
    motherlist
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("{i} {:?}", x.track_name));

    println!("{:?}", motherlist[0]);
    Ok(())
}

async fn account_details() -> Result<(Vec<data_structs::TrimmedTrack>, Vec<Option<u32>>)> {
    // Get user account
    let spotify = account::get_user_acct()
        .await
        .context("Error in account creation")?;
    // Get motherlist as a list of BetterSavedTrack
    let motherlist = account::get_motherlist(&spotify)
        .await
        .context("Error in getting motherlist")?;
    // Create sublists as a list of names for the sublists
    let sublists = labels::get_sublists().await;
    // Get labels for a subset of the motherlist - This becomes our training set
    let labels = labels::get_labels(sublists.as_slice(), &motherlist).await;
    // Restructure motherlist into a list of TrimmedTrack
    let motherlist: Vec<data_structs::TrimmedTrack> = join_all(
        motherlist
            .into_iter()
            .map(|track| data_structs::TrimmedTrack::new(&spotify, track)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>>>()
    .context("Error in trimming motherlist")?;
    Ok((motherlist, labels))
}

async fn database_pipeline(
    motherlist: &[data_structs::TrimmedTrack],
    labels: &[Option<u32>],
) -> Result<()> {
    dataset::write_to_db(motherlist, labels)
        .await
        .context("Error in creating/writing to database pipeline")?;
    Ok(())
}

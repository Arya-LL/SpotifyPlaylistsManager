use rand::seq::SliceRandom;

use crate::data_structs as data;

// pub async fn create_db(
//     motherlist: Vec<data::BetterSavedTrack>,
//     spotify: &rspotify::AuthCodeSpotify,
// ) {
//     let motherlist_trimmed = join_all(
//         motherlist
//             .into_iter()
//             .map(|track| data::TrimmedTrack::new(spotify, track)),
//     )
//     .await;
// }

pub async fn get_sublists() -> Vec<String> {
    println!("We will now create the subplaylists from the parent playlist. Currently only allows for mutually exclusive sublists.");

    let mut sublists = vec![];

    println!("Input the name of a subplaylist. Note that there is no check for if you have
        already created a subplaylist with the same name. Input 0 if you are done creating subplaylists.");
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Invalid input in creating sublists");

        match input.trim() {
            "0" => break,
            sublist_name => {
                if sublists.contains(&sublist_name.to_string()) {
                    println!("Sublists already contains {:?}", sublist_name);
                    continue;
                } else {
                    sublists.push(sublist_name.to_owned());
                };
            }
        };

        println!("Current subplaylists:");
        sublists.iter().for_each(|x| print!("{:?}, ", x));
        println!("\n");
    }
    sublists
}

pub async fn get_labels(
    sublists: &[String],
    motherlist: &[data::BetterSavedTrack],
) -> Vec<Option<u32>> {
    let mut labels: Vec<Option<u32>> = vec![None; motherlist.len()];

    println!("Now printing songs from the motherlist. For each song, please categorize the song into one of your provided subplaylists by typing the number corresponding to the selected subplaylist. This allows us to create seeds for the subplaylists, the more songs you categorize now will result in more accurate subplaylists. Input 0 when you are done seeding songs.");

    println!("0 break");
    sublists
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("{} {:?}", i + 1, x));

    let mut rng = rand::thread_rng();
    let mut randomized_motherlist: Vec<(usize, String, Vec<rspotify::model::SimplifiedArtist>)> =
        motherlist
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x.track.name.clone(), x.track.artists.clone()))
            .collect::<Vec<(usize, String, Vec<rspotify::model::SimplifiedArtist>)>>();
    randomized_motherlist.shuffle(&mut rng);
    let mut randomized_motherlist = randomized_motherlist.into_iter();
    let mut past_indices: Vec<usize> = vec![];

    'outer: loop {
        let (index, track, artists) = match randomized_motherlist.next() {
            Some(x) => x,
            None => {
                println!("No more tracks in motherlist");
                break;
            }
        };
        let artists: Vec<String> = artists.into_iter().map(|x| x.name).collect();

        println!(
            "Song: {:?} \t Artists: {:?}\n Input corresponding subplaylist number:",
            track, artists
        );

        let mut subplaylist_index: i32;
        'inner: loop {
            let mut subplaylist_index_string = String::new();
            std::io::stdin()
                .read_line(&mut subplaylist_index_string)
                .expect("Invalid input for subplaylist number/index");
            subplaylist_index = subplaylist_index_string.trim().parse().unwrap_or(-1);

            if subplaylist_index < 0
                || std::convert::TryInto::<usize>::try_into(subplaylist_index)
                    .expect("subplaylist_index really shouldn't be greater than max of usize")
                    > sublists.len()
            {
                println!("Please enter a number corresponding to one of the subplaylists.");
                continue 'inner;
            }
            break 'inner;
        }
        if subplaylist_index == 0 {
            break 'outer;
        }
        let subplaylist_index: u32 = subplaylist_index
            .try_into()
            .expect("subplaylist_index really shouldn't be greater than max of u32");

        labels[index] = Some(subplaylist_index);
        past_indices.push(index);
    }
    labels
}

//! Pick from a pool of music
//!
//! Would like to use AI generated music or scrape from external site in the future

use std::{collections::HashMap, path::Path};

use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Theme {
    Funky,
    Calm,
    Dark,
    Inspirational,
    Bright,
    Dramatic,
    Happy,
    Romantic,
    Angry,
    Sad,
}

pub fn choose_music(theme: Theme) -> String {
    let list = MUSIC_DIR.get(&theme).unwrap();
    let rand = list.choose(&mut rand::thread_rng()).unwrap();
    rand.to_string()
}

/// Fetch music from youtube
pub fn fetch_music(id: &str, path: &Path) -> anyhow::Result<()> {
    use rustube::blocking::Video;

    println!("Fetching music file {id}...");

    let url = Url::parse(&format!("https://youtube.com/watch?v={id}"))?;
    let video = Video::from_url(&url)?
        .worst_audio()
        .unwrap()
        .blocking_download_to(path)?;
    Ok(())
}

// URLS fetched using
// youtube-dl -j --flat-playlist <playlist-url> | jq -r '.id'
lazy_static! {
    pub static ref MUSIC_DIR: HashMap<Theme, Vec<&'static str>> = HashMap::from_iter([
        (
            Theme::Funky,
            vec![
                "9QUiMHKSN9c",
                "n92YHRZfmDc",
                "y7SoofrR8gA",
                "YbkTBBw8W8Y",
                "baSBfbzonpo",
                "HAI7UVQYKB8",
                "PZW5uzs99Es",
                "EhxhBCtI4gg",
                "Kt65oudi8Rg",
                "VgQReV0vM6M",
                "t3XLnVbVmD4",
                "T9Ui3VoXFKI",
                "DBeoI3ZnXmo",
                "DIz0MOg-VFs",
                "KiHz1DmOXZQ",
                "kDJFuL7EEJE",
                "7IVLEShXmBA",
                "_7T65r1tTJk",
                "YuI198efaHQ",
                "VWVk0YMGGmc",
            ]
        ),
        (
            Theme::Calm,
            vec![
                "uhQ3A8veER0",
                "MBO-4R-zGso",
                "qNVM0dxfkWQ",
                "sZRrUKvl55g",
                "s5RPcgai9dI",
                "oQpSRVIHg3w",
                "nwerJAOnYMI",
                "OBGcZZ3ClvA",
                "iiWWxKhfdnk",
                "kBrYtfH3h80",
                "ywv6s9xWldg",
                "iVvwttTdijE",
                "z-KjLrgO4us",
                "pea4UEmtCYE",
                "0yGubCCHaPs",
                "H3OwYWxyuUE",
                "m1upXcAYwmU",
                "WzlRZ5lQ7C4",
                "Qk2lZGofqsg",
                "TpeNDpRCMvg",
            ]
        ),
        (
            Theme::Dark,
            vec![
                "k5LI6jS8Sqs",
                "VzjOhVZ0CXQ",
                "3kTGwquBU1A",
                "Oq7HlmUJi7Q",
                "JAFyH1spRpA",
                "ikAMp1CDQQQ",
                "ViU423hHFFE",
                "hK0mSJ8tNkk",
                "KF30t9wTtC8",
                "PzPaDGbA1zc",
                "_PBNMGEwZSg",
                "SCFL3VQbuPw",
                "uz-EEcRPBbg",
                "bCQnOHopAY4",
                "oZ25z5bb6ww",
                "w1L0QS_3ZOc",
                "XXv95fI-M1w",
                "7LRM_o90tAw",
                "NIZHh-XvoaY",
                "4nyqH3UrQx8",
            ]
        ),
        (
            Theme::Inspirational,
            vec![
                "KnxDbIEoDTU",
                "tZq54IDY17Q",
                "r_f7_HKDw7Y",
                "whvw4lpqjM0",
                "oi1hxQLZbco",
                "tCwTe1-2Rng",
                "W-zkQU7Vhxg",
                "nqqBL8anNw8",
                "i5u0-tZWR8s",
                "6PEFzSC0jwM",
                "rfHKMtw4sCQ",
                "qVZdWz5G_IY",
                "X_cMQVLw-LU",
                "cYpV3M8qJz4",
                "FycIV0JwrTk",
                "IPCbVCpt5yQ",
                "jryVFKQhhgQ",
                "AnR00vCm16A",
                "nqb-dvXGp9k",
                "4P0prQ3PfBM",
            ]
        ),
        (
            Theme::Bright,
            vec![
                "f4bVH0hpK8A",
                "j-fs-K3duJg",
                "9zhJFFSJdUU",
                "_uqq9doeZfM",
                "onIWoZzpp7I",
                "iQNK4ouvFHI",
                "_DpJL1zjoL8",
                "LQeHUpeQKqY",
                "VI3d-yJYc6E",
                "erH3wPjbUVY",
                "oAyaov9jxrc",
                "7uqlyljQEPI",
                "EKOf9y9oMOw",
                "jn2UONZqdr8",
                "wiHzWWTCGUI",
                "00I6JCz5tvI",
                "CU7Sp7fw1yk",
                "bw1McglAaVE",
                "SKX0OMOpaWM",
                "ckPRIzSW1Q0",
            ]
        ),
        (
            Theme::Dramatic,
            vec![
                "aKRcGzwv62k",
                "7CII5tjHxAg",
                "mzyT25aIuD4",
                "Zt-6cWFuV9g",
                "D7-1Wy9pu64",
                "SXmP3FQs_yA",
                "vg9Y_NIZcmw",
                "kAfp9id_8gU",
                "IBpxl8JPcRs",
                "9eJgfUIt_UE",
                "MYOxO-aDgY0",
                "84v6cBcBBrc",
                "cssw_UsTrlQ",
                "AHLjnmrSxeY",
                "zE7wax709yA",
                "UUnVg1XL37Y",
                "v02ADiWwRk4",
                "PWC9bZRtqwA",
                "z9I8JIglSHU",
                "TsZwXPqjYxk",
            ]
        ),
        (
            Theme::Happy,
            vec![
                "fPixG4D6C6k",
                "F3D0oUsvHaw",
                "AzR5zzgdPXA",
                "F4kT9I7Sxvc",
                "DtgE378d4-U",
                "fkO1ULW19HA",
                "tAwdHxTYoM0",
                "38f4npAt9vM",
                "t3PisI5aXzY",
                "Eq67aA37ssA",
                "ZZRmT5Kfb5o",
                "1r7wQ2OMDWQ",
                "iD0hx-J7W3o",
                "xb8i9Plfk4g",
                "tooWGwP6Suo",
                "6NRtkIx6uaA",
                "6LDW004WVWk",
                "kczqJpn_DXY",
                "Okve5wSmg-s",
                "3EpfC2DYBPk",
            ]
        ),
        (
            Theme::Romantic,
            vec![
                "oN8virP-lFw",
                "KiBN8nnDn0c",
                "30_khq4EFTo",
                "pTOVmsPJIBY",
                "rYDcNqGIrRc",
                "THlyBNBaUy8",
                "JPr5f3BbtVc",
                "n6WRJ19QoY0",
                "7MXceJiyrOo",
                "NH84qD29Mhk",
                "jyJuUStGt_s",
                "1xZcAMO5xHU",
                "KVQKUQ2T28k",
                "cDyn5iKDChI",
                "O6vPLO6G77o",
                "RIxOutyM1W4",
                "Go5_JTCEV04",
                "YzJSuvi0vSw",
                "DV5FImAhxIU",
                "O0kaHmWPk3o",
            ]
        ),
        (
            Theme::Angry,
            vec![
                "YN9S5ecYmFw",
                "y46bPfa6xSY",
                "UuuS8APqhPg",
                "k_IYUFm5zIs",
                "Vssj3yL3WkM",
                "U9233EHL5d0",
                "as_juMSBEu0",
                "Ry9O60AAROw",
            ]
        ),
        (
            Theme::Sad,
            vec![
                "liwlFBDCo4c",
                "RThxmffrwIo",
                "PWRycHMM5ho",
                "rpwSr6z0v4k",
                "UWS7uMBZsJs",
                "ZuAMtopeI20",
                "v5b5juZL2CI",
                "XHE0Rpx7kV8",
                "wjbMpkvJ3uk",
                "AyVcSAaJlH0",
                "YPWcX1hCetc",
                "5PDYej1APkM",
                "PmtIs8E1Dhs",
                "ht0aCC1blxY",
            ]
        ),
    ]);
}

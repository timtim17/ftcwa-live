use phf::phf_map;
use once_cell::sync::Lazy;

pub static STREAMS: Lazy<&'static phf::Map<&'static str, &'static str>> = Lazy::new(|| {
    ALL_STREAMS.get("2026").expect("invalid year")
});

pub const ALL_STREAMS: phf::Map<&'static str, phf::Map<&'static str, &'static str>> = phf_map! {
    "2024" => phf_map! {
        "bardeen" => "https://www.youtube.com/watch?v=AfZ_z4BQZuk",
        "bardeen1" => "https://www.youtube.com/watch?v=W7ZDLTJKp5c",
        "salk" => "https://www.youtube.com/watch?v=yPoC3pbuk9o",
        "salk1" => "https://www.youtube.com/watch?v=yyXZ8Q5Va2E",
        "maxwell" => "https://www.youtube.com/watch?v=aj2QMS5zFaM",
        "maxwell1" => "https://www.youtube.com/watch?v=EnBG9x9p658",
        "turing" => "https://www.youtube.com/watch?v=HAAQTXrOZw4",
        "turing1" => "https://www.youtube.com/watch?v=U85i6WwwAJA",
        "knuth" => "https://www.youtube.com/watch?v=10XjXGPzAyE",
        "spencer" => "https://www.youtube.com/watch?v=zHW5F4EthHU",
        "spencer1" => "https://www.youtube.com/watch?v=oLkX1fAEXF4",
        "brattain" => "https://www.youtube.com/watch?v=pm6gNIm4dnU",
        "brattain1" => "https://www.youtube.com/watch?v=dFUwrtv--G4",
        "ritchie1" => "https://www.youtube.com/watch?v=LSgT-zcOUyo",
        "feynman" => "https://youtube.com/live/2QSo1Zb2Xo4",
        "tesla" => "https://youtube.com/live/3ppW3ZNnmDA?",
        "hawking" => "https://youtube.com/live/Me98sMD5KBs",
        "pasteur" => "https://youtube.com/live/pzwmqGg6mT4",
        "watt" => "https://youtube.com/live/UkmZnJgWUFA",
        "state" => "https://www.youtube.com/watch?v=xq-7jn_oMpU",
        "wyoming" => "https://youtube.com/live/-_atRRrqvxI",
    },
    "2025" => phf_map! {
        "asimov" => "https://www.youtube.com/watch?v=bk6L7yV46k0",
        "asimov-t" => "https://www.twitch.tv/firstwa_red1",
        "asimov-playlist" => "https://www.youtube.com/watch?v=bk6L7yV46k0&list=PLoRnKfyWNUldjdc7Q5dTY1HaFbbYtuD9I&pp=gAQB",
        "bardeen" => "https://youtube.com/live/clSg1FAUI7k",
        "bardeenlm1" => "https://www.youtube.com/watch?v=F0caNUelJS4",
        "bardeenlm2" => "https://www.youtube.com/watch?v=clSg1FAUI7k",
        "brattain" => "https://www.youtube.com/live/MDpf4HXiNdc",
        "brattainlm1" => "https://www.youtube.com/watch?v=euo4xfI06_g",
        "brattainlm2" => "https://www.youtube.com/live/MDpf4HXiNdc",
        "capek" => "https://www.youtube.com/watch?v=xouJCgLDhf0",
        "capek-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlcUXUUZUjdmNjZyhjg_VQnI",
        "celtic" => "https://www.youtube.com/watch?v=Yb4IY3b8VuI",
        "celtic-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlfH1l2Zs9_DNgY3qHeIU4ul",
        "coral-ytplaylist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUleDroiOiFaxPXGPPqZJ8Fvr",
        "feynman" => "https://youtube.com/live/bh76PN2sC0c",
        "feynman-yt" => "https://youtube.com/live/bh76PN2sC0c",
        "hdti" => "https://www.youtube.com/watch?v=GntVMSsTzsA",
        "hdti-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUldsknJNSwmuNzUKiLRv3AyG",
        "hawking" => "https://www.twitch.tv/firstwa_red1",
        "hawking-yt" => "https://youtube.com/live/4p14r6YbN8o",
        "ia-black-fri" => "https://www.youtube.com/watch?v=JzlM84IoIBg",
        "ia-black-sat" => "https://www.youtube.com/watch?v=vBGyGGZR5NU",
        "ia-black-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUldcH_NaPdqcH_bZoS8kLBy1",
        "ia-gold-fri" => "https://www.youtube.com/watch?v=f16mK4Gjr_M",
        "ia-gold-sat" => "https://www.youtube.com/watch?v=CKqRFk1nX2o",
        "ia-gold-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlfLgKWo1HOnkuN0KzmORCxT",
        "knuth" => "https://youtube.com/live/7N3Dam5n7HI",
        "knuthlm1" => "https://www.youtube.com/watch?v=KP2g5VyZMlg",
        "knuthlm2" => "https://www.youtube.com/watch?v=7N3Dam5n7HI",
        "lamarr" => "https://youtube.com/live/wZzUKAND7ys",
        "lamarrlm1" => "https://www.youtube.com/watch?v=B58rLrp_758",
        "lamarrlm2" => "https://www.youtube.com/watch?v=wZzUKAND7ys",
        "maxwell" => "https://youtube.com/live/Yp1pBfummqs",
        "maxwelllm1" => "https://youtube.com/watch?v=6jJjmvKK8FM",
        "maxwelllm2" => "https://www.youtube.com/watch?v=Yp1pBfummqs",
        "noddack" => "https://youtube.com/live/ZxRBlrI4GrU",
        "noddacklm1" => "https://www.youtube.com/watch?v=_HL5ZwIVgvw",
        "noddacklm2" => "https://youtube.com/live/ZxRBlrI4GrU?",
        "pasteur" => "https://youtube.com/live/5qkFrx-GZxY",
        "pasteur-yt" => "https://youtube.com/live/5qkFrx-GZxY",
        "ritchie" => "https://youtube.com/live/jZ35QGR-OG0",
        "ritchielm1" => "https://www.youtube.com/watch?v=Ph283fdDomo",
        "ritchielm2" => "https://youtube.com/live/jZ35QGR-OG0",
        "salk" => "https://youtube.com/live/6S86EBSxchU",
        "salklm1" => "https://www.youtube.com/watch?v=4Pp_X0RtDBI",
        "salklm2" => "https://www.youtube.com/watch?v=6S86EBSxchU",
        "spencer" => "https://youtube.com/live/8BBjhqQobeo",
        "spencerlm1" => "https://www.youtube.com/watch?v=PwI7hqNdWVg",
        "spencerlm2" => "https://youtube.com/live/8BBjhqQobeo",
        "state" => "https://youtube.com/live/YyaL8DwY2kE",
        "state-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlfikcHBOPWZ0ff7NCxGksx0",
        "tesla" => "https://youtube.com/live/2wFgknxFL2o",
        "tesla-yt" => "https://youtube.com/live/2wFgknxFL2o",
        "tesla-rankings" => "https://youtube.com/live/28cmxegu55Q",
        "turing" => "https://youtube.com/live/AZWak1GDx4o",
        "turinglm1" => "https://www.youtube.com/watch?v=4Pp_X0RtDBI",
        "turinglm2" => "https://youtube.com/live/AZWak1GDx4o",
        "wu" => "https://www.twitch.tv/firstwa_red1",
        "wu-yt" => "https://youtube.com/live/hE8VfHFS3lc",
        "edison" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlcuiB64Ybl7bG8newLU2KKs",
        "franklin" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlf2aV6J8F1Oy7XdV65VzO2n",
        "jemison" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlcvcy1-BOwFUpcy_vGp2ihD",
        "ochoa" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlctdFDJQR_IhFTenM5PjxuF",
        "cri-stream" => "https://www.youtube.com/watch?v=-zaWigr8KFE&list=PLG-GBCx5_RF_CsV4-Egb7riZFWF6ZaOie&index=1&pp=iAQB",
        "cri-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlcnrYyst4KnrLRC2oyqZioe"
    },
    "2026" => phf_map! {
        //Weekend 1
        "bardeen-lm1" => "https://www.youtube.com/watch?v=3mtZiUD4zzo",
        "noddack-lm1" => "https://youtube.com/live/UN1wMNuX9IU",
        "salk-lm1" => "https://youtube.com/live/CdzKeENztQs",
        "turing-lm1" => "https://youtube.com/live/Ixm2h2kHJSM",
        "knuth-lm1" => "https://youtube.com/live/Zk3hcROvo1g",
        //Weekend 2
        "lamarr-lm1" => "https://youtube.com/live/Ho9TGeFBlRE",
        "spencer-lm1" => "https://youtube.com/live/J5FwcDN7Mhs",
        "maxwell-lm1" => "https://www.youtube.com/watch?v=H8AD_ESdLRI",
        "ritchie-lm1" => "https://youtube.com/live/tz-MMIJmUw8",
        //Weekend 3
        "bardeen" => "https://youtube.com/live/34eeydtEEmA",
        "salk" => "https://youtube.com/live/iMan_bXS3NA",
        "noddack" => "https://youtube.com/live/HZVQrcbPkzI",
        "knuth" => "https://youtube.com/live/sbcvMd0L-_4",
        //Weekend 4
        "lamarr" => "https://youtube.com/live/5pqmXWgNoeE",
        "spencer" => "https://youtube.com/live/ZXHvLoKte-E",
        "ritchie" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlcC7MLtZsgo8zmw3ibySugi",
        "turing" => "https://youtube.com/live/nFcR4bpoDgY",
        "brattain" => "https://youtube.com/live/A9Ub3_RHDSk",
        //League Tournaments
        "feynman" => "https://youtube.com/live/ZTbUW9LGbyE",
        "feynman-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUldy1dJOnpY0PkgZvi5bicWb",
        "pasteur" => "https://youtube.com/live/G4AB_A9n1Ls",
        "pasteur-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlcr5ChZIozJH0o2jDUnFZCX",
        "wu" => "https://youtube.com/live/h6wvIfAQdYA",
        "wu-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUldTWZN1JY-eFgWdUzhLP-mP",
        "tesla" => "https://youtube.com/live/4_H0Xhz3T10",
        "tesla-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlfJY0O2uVBTrOaZpv7gdzqk",
        "hawking" => "https://youtube.com/live/ypZQyM435qE",
        "hawking-playlist" => "https://www.youtube.com/playlist?list=PLoRnKfyWNUlcKmhJUSjLWUUdGA3AwqXEe",
        "watt" => "https://youtube.com/live/8lqLxOWAmvQ",
        //Super Qualifiers
        "asimov" => "https://youtube.com/live/c0wFbhRlusE",
        "capek" => "https://youtube.com/live/uQlvwiwokMo",
        //State
        "state" => "https://youtube.com/live/nKypRFORACw"

    },
};

macro_rules! generate_links {
    (@boilerplate) => {
        "<!doctype html><style>
                body {
                    font-family: sans-serif;
                    color: #14171a;
                    margin: 60px auto;
                    max-width: 95%;
                }

                a, a:link {
                    color: #141414;
                    line-height: 1.5;
                    transition: color 500ms;
                    text-decoration: none;
                    border-bottom: 1px dotted;
                }

                @media (prefers-color-scheme: dark) {
                    body {
                        /* borrowed from whattheref.info,
                         * which borrowed from vulpes.one's gemini proxy
                         */
                        color: #cad1d8;
                        background-color: #14171a;
                    }

                    a, a:link {
                        color: #eee;
                    }
                }

                a:focus, a:hover {
                    color: #F57E25;
                }

                h1 {
                    margin: 0 0 8px;
                }

                @media screen and (min-width: 700px) {
                    body {
                        max-width: 650px;
                    }
                }
            </style><body>"
    };
    (@links, $(($name:expr, $url:expr)),*) => {
        concat!($(
            concat!("<a href=\"", $url, "\">", $name, "</a><br>")
        ),*)
    };
    ($(($name:expr, $url:expr)),*) => {
        concat!(
            generate_links!(@boilerplate),
            generate_links!(@links, $(($name, $url)),*),
            "</body>"
        )
    };
    ($title:expr, $(($name:expr, $url:expr)),*) => {
        concat!(
            generate_links!(@boilerplate),
            "<h1>", $title, "</h1>",
            generate_links!(@links, $(($name, $url)),*),
            "</body>"
        )
    };
}

pub const STATIC_PAGES: phf::Map<&'static str, &'static str> = phf_map! {
    "2024" => generate_links!(
        "Centerstage - 2023/24",
        ("Bardeen", "/2024/bardeen"),
        ("Salk", "/2024/salk"),
        ("Maxwell", "/2024/maxwell"),
        ("Turing", "/2024/turing"),
        ("Knuth", "/2024/knuth"),
        ("Spencer", "/2024/spencer"),
        ("Brattain", "/2024/brattain"),
        ("Ritchie", "/2024/ritchie"),
        ("Feynman", "/2024/feynman"),
        ("Tesla", "/2024/tesla"),
        ("Hawking", "/2024/hawking"),
        ("Pasteur", "/2024/pasteur"),
        ("Watt", "/2024/watt"),
        ("Washington State Championship", "/2024/state"),
        ("Wyoming State Championship", "/2024/wyoming")
    ),
    "2025" => generate_links!(
        "Into The Deep - 2024/25",
        ("Asimov", "/itd/semis/asimov"),
        ("Bardeen", "/2025/bardeen"),
        ("Brattain", "/2025/brattain"),
        ("Capek", "/itd/semis/capek"),
        ("Feynman", "/2025/feynman"),
        ("High Definition Tech Invitation (HDTI)", "/2025/hdti"),
        ("Hawking", "/2025/hawking"),
        ("Knuth", "/2025/knuth"),
        ("Lamarr", "/2025/lamarr"),
        ("Maxwell", "/2025/maxwell"),
        ("Noddack", "/2025/noddack"),
        ("Pasteur", "/2025/pasteur"),
        ("Ritchie", "/2025/ritchie"),
        ("Salk", "/2025/salk"),
        ("Spencer", "/2025/spencer"),
        ("Tesla", "/2025/tesla"),
        ("Turing", "/2025/turing"),
        ("Wu", "/2025/wu"),
        ("Washington State Championship", "itd/wa/state"),
        ("Chicago Robotics Invitiational", "itd/cri")
    ),
    "2026" => generate_links!(
        "FTC Washington Decode Streams",
        ("League Meet 1 - Events Finished. Recording found here", "/decode/lm1"),
        ("League Meet 2 - Events Finished. Recording found here", "/decode/lm2"),
        ("League Tournaments - In Progress", "/decode/lt"),
        ("Super Qualifiers - Coming Soon", "/decode/sq"),
        ("Washington State Championship - Coming Soon", "/decode/state")
    ),
    // Copy of "2026"
    "decode" => generate_links!(
        "FTC Washington Decode Streams",
        ("League Meet 1 - Events Finished. Recording found here", "/decode/lm1"),
        ("League Meet 2 - Events Finished. Recording found here", "/decode/lm2"),
        ("League Tournaments - In Progress", "/decode/lt"),
        ("Super Qualifiers - Coming Soon", "/decode/sq"),
        ("Washington State Championship - Coming Soon", "/decode/state")
    ),
    "decode/lm1" => generate_links!(
        "FTC Washington Decode Streams - League Meet 1",
        ("Bardeen League Meet 1", "/2026/bardeen-lm1"),
        ("Salk League Meet 1", "/2026/salk-lm1"),
        ("Knuth League Meet 1", "/2026/knuth-lm1"),
        ("Noddack League Meet 1", "/2026/noddack-lm1"),
        ("Turing League Meet 1", "/2026/turing-lm1"),
        ("Lamarr League Meet 1", "/2026/lamarr-lm1"),
        ("Spencer League Meet 1", "/2026/spencer-lm1"),
        ("Maxwell League Meet 1", "/2026/maxwell-lm1"),
        ("Richie League Meet 1", "/2026/richie-lm1")
    ),
    "decode/lm2" => generate_links!(
        "FTC Washington Decode Streams - League Meet 2",
        ("Bardeen League Meet 2", "/2026/bardeen"),
        ("Salk League Meet 2", "/2026/salk"),
        ("Noddack League Meet 2", "/2026/noddack"),
        ("Knuth League Meet 2", "/2026/knuth"),
        ("Lamarr League Meet 2", "/2026/lamarr"),
        ("Ritchie League Meet 2 - (NOTE: Broken due to venue errors)", "/2026/ritchie"),
        ("Turing League Meet 2", "/2026/turing"),
        ("Brattain League Meet 2", "/2026/brattain")
    ),
    "decode/lt" => generate_links!(
        "FTC Washington Decode Streams - League Tournaments",
        ("Feynman League Tournament Stream (12/6)", "/2026/feynman"),
        ("Feynman - Match Playlist", "/2026/feynman-playlist"),
        ("Pasteur League Tournament Stream (12/7)", "/2026/pasteur"),
        ("Pasteur - Match Playlist", "/2026/pasteur-playlist"),
        ("Wu League Tournament Stream (12/13)", "/2026/wu"),
        ("Wu - Match Playlist", "2026/wu-playlist"),
        ("Tesla League Tournament Stream (12/14)", "/2026/tesla"),
        ("Tesla - Match Playlist", "2026/tesla-playlist"),
        ("Hawking League Tournament Stream (12/14)", "/2026/hawking"),
        ("Hawking - Match Playlist", "2026/hawking-playlist"),
        ("Watt League Tournament Stream (12/20)", "/2026/watt")
    ),
    "decode/sq" => generate_links!(
        "FTC Washington Decode Streams - Super Qualifiers",
        ("Asimov Super Qualifier (Saturday)", "/2026/asimov"),
        ("Capek Super Qualifer (Sunday)", "/2026/capek")
    ),
    "decode/state" => generate_links!(
        "FTC Washington Decode Streams - State Championship",
        ("Washington State Championship (Stream)", "/2026/state")
    ),
    "itd" => generate_links!(
        "Into The Deep - 2024/25",
        ("Asimov", "/itd/semis/asimov"),
        ("Bardeen", "/2025/bardeen"),
        ("Brattain", "/2025/brattain"),
        ("Capek", "/itd/semis/capek"),
        ("Feynman", "/2025/feynman"),
        ("High Definition Tech Invitation (HDTI)", "/2025/hdti"),
        ("Hawking", "/2025/hawking"),
        ("Knuth", "/2025/knuth"),
        ("Lamarr", "/2025/lamarr"),
        ("Maxwell", "/2025/maxwell"),
        ("Noddack", "/2025/noddack"),
        ("Pasteur", "/2025/pasteur"),
        ("Ritchie", "/2025/ritchie"),
        ("Salk", "/2025/salk"),
        ("Spencer", "/2025/spencer"),
        ("Tesla", "/2025/tesla"),
        ("Turing", "/2025/turing"),
        ("Wu", "/2025/wu"),
        ("Washington State Championship", "itd/wa"),
        ("Chicago Robotics Invitiational", "itd/cri")
    ),
    "itd/asimov" => generate_links!(
        "Asmiov Semifinal",
        ("YouTube", "/2025/asimov"),
        ("YouTube Playlist with Individual Matches", "/2025/asimov-playlist"),
        ("FTC Events", "https://ftc-events.firstinspires.org/2024/USWAMVSQ1")
    ),
    "itd/capek" => generate_links!(
        "Capek Semifinal",
        ("YouTube", "/2025/capek"),
        ("YouTube Playlist with Individual Matches", "/2025/capek-playlist"),
        ("FTC Events", "https://ftc-events.firstinspires.org/2024/USWAMVSQ2")
    ),
    "itd/wa" => generate_links!(
        "Washington Championship",
        ("YouTube", "/2025/state"),
        ("YouTube Playlist with Individual Matches", "/2025/state-playlist"),
        ("FTC Events", "https://ftc-events.firstinspires.org/2024/USWACMP")
    ),
    "itd/celtic" => generate_links!(
        "Iowa Celtic Sea League Tournament",
        ("Celtic League Tournament - YouTube Stream", "/celtic"),
        ("Celtic League Tournament YouTube Playlist with Individual Matches", "/celtic-playlist"),
        ("Celtic League Tournament - FTC Iowa", "https://www.youtube.com/watch?v=IYCVKrod6ds")
    ),
    "itd/iowa" => generate_links!(
        "Iowa Championship",
        ("Black Division - Friday", "/ia-black-fri"),
        ("Black Division - Saturday", "/ia-black-sat"),
        ("Black Division - Match Playlist", "/ia-black-playlist"),
        ("Gold Division - Friday", "/ia-gold-fri"),
        ("Gold Division - Saturday", "/ia-gold-sat"),
        ("Gold Division - Match Playlist", "/ia-gold-playlist")
    ),
    "itd/cri" => generate_links!(
        "Chicago Robotics Invitational",
        ("Official Stream", "/cri-stream"),
        ("Official Match Playlist", "/cri-playlist")
    )
};
pub(crate) use generate_links; 
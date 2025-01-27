use phf::phf_map;
use once_cell::sync::Lazy;

pub static STREAMS: Lazy<&'static phf::Map<&'static str, &'static str>> = Lazy::new(|| {
    ALL_STREAMS.get("2025").expect("invalid year")
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
        "capek-t" => "https://www.twitch.tv/firstwa_red1",
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
        "state-t" => "https://www.twitch.tv/firstwa_red1",
        "tesla" => "https://youtube.com/live/2wFgknxFL2o",
        "tesla-yt" => "https://youtube.com/live/2wFgknxFL2o",
        "tesla-rankings" => "https://youtube.com/live/28cmxegu55Q",
        "turing" => "https://youtube.com/live/AZWak1GDx4o",
        "turinglm1" => "https://www.youtube.com/watch?v=4Pp_X0RtDBI",
        "turinglm2" => "https://youtube.com/live/AZWak1GDx4o",
        "wu" => "https://www.twitch.tv/firstwa_red1",
        "wu-yt" => "https://youtube.com/live/hE8VfHFS3lc",
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
        ("State", "/2024/state"),
        ("Wyoming", "/2024/wyoming")
    ),
    "2025" => generate_links!(
        ("Asimov", "/2025/asimov"),
        ("Bardeen", "/2025/bardeen"),
        ("Brattain", "/2025/brattain"),
        ("Capek", "/2025/capek"),
        ("Feynman", "/2025/feynman"),
        ("HDTI", "/2025/hdti"),
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
        ("Wu", "/2025/wu")
    ),
    "semis/asimov" => generate_links!(
        "Asmiov Semifinal",
        ("YouTube", "/asimov"),
        ("Twitch", "/asimov-t"),
        ("YouTube Playlist with Individual Matches", "/asimov-playlist"),
        ("FTC Events", "https://ftc-events.firstinspires.org/2024/USWAMVSQ1"),
        ("Event Details", "https://firstwa.org/event/ftcasimov/")
    ),
    "semis/capek" => generate_links!(
        "Capek Semifinal",
        ("YouTube", "/capek"),
        ("Twitch", "/capek-t"),
        ("YouTube Playlist with Individual Matches", "/capek-playlist"),
        ("FTC Events", "https://ftc-events.firstinspires.org/2024/USWAMVSQ2"),
        ("Event Details", "https://firstwa.org/event/ftccapek/")
    ),
    "wa/state" => generate_links!(
        "Washington Championship",
        ("YouTube", "/state"),
        ("Twitch", "/state-t"),
        ("YouTube Playlist with Individual Matches", "/state-playlist"),
        ("FTC Events", "https://ftc-events.firstinspires.org/2024/USWACMP"),
        ("Event Details", "https://firstwa.org/event/ftc-wachamps/")
    ),
    "iowa/celtic" => generate_links!(
        "Iowa Celtic Sea League Tournament",
        ("Celtic League Tournament - YouTube Stream", "/celtic"),
        ("Celtic League Tournament YouTube Playlist with Individual Matches", "/celtic-playlist"),
        ("Celtic League Tournament - FTC Iowa", "https://www.youtube.com/watch?v=IYCVKrod6ds")
    )
};

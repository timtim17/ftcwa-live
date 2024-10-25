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
        "ritchie" => "https://youtube.com/live/Ph283fdDomo",
        "ritchielm1" => "https://youtube.com/live/Ph283fdDomo",
        "maxwell" => "https://youtube.com/live/IWESchJ58ZQ",
        "maxwelllm1" => "https://youtube.com/live/IWESchJ58ZQ"
    },
};

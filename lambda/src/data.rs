use phf::phf_map;
use once_cell::sync::Lazy;

pub static STREAMS: Lazy<&'static phf::Map<&'static str, &'static str>> = Lazy::new(|| {
    ALL_STREAMS.get("2024").expect("invalid year")
});

pub const ALL_STREAMS: phf::Map<&'static str, phf::Map<&'static str, &'static str>> = phf_map! {
    "2024" => phf_map! {
        "bardeen" => "https://www.youtube.com/watch?v=AfZ_z4BQZuk",
        "salk" => "https://www.youtube.com/watch?v=yPoC3pbuk9o",
        "maxwell" => "https://www.youtube.com/watch?v=aj2QMS5zFaM",
        "turing" => "https://www.youtube.com/watch?v=HAAQTXrOZw4",
        "knuth" => "https://www.youtube.com/watch?v=10XjXGPzAyE",
        "spencer" => "https://www.youtube.com/watch?v=zHW5F4EthHU",
        "brattain" => "https://www.youtube.com/watch?v=pm6gNIm4dnU",
        "feynman" => "https://youtube.com/live/2QSo1Zb2Xo4",
        "tesla" => "https://youtube.com/live/3ppW3ZNnmDA?",
        "hawking" => "https://youtube.com/live/Me98sMD5KBs",
        "pasteur" => "https://youtube.com/live/pzwmqGg6mT4",
        "watt" => "https://youtube.com/live/UkmZnJgWUFA",
        "state" => "https://www.youtube.com/watch?v=xq-7jn_oMpU",
        "wyoming" => "https://youtube.com/live/-_atRRrqvxI",
    },
};

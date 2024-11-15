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
        "bardeen" => "https://youtube.com/live/clSg1FAUI7k",
        "bardeenlm1" => "https://www.youtube.com/watch?v=F0caNUelJS4",
        "bardeenlm2" => "https://www.youtube.com/watch?v=clSg1FAUI7k",
        "brattain" => "https://www.youtube.com/watch?v=euo4xfI06_g",
        "brattainlm1" => "https://www.youtube.com/watch?v=euo4xfI06_g",
        "knuth" => "https://youtube.com/live/7N3Dam5n7HI",
        "knuthlm1" => "https://www.youtube.com/watch?v=KP2g5VyZMlg",
        "knuthlm2" => "https://www.youtube.com/watch?v=7N3Dam5n7HI",
        "lamarr" => "https://youtube.com/live/wZzUKAND7ys",
        "lamarrlm1" => "https://www.youtube.com/watch?v=B58rLrp_758",
        "lamarrlm2" => "https://www.youtube.com/watch?v=wZzUKAND7ys",
        "maxwell" => "https://youtube.com/live/Yp1pBfummqs",
        "maxwelllm1" => "https://youtube.com/watch?v=6jJjmvKK8FM",
        "maxwelllm2" => "https://www.youtube.com/watch?v=Yp1pBfummqs",
        "noddack" => "https://www.youtube.com/watch?v=_HL5ZwIVgvw",
        "noddacklm1" => "https://www.youtube.com/watch?v=_HL5ZwIVgvw",
        "ritchie" => "https://www.youtube.com/watch?v=Ph283fdDomo",
        "ritchielm1" => "https://www.youtube.com/watch?v=Ph283fdDomo",
        "salk" => "https://youtube.com/live/6S86EBSxchU",
        "salklm1" => "https://www.youtube.com/watch?v=4Pp_X0RtDBI",
        "salklm2" => "https://www.youtube.com/watch?v=6S86EBSxchU",
        "spencer" => "https://www.youtube.com/watch?v=PwI7hqNdWVg",
        "spencerlm1" => "https://www.youtube.com/watch?v=PwI7hqNdWVg",
        "turing" => "https://www.youtube.com/watch?v=4Pp_X0RtDBI",
        "turinglm1" => "https://www.youtube.com/watch?v=4Pp_X0RtDBI",
    },
};

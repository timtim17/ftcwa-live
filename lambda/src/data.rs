use phf::phf_map;

pub const STREAMS: phf::Map<&'static str, &'static str> = phf_map! {
    "bardeen" => "https://www.youtube.com/watch?v=AfZ_z4BQZuk",
    "salk" => "https://www.youtube.com/watch?v=yPoC3pbuk9o",
    "maxwell" => "https://www.youtube.com/watch?v=aj2QMS5zFaM",
    "turing" => "https://www.youtube.com/watch?v=HAAQTXrOZw4",
    "knuth" => "https://www.youtube.com/watch?v=UN8VEVwoNyA",
    "spencer" => "https://www.youtube.com/watch?v=zHW5F4EthHU",
    "brattain" => "https://www.youtube.com/watch?v=pm6gNIm4dnU",
};

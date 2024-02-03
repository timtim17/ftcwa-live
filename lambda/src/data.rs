use phf::phf_map;

pub const STREAMS: phf::Map<&'static str, &'static str> = phf_map! {
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
    "state" => "https://youtube.com/live/OGVQthn9L38"
};

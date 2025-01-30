use clap::{Arg, Command};


mod download;
mod errors;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .help("set which mode to download with (default: auto, other: audio)"),
        )
        .arg(
            Arg::new("apiurl")
                .short('a')
                .long("apiurl")
                .help("set the api url, don't include https (default: co.wuk.sh"),

        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("url to download form"),
        )
        // video
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .help("set the video quality (default: 1080p, other: 4320p+ 2160, 720p, 480p, 360p)"),
        )
        .arg(
            Arg::new("codec")
                .short('c')
                .long("codec")
                .help("set youtube codec (default: h264, other: av1, vp9)"),
        )
        .arg(
            Arg::new("ttwatermark")
                .short('w')
                .long("ttwatermark")
                .num_args(0)
                .help("disable tiktok watermark (default: false)"),
        )
        // auto
        .arg(
            Arg::new("dublang")
                .short('d')
                .long("dublang")
                .num_args(0)
                .help("dub language (default: false)"),
        )
        .arg(
            Arg::new("fullaudio")
                .short('k')
                .long("fullaudio")
                .num_args(0)
                .help("get tiktok full audio (default: false)"),
        )
        .arg(
            Arg::new("mute")
                .short('j')
                .long("mute")
                .num_args(0)
                .help("mute the video (default: false)"),
        )
        .get_matches();

    let homedirpathbuf = dirs::home_dir();
    let homedirexpect = homedirpathbuf.expect("method not found in `Option<OathBuf>`");
    let homedir = homedirexpect.display();

    let mut mode = "unspecified".to_string();
    if matches.get_one::<String>("mode").is_none() {
        errors::create_end("you didn't specify a mode");
    } else {
        mode = matches.get_one::<String>("mode").unwrap().to_string();
    }

    let d_apiurl = "co.wuk.sh".to_string();
    let apiurl: &String = matches.get_one::<String>("apiurl").unwrap_or(&d_apiurl);

    let d_path = format!("{homedir}/Downloads").to_string();
    let path: &String = matches.get_one::<String>("path").unwrap_or(&d_path);

    let mut url = "unspecified".to_string();
    if matches.get_one::<String>("url").is_none() {
        errors::create_end("you didn't specify a video url");;
    } else {
        url = matches.get_one::<String>("url").unwrap().to_string();
    }
}

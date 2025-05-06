use std::process::Command;

use clap::Parser;

#[derive(Parser)]
#[command(name = "yt")]
#[command(version = "1.0")]
#[command(about = "A simple wrapper for yt-dlp")]
struct Cli {
    link: String,

    #[arg(short, long)]
    audio_only: bool,
    #[arg(short, long)]
    video_only: bool,
    #[arg(short, long, help = "16:9 aspect ratio + mp4")]
    defaults: bool,

    #[arg(short, long, help = "file name without extension")]
    name: Option<String>,
    #[arg(
        short,
        long,
        num_args = 2,
        help = "Sections are HR:MIN:SEC MIN:SEC or SEC"
    )]
    section: Option<Vec<String>>,
}

impl Cli {
    fn medium_selection(&self) -> MediumSelection {
        if self.audio_only && self.video_only {
            return MediumSelection::Error;
        }
        if self.audio_only {
            return MediumSelection::AudioOnly;
        }
        if self.video_only {
            return MediumSelection::VideoOnly;
        }
        MediumSelection::Default
    }
}

enum MediumSelection {
    Default,
    AudioOnly,
    VideoOnly,
    Error,
}

fn main() {
    let cli = Cli::parse();

    let mut command = Command::new("yt-dlp");

    command.arg(&cli.link);

    match cli.medium_selection() {
        MediumSelection::Default => {
            if cli.defaults {
                command
                    .args(["--format", "bestvideo[aspect_ratio=1.78]+bestaudio"])
                    .args(["--recode-video", "mp4"]);
            }
        }
        MediumSelection::AudioOnly => {
            command
                .arg("--extract-audio")
                .args(["--audio-format", "wav"]);
        }
        MediumSelection::VideoOnly => {
            if cli.defaults {
                command
                    .args(["--format", "bestvideo[aspect_ratio=1.78]"])
                    .args(["--recode-video", "mp4"]);
            } else {
                command.args(["--format", "bestvideo"]);
            }
        }
        MediumSelection::Error => {
            println!("You can only select one medium");
            return;
        }
    }

    if let Some(section) = cli.section {
        command.args([
            "--download-sections",
            format!("*{}-{}", section[0], section[1]).as_str(),
        ]);
    }

    if let Some(name) = cli.name {
        command.args(["--output", format!("{name}.%(ext)s").as_str()]);
    }

    let _out = command.status().expect("didn't work");
}

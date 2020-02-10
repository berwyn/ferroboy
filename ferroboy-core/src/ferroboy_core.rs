#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

use std::env;

use ferroboy::State;
use libretro_backend::{
    AudioVideoInfo, Core, CoreInfo, GameData, LoadGameResult, PixelFormat, Region, RuntimeHandle,
};

pub struct FerroboyCore {
    game_data: Option<GameData>,
    state: State,
}

impl FerroboyCore {
    fn new() -> Self {
        Self {
            game_data: None,
            state: Default::default(),
        }
    }
}

impl Default for FerroboyCore {
    fn default() -> Self {
        Self::new()
    }
}

impl Core for FerroboyCore {
    fn info() -> CoreInfo {
        CoreInfo::new("Ferroboy", env!("CARGO_PKG_VERSION")).supports_roms_with_extension("gb")
    }

    fn on_load_game(&mut self, game_data: GameData) -> LoadGameResult {
        if game_data.is_empty() {
            return LoadGameResult::Failed(game_data);
        }

        let result: Result<(), String> = if let Some(data) = game_data.data() {
            self.state.load_cartridge_from_buffer(data)
        } else if let Some(path) = game_data.path() {
            self.state.load_cartridge_from_file(path)
        } else {
            // Since the game data isn't empty, we must have a path or a buffer
            unreachable!();
        };

        match result {
            Ok(_) => {
                self.game_data = Some(game_data);
                let av_info = AudioVideoInfo::new()
                    .video(160, 144, 60.0, PixelFormat::ARGB8888)
                    .audio(44100.0)
                    .region(Region::NTSC);

                LoadGameResult::Success(av_info)
            }
            Err(_) => LoadGameResult::Failed(game_data),
        }
    }

    fn on_unload_game(&mut self) -> GameData {
        self.game_data.take().unwrap()
    }

    fn on_run(&mut self, _handle: &mut RuntimeHandle) {}

    fn on_reset(&mut self) {}
}

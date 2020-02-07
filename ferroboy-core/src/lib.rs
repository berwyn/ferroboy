#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

use std::env;

use libretro_backend::{
    libretro_core, AudioVideoInfo, Core, CoreInfo, GameData, LoadGameResult, PixelFormat, Region,
    RuntimeHandle,
};

struct FerroboyCore {
    game_data: Option<GameData>,
}

impl FerroboyCore {
    fn new() -> Self {
        Self { game_data: None }
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

        // TODO(berwyn): Actually use a real type here
        let result: Result<&str, &str> = if let Some(_data) = game_data.data() {
            // TODO(berwyn): load the ROM from a buffer
            Ok("")
        } else if let Some(_path) = game_data.path() {
            // TODO(berwyn): open the file and read the ROM
            Ok("")
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

libretro_core!(FerroboyCore);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#![deny(nonstandard_style)]
#![deny(clippy::all)]

use std::env;

use ferroboy::{CartridgeBuilder, State};
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
        CoreInfo::new("Ferroboy", env!("CARGO_PKG_VERSION"))
            .supports_roms_with_extension("gb")
            .requires_path_when_loading_roms()
    }

    fn on_load_game(&mut self, game_data: GameData) -> LoadGameResult {
        if game_data.is_empty() {
            return LoadGameResult::Failed(game_data);
        }

        let mut cartridge_builder = CartridgeBuilder::new();

        if let Some(data) = game_data.data() {
            cartridge_builder = cartridge_builder.with_buffer(data);
        } else if let Some(path) = game_data.path() {
            match std::fs::File::open(path) {
                Ok(file) => {
                    cartridge_builder = cartridge_builder.with_file(file);
                }
                _ => return LoadGameResult::Failed(game_data),
            }
        } else {
            unreachable!();
        }

        match cartridge_builder.build() {
            Ok(cart) => {
                self.state.cartridge.replace(cart);
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

    fn on_run(&mut self, handle: &mut RuntimeHandle) {
        let mut video_buffer: Vec<u8> = Vec::<u8>::with_capacity(160 * 144 * 4);

        for _ in 0..160 * 144 {
            video_buffer.push(0x00);
            video_buffer.push(0x00);
            video_buffer.push(0x00);
            video_buffer.push(0xFF);
        }

        handle.upload_video_frame(&video_buffer);

        for _ in 0..1470 {
            handle.upload_audio_frame(&[0, 0]);
        }
    }

    fn on_reset(&mut self) {}
}

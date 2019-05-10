// Copyright (c) 2017-2018, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

mod muxer;
pub use muxer::*;

mod ivfmuxer;
use ivfmuxer::IvfMuxer;

mod y4mmuxer;
pub use y4mmuxer::write_y4m_frame;

#[cfg(feature = "ffmpeg-sys")]
mod avformatmuxer;
#[cfg(feature = "ffmpeg-sys")]
use avformatmuxer::AvformatMuxer;

use std::ffi::OsStr;
use std::path::Path;

fn need_container(path: &str) -> bool {
  let ext =
    Path::new(path).extension().and_then(OsStr::to_str).map(str::to_lowercase);
  match ext {
    Some(ref s) => match &s[..] {
      //webm stil have problem. It may related to https://github.com/FFmpeg/FFmpeg/commit/de1b44c20604c05812ad70167a26d45e0ec1526f#diff-c0b3e3c679bfc528be17df29400712bdR2361
      //need time to figure out.
      "mp4" => true,
      _ => false
    },
    _ => false
  }
}

#[allow(unreachable_code)]
pub fn create_muxer(path: &str) -> Box<dyn Muxer> {
  if need_container(path) {
    #[cfg(feature = "ffmpeg-sys")]
    return AvformatMuxer::open(path);
    panic!("need ffmpeg-sys for container format, please build with --features=\"ffmpeg-sys\", or you can use .ivf extesion");
  }

  IvfMuxer::open(path)
}

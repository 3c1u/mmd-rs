use std::io::Read;

use byteorder::{ReadBytesExt, LE};
use encoding_rs::SHIFT_JIS;

const VMD_HEADER: &'static [u8] = b"Vocaloid Motion Data 0002\0";
const VMD_MODEL_NAME_SIZE: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmdHeader {
  pub model_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MotionFrame {
  pub name: String,
  pub frame_no: u32,
  pub position: [f32; 3],
  pub rotation: [f32; 4],
  pub interpolation: [u8; 64],
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkinFrame {
  pub unknown: [u8; 23]
}

#[derive(Debug, Clone, PartialEq)]
pub struct CameraFrame {
  pub unknown: [u8; 61]
}

#[derive(Debug, Clone, PartialEq)]
pub struct LightFrame {
  pub unknown: [u8; 28]
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowFrame {
  pub unknown: [u8; 9]
}

fn read_string<R: Read>(read: &mut R, size: usize) -> crate::Result<String> {
  let mut buf = vec![0; size];
  read.read_exact(&mut buf)?;

  // Truncate null bytes
  // NOTE: in some motion files the model name is filled with 0xfd after the null byte
  let buf = &buf[0..buf.iter().position(|&x| x == 0).unwrap_or(size)];

  // Convert to string (Shift_JIS)
  let (s, _, is_malformed) = SHIFT_JIS.decode(buf);
  let s = if is_malformed {
    // Try UTF-8, then fallback to Shift_JIS
    std::str::from_utf8(buf)
      .map(|s| s.to_string())
      .unwrap_or_else(|_| s.to_string())
  } else {
    s.to_string()
  };

  Ok(s)
}

fn read_vec<R: Read, const N: usize>(read: &mut R) -> crate::Result<[f32; N]> {
  let mut buf = [0f32; N];

  for i in 0..N {
    buf[i] = read.read_f32::<LE>()?;
  }

  Ok(buf)
}

impl VmdHeader {
  pub fn read<R: Read>(read: &mut R) -> crate::Result<Self> {
    // Read header
    let mut buf = [0; 30];
    read.read_exact(&mut buf)?;

    if buf[..VMD_HEADER.len()] != *VMD_HEADER {
      return Err(crate::Error::InvalidHeader);
    }

    let model_name = read_string(read, VMD_MODEL_NAME_SIZE)?;

    Ok(VmdHeader { model_name })
  }
}

impl MotionFrame {
  pub fn read_all<R: Read>(read: &mut R) -> crate::Result<Vec<Self>> {
    let total_frames = read.read_u32::<LE>()?;

    let mut frames = Vec::with_capacity(total_frames as usize);

    for _ in 0..total_frames {
      frames.push(Self::read(read)?);
    }

    Ok(frames)
  }

  pub fn read<R: Read>(read: &mut R) -> crate::Result<Self> {
    let name = read_string(read, 15)?;

    let frame_no = read.read_u32::<LE>()?;
    let position = read_vec::<_, 3>(read)?;
    let rotation = read_vec::<_, 4>(read)?;
    let interpolation = {
      let mut buf = [0; 64];
      read.read_exact(&mut buf)?;
      buf
    };

    Ok(Self {
      name,
      frame_no,
      position,
      rotation,
      interpolation,
    })
  }
}

impl SkinFrame {
  pub fn read_all<R: Read>(read: &mut R) -> crate::Result<Vec<Self>> {
    let total_frames = read.read_u32::<LE>()?;

    let mut frames = Vec::with_capacity(total_frames as usize);

    for _ in 0..total_frames {
      frames.push(Self::read(read)?);
    }

    Ok(frames)
  }

  pub fn read<R: Read>(read: &mut R) -> crate::Result<Self> {
    let unknown = {
      let mut buf = [0; 23];
      read.read_exact(&mut buf)?;
      buf
    };

    Ok(Self { unknown })
  }
}

impl CameraFrame {
  pub fn read_all<R: Read>(read: &mut R) -> crate::Result<Vec<Self>> {
    let total_frames = read.read_u32::<LE>()?;

    let mut frames = Vec::with_capacity(total_frames as usize);

    for _ in 0..total_frames {
      frames.push(Self::read(read)?);
    }

    Ok(frames)
  }

  pub fn read<R: Read>(read: &mut R) -> crate::Result<Self> {
    let unknown = {
      let mut buf = [0; 61];
      read.read_exact(&mut buf)?;
      buf
    };

    Ok(Self { unknown })
  }
}

impl LightFrame {
  pub fn read_all<R: Read>(read: &mut R) -> crate::Result<Vec<Self>> {
    let total_frames = read.read_u32::<LE>()?;

    let mut frames = Vec::with_capacity(total_frames as usize);

    for _ in 0..total_frames {
      frames.push(Self::read(read)?);
    }

    Ok(frames)
  }

  pub fn read<R: Read>(read: &mut R) -> crate::Result<Self> {
    let unknown = {
      let mut buf = [0; 28];
      read.read_exact(&mut buf)?;
      buf
    };

    Ok(Self { unknown })
  }
}

impl ShadowFrame {
  pub fn read_all<R: Read>(read: &mut R) -> crate::Result<Vec<Self>> {
    let total_frames = read.read_u32::<LE>()?;

    let mut frames = Vec::with_capacity(total_frames as usize);

    for _ in 0..total_frames {
      frames.push(Self::read(read)?);
    }

    Ok(frames)
  }

  pub fn read<R: Read>(read: &mut R) -> crate::Result<Self> {
    let unknown = {
      let mut buf = [0; 9];
      read.read_exact(&mut buf)?;
      buf
    };

    Ok(Self { unknown })
  }
}

#[cfg(test)]
mod tests {
  const FIXTURE_MOTION_VMD: &'static [u8] = include_bytes!("../../fixtures/motion.vmd");
  const FIXTURE_CAMERA_VMD: &'static [u8] = include_bytes!("../../fixtures/camera.vmd");
  const FIXTURE_ISSUE1_VMD: &'static [u8] = include_bytes!("../../fixtures/issue1.vmd");

  fn util_test_vmd_header(bytes: &[u8], model_name: &str) {
    let header = super::VmdHeader::read(&mut std::io::Cursor::new(bytes)).unwrap();
    assert_eq!(header.model_name, model_name);
  }

  #[test]
  fn test_vmd_header_motion() {
    util_test_vmd_header(FIXTURE_MOTION_VMD, "初音ミク");
  }

  #[test]
  fn test_vmd_header_camera() {
    util_test_vmd_header(FIXTURE_CAMERA_VMD, "カメラ・照明");
  }

  #[test]
  fn test_vmd_header_issue1() {
    util_test_vmd_header(FIXTURE_ISSUE1_VMD, "初音ミク");
  }

  #[test]
  fn test_vmd_frame_motion() {
    let mut cursor = std::io::Cursor::new(FIXTURE_MOTION_VMD);
    super::VmdHeader::read(&mut cursor).unwrap();

    let frame = super::MotionFrame::read_all(&mut cursor).unwrap();

    assert_eq!(frame.len(), 164);
    assert_eq!(frame[0].name, "センター");
    assert_eq!(frame[0].frame_no, 0);
  }

  #[test]
  fn test_vmd_frame_camera() {
    let mut cursor = std::io::Cursor::new(FIXTURE_CAMERA_VMD);
    super::VmdHeader::read(&mut cursor).unwrap();

    let frame = super::MotionFrame::read_all(&mut cursor).unwrap();

    assert_eq!(frame.len(), 0);
  }

  #[test]
  fn test_vmd_frame_issue1() {
    let mut cursor = std::io::Cursor::new(FIXTURE_ISSUE1_VMD);
    super::VmdHeader::read(&mut cursor).unwrap();

    let frame = super::MotionFrame::read_all(&mut cursor).unwrap();

    assert_eq!(frame.len(), 7);
    assert_eq!(frame[0].name, "左目");
    assert_eq!(frame[0].frame_no, 0);
  }
}

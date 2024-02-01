use crate::{Config, DefaultConfig};

const HEADER: &str = "Vocaloid Pose Data file";

pub struct BoneTransform<C: Config = DefaultConfig> {
  pub id: u32,
  pub name: String,
  pub position: C::Vec3,
  pub rotation: C::Vec4,
}

pub struct MorphValue<C: Config = DefaultConfig> {
  pub id: u32,
  pub name: String,
  pub weight: f32,
  pub offset: C::Vec3,
}

pub struct Vpd<C: Config = DefaultConfig> {
  pub name: String,
  pub bone_transforms: Vec<BoneTransform<C>>,
  pub morph_values: Vec<MorphValue<C>>,
}

impl<C: Config> Vpd<C> {
  pub fn new(name: String) -> Self {
    Self {
      name,
      bone_transforms: Vec::new(),
      morph_values: Vec::new(),
    }
  }

  pub fn read<R: std::io::Read>(mut reader: R) -> crate::Result<Self> {
    let mut string_buf = String::new();

    fn read_line<'a, R: std::io::Read>(
      reader: &mut R,
      buf: &'a mut String,
    ) -> crate::Result<(&'a str, usize)> {
      buf.clear();
      let bytes = reader.read_to_string(buf)?;
      let mut line = buf.trim();

      // Remove comments (starting with "//")
      if let Some(pos) = line.find("//") {
        line = &line[..pos].trim();
      }

      Ok((line, bytes))
    }

    // Read header
    read_line(&mut reader, &mut string_buf)?;
    if string_buf.trim() != HEADER {
      return Err(crate::Error::InvalidHeader);
    }

    loop {
      string_buf.clear();

      let (line, total_bytes) = read_line(&mut reader, &mut string_buf)?;

      // EOF
      if total_bytes == 0 {
        break;
      }

      // Skip empty lines
      if line.is_empty() {
        continue;
      }

      if line.starts_with("Bone") {
        let _id: u32 = line[4..].trim().parse().expect("Invalid bone ID");

        todo!()
      } else if line.starts_with("Morph") {
        todo!()
      } else {
        // TODO: Better error handling
        panic!("Invalid line: {}", line);
      }
    }

    todo!()
  }
}

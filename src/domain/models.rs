pub struct FaceAttributes {
    bbox_index: usize,
    identity_id: String, // e.g. Some("person_023") or None
    age_group: AgeGroup,
    ethnicity: Ethnicity,
    eye_color: EyeColor,
    hair_color: HairColor,
    hair_length: HairLength,
    emotion: Emotion,
    pose: Pose,
}

// For each image
pub struct ImageAttributes {
    image: String,
    faces: Vec<FaceAttributes>,
}

pub struct YoloFormat {
    pub class_id: usize,
    pub x_center: f32,
    pub y_center: f32,
    pub width: f32,
    pub height: f32,
}

// default FULL-IMAGE bbox for "already cropped face" images:
impl Default for YoloFormat {
    fn default() -> Self {
        Self {
            class_id: 0,
            x_center: 0.5,
            y_center: 0.5,
            width: 1.0,
            height: 1.0,
        }
    }
}

pub enum AgeGroup {
    Infant,
    Child,
    YoungAdult,
    Adult,
    MiddleAge,
    Unknown,
}
pub enum Ethnicity {
    Asian,
    Black,
    Latino,
    White,
    Other,
    Unknown,
}
pub enum EyeColor {
    Brown,
    Blue,
    Gray,
    Green,
    Other,
    Unknown,
}
pub enum HairColor {
    Black,
    Brown,
    Blonde,
    Gray,
    Other,
    Unknown,
}
pub enum HairLength {
    Short,
    Medium,
    Long,
    Unknown,
}
pub enum Emotion {
    Neutral,
    Joy,
    Other,
    Unknown,
}
pub enum Pose {
    Front,
    Left,
    Right,
    Other,
    Unknown,
}

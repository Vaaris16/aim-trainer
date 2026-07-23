use bevy::{prelude::*, text::LetterSpacing};

pub struct SpawnText {
    text: String,
    text_font: Handle<Font>,
    text_size: Option<f32>,
    text_color: Color,
    letter_spacing: Option<f32>,
}

type SpawnTextReturn = (Text, TextFont, TextColor, LetterSpacing);

impl SpawnText {
    pub fn new(
        text: String,
        text_font: Handle<Font>,
        text_size: Option<f32>,
        text_color: Color,
        letter_spacing: Option<f32>,
    ) -> Self {
        Self {
            text: text.into(),
            text_font: text_font,
            text_size: text_size,
            text_color: text_color,
            letter_spacing: letter_spacing,
        }
    }

    pub fn spawn_text(&self) -> SpawnTextReturn {
        (
            Text::new(self.text.clone()),
            TextFont {
                font: FontSource::Handle(self.text_font.clone()),
                font_size: self.text_size.map(FontSize::Px).unwrap_or_default(),
                ..Default::default()
            },
            TextColor(self.text_color),
            self.letter_spacing
                .map(LetterSpacing::Px)
                .unwrap_or_default(),
        )
    }
}

#![allow(unused)]
use crate::SortModel;

impl crate::Content for SortModel {
    #[inline]
    fn capacity_hint(&self, tpl: &crate::Template) -> usize {
        tpl.capacity_hint() + self.field.capacity_hint(tpl)
            + self.sort.capacity_hint(tpl)
    }
    #[inline]
    fn render_section<C, E, IC>(
        &self,
        section: crate::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: crate::traits::ContentSequence,
        E: crate::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn apply_section<C>(
        &self,
        section: crate::SimpleSection<C>,
    ) -> std::result::Result<crate::SimpleValue, crate::SimpleError>
    where
        C: crate::traits::ContentSequence,
    {
        section.with(self).apply()
    }
    #[inline]
    fn render_notnone_section<C, E, IC>(
        &self,
        section: crate::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: crate::traits::ContentSequence,
        E: crate::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn render_field_escaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: crate::encoding::Encoder,
    {
        match hash {
            5264107000299760680u64 => self.field.render_escaped(encoder).map(|_| true),
            9189260103713392746u64 => self.sort.render_escaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    #[inline]
    fn render_field_unescaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: crate::encoding::Encoder,
    {
        match hash {
            5264107000299760680u64 => self.field.render_unescaped(encoder).map(|_| true),
            9189260103713392746u64 => self.sort.render_unescaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    #[inline]
    fn apply_field_unescaped(
        &self,
        hash: u64,
        name: &str,
    ) -> std::result::Result<crate::SimpleValue, crate::SimpleError> {
        match hash {
            5264107000299760680u64 => self.field.apply_unescaped(),
            9189260103713392746u64 => self.sort.apply_unescaped(),
            _ => {
                Err(
                    crate::SimpleInnerError(std::format!("the data type of field: {0} is not supported ", name)).into()
                )
            }
        }
    }
    fn render_field_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: crate::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: crate::traits::ContentSequence,
        E: crate::encoding::Encoder,
    {
        match hash {
            5264107000299760680u64 => {
                self.field
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            9189260103713392746u64 => {
                self.sort
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn apply_field_section<P>(
        &self,
        hash: u64,
        name: &str,
        section: crate::SimpleSection<P>,
    ) -> std::result::Result<crate::SimpleValue, crate::SimpleError>
    where
        P: crate::traits::ContentSequence,
    {
        match hash {
            5264107000299760680u64 => self.field.apply_section(section),
            9189260103713392746u64 => self.sort.apply_section(section),
            _ => {
                Err(
                    crate::SimpleInnerError(std::format!("tthe data type of field is not supported")).into()
                )
            }
        }
    }
    fn render_field_inverse<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: crate::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: crate::traits::ContentSequence,
        E: crate::encoding::Encoder,
    {
        match hash {
            5264107000299760680u64 => {
                self.field
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            9189260103713392746u64 => {
                self.sort
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_notnone_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: crate::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: crate::traits::ContentSequence,
        E: crate::encoding::Encoder,
    {
        match hash {
            5264107000299760680u64 => {
                self.field
                    .render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.field.is_truthy())
            }
            9189260103713392746u64 => {
                self.sort.render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.sort.is_truthy())
            }
            _ => Ok(false),
        }
    }
}

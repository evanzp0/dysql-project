#![allow(unused)]

use super::Value;

impl<T> dysql_tpl::Content for Value<T>
where
    T: dysql_tpl::Content,
{
    #[inline]
    fn capacity_hint(&self, tpl: &dysql_tpl::Template) -> usize {
        tpl.capacity_hint() + self.value.capacity_hint(tpl)
    }
    #[inline]
    fn render_section<C, E, IC>(
        &self,
        section: dysql_tpl::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: dysql_tpl::traits::ContentSequence,
        E: dysql_tpl::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn apply_section<C>(
        &self,
        section: dysql_tpl::SimpleSection<C>,
    ) -> std::result::Result<dysql_tpl::SimpleValue, dysql_tpl::SimpleError>
    where
        C: dysql_tpl::traits::ContentSequence,
    {
        section.with(self).apply()
    }
    #[inline]
    fn render_notnone_section<C, E, IC>(
        &self,
        section: dysql_tpl::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: dysql_tpl::traits::ContentSequence,
        E: dysql_tpl::encoding::Encoder,
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
        E: dysql_tpl::encoding::Encoder,
    {
        match hash {
            2388869121238140847u64 => self.value.render_escaped(encoder).map(|_| true),
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
        E: dysql_tpl::encoding::Encoder,
    {
        match hash {
            2388869121238140847u64 => self.value.render_unescaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    #[inline]
    fn apply_field_unescaped(
        &self,
        hash: u64,
        name: &str,
    ) -> std::result::Result<dysql_tpl::SimpleValue, dysql_tpl::SimpleError> {
        match hash {
            2388869121238140847u64 => self.value.apply_unescaped(),
            _ => {
                Err(
                    dysql_tpl::SimpleInnerError(std::format!("the data type of field: {0} is not supported ", name)).into()
                )
            }
        }
    }
    fn render_field_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: dysql_tpl::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: dysql_tpl::traits::ContentSequence,
        E: dysql_tpl::encoding::Encoder,
    {
        match hash {
            2388869121238140847u64 => {
                self.value
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
        section: dysql_tpl::SimpleSection<P>,
    ) -> std::result::Result<dysql_tpl::SimpleValue, dysql_tpl::SimpleError>
    where
        P: dysql_tpl::traits::ContentSequence,
    {
        match hash {
            2388869121238140847u64 => self.value.apply_section(section),
            _ => {
                Err(
                    dysql_tpl::SimpleInnerError(std::format!("tthe data type of field is not supported")).into()
                )
            }
        }
    }
    fn render_field_inverse<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: dysql_tpl::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: dysql_tpl::traits::ContentSequence,
        E: dysql_tpl::encoding::Encoder,
    {
        match hash {
            2388869121238140847u64 => {
                self.value
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
        section: dysql_tpl::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: dysql_tpl::traits::ContentSequence,
        E: dysql_tpl::encoding::Encoder,
    {
        match hash {
            2388869121238140847u64 => {
                self.value
                    .render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.value.is_truthy())
            }
            _ => Ok(false),
        }
    }
}

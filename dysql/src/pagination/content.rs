#![allow(unused)]
use crate::Pagination;

impl<T> crate::Content for Pagination<T>
where
    T: crate::Content,
{
    #[inline]
    fn capacity_hint(&self, tpl: &crate::Template) -> usize {
        tpl.capacity_hint() + self.total.capacity_hint(tpl)
            + self.data.capacity_hint(tpl) + self.total_page.capacity_hint(tpl)
            + self.page_size.capacity_hint(tpl) + self.page_no.capacity_hint(tpl)
            + self.start.capacity_hint(tpl)
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
            1212331373962215526u64 => self.total.render_escaped(encoder).map(|_| true),
            4430724373119788750u64 => self.data.render_escaped(encoder).map(|_| true),
            9928246550803098198u64 => {
                self.total_page.render_escaped(encoder).map(|_| true)
            }
            10087286125916898991u64 => {
                self.page_size.render_escaped(encoder).map(|_| true)
            }
            11609058959308731613u64 => self.page_no.render_escaped(encoder).map(|_| true),
            13127600857983441824u64 => self.start.render_escaped(encoder).map(|_| true),
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
            1212331373962215526u64 => self.total.render_unescaped(encoder).map(|_| true),
            4430724373119788750u64 => self.data.render_unescaped(encoder).map(|_| true),
            9928246550803098198u64 => {
                self.total_page.render_unescaped(encoder).map(|_| true)
            }
            10087286125916898991u64 => {
                self.page_size.render_unescaped(encoder).map(|_| true)
            }
            11609058959308731613u64 => {
                self.page_no.render_unescaped(encoder).map(|_| true)
            }
            13127600857983441824u64 => self.start.render_unescaped(encoder).map(|_| true),
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
            1212331373962215526u64 => self.total.apply_unescaped(),
            4430724373119788750u64 => self.data.apply_unescaped(),
            9928246550803098198u64 => self.total_page.apply_unescaped(),
            10087286125916898991u64 => self.page_size.apply_unescaped(),
            11609058959308731613u64 => self.page_no.apply_unescaped(),
            13127600857983441824u64 => self.start.apply_unescaped(),
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
            1212331373962215526u64 => {
                self.total
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            4430724373119788750u64 => {
                self.data
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            9928246550803098198u64 => {
                self.total_page
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            10087286125916898991u64 => {
                self.page_size
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            11609058959308731613u64 => {
                self.page_no
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            13127600857983441824u64 => {
                self.start
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
            1212331373962215526u64 => self.total.apply_section(section),
            4430724373119788750u64 => self.data.apply_section(section),
            9928246550803098198u64 => self.total_page.apply_section(section),
            10087286125916898991u64 => self.page_size.apply_section(section),
            11609058959308731613u64 => self.page_no.apply_section(section),
            13127600857983441824u64 => self.start.apply_section(section),
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
            1212331373962215526u64 => {
                self.total
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            4430724373119788750u64 => {
                self.data
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            9928246550803098198u64 => {
                self.total_page
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            10087286125916898991u64 => {
                self.page_size
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            11609058959308731613u64 => {
                self.page_no
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            13127600857983441824u64 => {
                self.start
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
            1212331373962215526u64 => {
                self.total
                    .render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.total.is_truthy())
            }
            4430724373119788750u64 => {
                self.data.render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.data.is_truthy())
            }
            9928246550803098198u64 => {
                self.total_page
                    .render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.total_page.is_truthy())
            }
            10087286125916898991u64 => {
                self.page_size
                    .render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.page_size.is_truthy())
            }
            11609058959308731613u64 => {
                self.page_no
                    .render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.page_no.is_truthy())
            }
            13127600857983441824u64 => {
                self.start
                    .render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.start.is_truthy())
            }
            _ => Ok(false),
        }
    }
}

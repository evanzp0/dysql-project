
use dysql::Template;
fn main() {
    let v1 = Sa {
        bb: Sb { name: "my_name".to_owned() },
    };
    let s = "{{bb.name}}";
    let tpl = Template::new(s).unwrap();
    let rst = tpl.render(&v1);

    println!("{}", rst)
}
struct Sa {
    bb: Sb,
}
impl ::dysql::Content for Sa {
    #[inline]
    fn capacity_hint(&self, tpl: &::dysql::Template) -> usize {
        tpl.capacity_hint() + self.bb.capacity_hint(tpl)
    }
    #[inline]
    fn render_section<C, E, IC>(
        &self,
        section: ::dysql::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn render_notnone_section<C, E, IC>(
        &self,
        section: ::dysql::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
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
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => self.bb.render_escaped(encoder).map(|_| true),
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
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => self.bb.render_unescaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    fn render_field_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::dysql::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => {
                self.bb
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_inverse<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::dysql::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => {
                self.bb
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
        section: ::dysql::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => {
                self.bb.render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.bb.is_truthy())
            }
            _ => Ok(false),
        }
    }
}
struct Sb {
    name: String,
}
impl ::dysql::Content for Sb {
    #[inline]
    fn capacity_hint(&self, tpl: &::dysql::Template) -> usize {
        tpl.capacity_hint() + self.name.capacity_hint(tpl)
    }
    #[inline]
    fn render_section<C, E, IC>(
        &self,
        section: ::dysql::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn render_notnone_section<C, E, IC>(
        &self,
        section: ::dysql::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
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
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => self.name.render_escaped(encoder).map(|_| true),
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
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => self.name.render_unescaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    fn render_field_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::dysql::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => {
                self.name
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_inverse<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::dysql::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => {
                self.name
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
        section: ::dysql::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::dysql::traits::ContentSequence,
        E: ::dysql::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => {
                self.name.render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.name.is_truthy())
            }
            _ => Ok(false),
        }
    }
}

use juniper::graphql_object;

type Context = crate::schema::Context;

#[derive(Clone, Debug)]
pub struct Quote {
    pub inner: crate::protos::quotes::Quote,
}

#[derive(Clone, Debug)]
pub struct QuoteTranslation {
    pub inner: crate::protos::quotes::QuoteTranslation,
}

#[derive(Clone, Debug)]
pub struct QuoteAuthor {
    pub inner: crate::protos::quotes::QuoteAuthor,
}

#[graphql_object(context = Context)]
impl Quote {
    pub fn id(&self) -> &str {
        self.inner.id.as_str()
    }

    pub fn translations(&self) -> Vec<QuoteTranslation> {
        let translations = self.inner.translations.clone();
        translations
            .into_iter()
            .map(|t| QuoteTranslation { inner: t })
            .collect()
    }

    pub fn author(&self) -> Option<QuoteAuthor> {
        let author = self.inner.author.clone();
        author.map(|a| QuoteAuthor { inner: a })
    }
}

#[graphql_object(context = Context)]
impl QuoteTranslation {
    pub fn language(&self) -> &str {
        self.inner.language.as_str()
    }

    pub fn text(&self) -> &str {
        self.inner.text.as_str()
    }
}

#[graphql_object(context = Context)]
impl QuoteAuthor {
    pub fn id(&self) -> &str {
        self.inner.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.inner.name.as_str()
    }
}

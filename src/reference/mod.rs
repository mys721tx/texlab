mod bibtex_entry;
mod bibtex_string;
mod latex_label;

use self::{
    bibtex_entry::BibtexEntryReferenceProvider, bibtex_string::BibtexStringReferenceProvider,
    latex_label::LatexLabelReferenceProvider,
};
use crate::{
    feature::{ConcatProvider, FeatureProvider, FeatureRequest},
    protocol::{Location, ReferenceParams},
};
use futures_boxed::boxed;

pub struct ReferenceProvider {
    provider: ConcatProvider<ReferenceParams, Location>,
}

impl ReferenceProvider {
    pub fn new() -> Self {
        Self {
            provider: ConcatProvider::new(vec![
                Box::new(BibtexEntryReferenceProvider),
                Box::new(BibtexStringReferenceProvider),
                Box::new(LatexLabelReferenceProvider),
            ]),
        }
    }
}

impl Default for ReferenceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureProvider for ReferenceProvider {
    type Params = ReferenceParams;
    type Output = Vec<Location>;

    #[boxed]
    async fn execute<'a>(&'a self, req: &'a FeatureRequest<Self::Params>) -> Self::Output {
        self.provider.execute(req).await
    }
}

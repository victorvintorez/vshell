use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::DesktopLanguageRequester;
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::env;

#[derive(RustEmbed)]
#[folder = "i18n/"]
pub struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader = fluent_language_loader!();
    match env::var("VSHELL_LANG") {
        Ok(lang) => {
            let lang: LanguageIdentifier =
                lang.parse().expect("Failed to parse language identifier");
            i18n_embed::select(&loader, &Localizations, &[lang]).unwrap();
        }
        Err(_) => {
            let languages = DesktopLanguageRequester::requested_languages();
            i18n_embed::select(&loader, &Localizations, &languages).unwrap();
        }
    }
    loader.set_use_isolating(false);
    loader
});

use std::collections::HashMap;

use icu_locale_core::langid;
use icu_locale_core::LanguageIdentifier;
use serde::Deserialize;

use dioxus::prelude::*;

use crate::{
	main_menu::MenuBarGroupConfig,
	plugin::{Plugin, PluginRegistry},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Translation {
	Single(String),
	Plural { one: String, few: String, many: String },
}

#[derive(Clone)]
pub struct I18n {
	translations: HashMap<LanguageIdentifier, HashMap<String, Translation>>,
	language: LanguageIdentifier,
}

impl I18n {
	pub fn get(&self, key: &str) -> String {
		self.translations
			.get(&self.language)
			.and_then(|translations| {
				info!("I18n: Looking up translation for key '{key}' in language '{}'", self.language);
				translations.get(key).map(|translation| match translation {
					Translation::Single(value) => value.clone(),
					Translation::Plural { one, .. } => one.clone(),
				})
			})
			.unwrap_or_else(|| key.to_string())
	}

	pub fn get_plural(&self, key: &str, n: u64) -> String {
		let translation = self.translations.get(&self.language).and_then(|t| t.get(key));

		let Some(translation) = translation else {
			return key.to_string();
		};

		match translation {
			Translation::Single(s) => s.clone(),
			Translation::Plural { one, few, many } => key.to_string(),
			// Translation::Plural { one, few, many } => {
			// 	let form = self
			// 		.get(&self.language)
			// 		.map(|rules| rules.category_for(n))
			// 		.unwrap_or(PluralCategory::Other);

			// 	let s = match form {
			// 		PluralCategory::One => one,
			// 		PluralCategory::Few => few,
			// 		_ => many,
			// 	};
			// 	s.replace("{n}", &n.to_string())
			// }
		}
	}
}

pub fn i18n_core_plugin() -> Plugin {
	Plugin {
		name: "i18n_core".to_string(),
		description: "Core plugin for i18n support".to_string(),
		// entry: Some(i18n_core_entry),
		setup_context: Some(setup_context),
		menu_groups: vec![MenuBarGroupConfig {
			items: vec![],
			label: "menu_bar::i18n_core::languages",
		}],
		i18n: Some(HashMap::from([
			(
				langid!("en"),
				HashMap::from([(
					"menu_bar::i18n_core::languages".to_string(),
					Translation::Single("Languages".to_string()),
				)]),
			),
			(
				langid!("ua"),
				HashMap::from([(
					"menu_bar::i18n_core::languages".to_string(),
					Translation::Single("Мови".to_string()),
				)]),
			),
		])),
		..Default::default()
	}
}
fn setup_context() -> Element {
	let plugins = use_context::<Signal<PluginRegistry>>();

	let translations = plugins
		.read()
		.plugins
		.values()
		.filter(|p| p.is_enabled)
		.filter_map(|p| p.i18n.as_ref())
		.fold(
			HashMap::new(),
			|mut acc: HashMap<LanguageIdentifier, HashMap<String, Translation>>, plugin_i18n| {
				for (lang, keys) in plugin_i18n {
					acc.entry(lang.clone())
						.or_default()
						.extend(keys.iter().map(|(k, v)| (k.clone(), v.clone())));
				}
				acc
			},
		);

	use_context_provider(|| {
		Signal::new(I18n {
			translations,
			language: langid!("en"),
		})
	});
	rsx!()
}

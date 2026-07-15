use std::collections::HashMap;

use icu_locale_core::langid;
use icu_locale_core::LanguageIdentifier;
use serde::Deserialize;

use dioxus::prelude::*;

use crate::{
	event::Events,
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
	pub translations: HashMap<LanguageIdentifier, HashMap<String, Translation>>,
	pub language: LanguageIdentifier,
}

impl I18n {
	pub fn get(&self, key: &str) -> String {
		self.translations
			.get(&self.language)
			.and_then(|translations| {
				translations.get(key).map(|translation| match translation {
					Translation::Single(value) => value.clone(),
					Translation::Plural { one, .. } => one.clone(),
				})
			})
			.unwrap_or_else(|| key.to_string())
	}

	pub fn languages(&self) -> Vec<LanguageIdentifier> {
		self.translations.keys().cloned().collect()
	}

	pub fn get_for_language(&self, lang: &LanguageIdentifier, key: &str) -> Option<String> {
		self.translations.get(lang)?.get(key).map(|t| match t {
			Translation::Single(s) => s.clone(),
			Translation::Plural { one, .. } => one.clone(),
		})
	}

	pub fn get_plural(&self, key: &str, n: u64) -> String {
		let translation = self.translations.get(&self.language).and_then(|t| t.get(key));

		let Some(translation) = translation else {
			return key.to_string();
		};

		match translation {
			Translation::Single(s) => s.clone(),
			Translation::Plural { one, few, many } => key.to_string(),
		}
	}
}

pub struct ChangeLanguageEvent {
	pub code: String,
}

pub fn i18n_core_plugin() -> Plugin {
	Plugin {
		name: "i18n_core".to_string(),
		description: "Core plugin for i18n support".to_string(),
		entry: Some(entry),
		setup_context: Some(setup_context),
		menu_groups: vec![MenuBarGroupConfig {
			label: "menu_bar::i18n_core::languages",
			items: vec![],
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
				langid!("uk"),
				HashMap::from([(
					"menu_bar::i18n_core::languages".to_string(),
					Translation::Single("Мови".to_string()),
				)]),
			),
		])),
		..Default::default()
	}
}

fn entry() -> Element {
	let mut i18n = use_context::<Signal<I18n>>();
	let events = use_context::<Events>();
	let mut registry = use_context::<Signal<PluginRegistry>>();

	use_hook(move || {
		registry.write().plugins.get_mut("i18n_core").unwrap().is_initialized = true;
	});

	use_effect(move || {
		events.subscribe(move |event: &ChangeLanguageEvent| {
			if let Ok(lang) = event.code.parse::<LanguageIdentifier>() {
				i18n.write().language = lang;
			}
		});
	});

	rsx!()
}

fn setup_context() -> Element {
	let plugins = use_context::<Signal<PluginRegistry>>();

	let mut i18n = use_context_provider(|| {
		Signal::new(I18n {
			translations: HashMap::new(),
			language: langid!("en"),
		})
	});

	use_effect(move || {
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
		i18n.write().translations = translations;
	});

	rsx!()
}

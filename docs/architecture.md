## i18n

Internationalization is handled by the `i18n_core` plugin which provides a reactive `Signal<I18n>` context.

### Data structures

```rust
pub struct I18n {
    pub translations: HashMap<LanguageIdentifier, HashMap<String, Translation>>,
    pub language: LanguageIdentifier,  // current locale, e.g. langid!("uk")
}

pub enum Translation {
    Single(String),
    Plural { one: String, few: String, many: String },
}
```

Locale keys are `LanguageIdentifier` from `icu_locale_core` (BCP 47). This prevents plugins from registering arbitrary locale strings — the type validates on parse.

### Translation files (per plugin)

Each plugin provides its own translations inline in the `Plugin` struct:

```rust
i18n: Some(HashMap::from([
    (langid!("en"), HashMap::from([
        ("my_plugin:label".to_string(), Translation::Single("My Label".to_string())),
        ("my_plugin:files".to_string(), Translation::Plural {
            one:  "{n} file".to_string(),
            few:  "{n} files".to_string(),
            many: "{n} files".to_string(),
        }),
    ])),
    (langid!("uk"), HashMap::from([
        ("my_plugin:label".to_string(), Translation::Single("Моя мітка".to_string())),
        ("my_plugin:files".to_string(), Translation::Plural {
            one:  "{n} файл".to_string(),
            few:  "{n} файли".to_string(),
            many: "{n} файлів".to_string(),
        }),
    ])),
]))
```

### Initialization

`i18n_core/setup_context` provides `Signal<I18n>` and uses `use_effect` to collect and merge translations from all enabled plugins whenever `PluginRegistry` changes.

`i18n_core/entry` subscribes to `ChangeLanguageEvent` and updates `i18n.write().language`.

### Lookup

```rust
let i18n = use_context::<Signal<I18n>>();

i18n.read().get("my_plugin:label")        // → "My Label"
i18n.read().get_plural("my_plugin:files", 3) // → "3 files"
```

`get()` returns the key itself as fallback when translation is missing.
`get_plural()` uses `icu_plurals` (CLDR data) to select the correct plural form for the current locale.

### Language switching

Language names are displayed using `isolang` autonyms (e.g. `"українська"` for `langid!("uk")`). The `top_bar` plugin generates language menu items reactively in `use_memo` by reading `Signal<I18n>`. Clicking a language item publishes `ChangeLanguageEvent { code }` which `i18n_core/entry` handles.

### Reactivity

`Signal<I18n>` is read during component render, so any component that calls `i18n.read().get(...)` automatically re-renders when the language changes.

---

## IPC
for IPC following terms are used:
all data incoming and outgoing from app is just a stream - everything that implements AsyncRead/AsyncWrite
stream + framer = transport

transport gives flume channels to read/write

next you can use multiplexer and split data further for channels

or pass it to connection

connection is then used in protocols

Protocol is used as a wraper around connection.send. If you don't send anything, you don't need protocol, use Connection instead.

codec + transport flume channels = connection

each protocol shares some boilerplate:
```rust
pub struct Protocol<C: Codec> {
	connection: Connection<C>,
}

impl<C: Codec> Protocol<C> {
	pub fn send(&self, msg: C::Message) {
		self.connection.send(msg);
	}

	pub fn try_recv(&self) -> Option<C::Message> {
		self.connection.try_recv().ok().flatten()
	}
}
```

Inside app some plugin should create and start using protocol, fully owning it. Then it should register some resources using `use_context_provider`. This protocol plugin is responsible for creating and updating such resources as well as reacting on resource updates and sending data via protocol.

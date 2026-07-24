use dioxus::prelude::*;
use dioxus_primitives::menubar::{
	self, MenubarContentProps, MenubarItemProps, MenubarMenuProps, MenubarProps, MenubarTriggerProps,
};

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
	rsx! {
		menubar::Menubar {
			class: "menubar",
			disabled: props.disabled,
			roving_loop: props.roving_loop,
			attributes: props.attributes,
			{props.children}
		}
	}
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
	rsx! {
		menubar::MenubarMenu {
			class: "relative group/menu",
			index: props.index,
			disabled: props.disabled,
			attributes: props.attributes,
			{props.children}
		}
	}
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
	rsx! {
		menubar::MenubarTrigger {
			class: "px-3 py-1 rounded text-sm text-text-muted cursor-pointer transition-colors hover:bg-secondary hover:text-text group-data-[state=open]/menu:bg-secondary group-data-[state=open]/menu:text-text data-[disabled=true]:opacity-50 data-[disabled=true]:cursor-not-allowed focus-visible:outline-none",
			attributes: props.attributes,
			{props.children}
		}
	}
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
	rsx! {
		menubar::MenubarContent {
			class: "absolute top-full left-0 mt-1 min-w-44 p-1 rounded bg-secondary border border-border shadow-lg z-50 data-[state=closed]:hidden",
			id: props.id,
			attributes: props.attributes,
			{props.children}
		}
	}
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
	rsx! {
		menubar::MenubarItem {
			class: "flex items-center w-full px-2 py-1.5 rounded text-sm text-text cursor-pointer select-none transition-colors hover:bg-tertiary data-[disabled=true]:opacity-50 data-[disabled=true]:cursor-not-allowed focus-visible:outline-none focus-visible:bg-tertiary",
			index: props.index,
			value: props.value,
			disabled: props.disabled,
			on_select: props.on_select,
			attributes: props.attributes,
			{props.children}
		}
	}
}

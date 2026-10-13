// src/layout/setup.rs

use crate::app::ViewRegistry;
use crate::layout::frame::{SideNav, SideNavItem};
use crate::layout::table::tools::engine::{ModifierButton, ToolbarButton, ToolbarItem};
use crate::layout::table::types::TableAction;
use crate::theme::{ico, tab::TableIcons};
use crate::widget::{
    records_assets::RecordsAssetsWidget,
    records_individuals::RecordsIndividualsWidget,
};

pub fn build_nav_side_left<'a>() -> SideNav<'a> {
    SideNav::left(vec![
        SideNavItem::Action {
            id: "files",
            icon: ico::file(),
            tooltip: "Źródła i pliki",
        },
        SideNavItem::Action {
            id: "search",
            icon: ico::magnifier(),
            tooltip: "Szukaj",
        },
    ])
}

pub fn build_nav_side_right<'a>() -> SideNav<'a> {
    SideNav::right(vec![
        SideNavItem::Action {
            id: "layers",
            icon: ico::puzzle(),
            tooltip: "Warstwy",
        },
        SideNavItem::Action {
            id: "properties",
            icon: ico::notebook(),
            tooltip: "Właściwości",
        },
        SideNavItem::Separator,
        SideNavItem::Action {
            id: "settings",
            icon: ico::gear(),
            tooltip: "Ustawienia",
        },
    ])
}

pub fn build_table_toolbar<'a>() -> Vec<ToolbarItem<'a>> {
    vec![
        // Grupa nawigacji
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Prev,
            label_or_icon: TableIcons::PREV,
            tooltip: "Poprzednie zaznaczenie",
        }),
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Next,
            label_or_icon: TableIcons::NEXT,
            tooltip: "Następne zaznaczenie",
        }),
        ToolbarItem::Spacer(8.0),

        // Grupa edycji / CRUD
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Add,
            label_or_icon: TableIcons::ADD,
            tooltip: "Nowa pozycja",
        }),
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Edit,
            label_or_icon: TableIcons::EDIT,
            tooltip: "Edytuj pozycję",
        }),
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Delete,
            label_or_icon: TableIcons::DELETE,
            tooltip: "Usuń pozycję",
        }),
        ToolbarItem::Spacer(8.0),

        // Grupa zaawansowana
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Merge,
            label_or_icon: TableIcons::MERGE,
            tooltip: "Scal wybrane",
        }),
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Duplicate,
            label_or_icon: TableIcons::DUPLICATE,
            tooltip: "Duplikuj pozycję",
        }),
        ToolbarItem::Spacer(8.0),

        // Grupa tagów i widoku
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::Tag,
            label_or_icon: TableIcons::TAG,
            tooltip: "Etykietuj pozycję",
        }),
        ToolbarItem::Spacer(8.0),
        ToolbarItem::Button(ToolbarButton {
            action: TableAction::ConfigColumns,
            label_or_icon: TableIcons::CONFIG,
            tooltip: "Konfiguruj kolumny",
        }),

        // WIRTUALNE MODYFIKATORY
        ToolbarItem::Spacer(10.0),
        ToolbarItem::Separator,
        ToolbarItem::Spacer(4.0),
        
        ToolbarItem::Modifier(ModifierButton {
            action: TableAction::ToggleCtrl,
            icon: ico::key_ctrl(),
            size: eframe::egui::vec2(29.3, 22.0),
            tooltip: "Wirtualny Ctrl (multiselekcja wierszy)",
        }),
        ToolbarItem::Modifier(ModifierButton {
            action: TableAction::ToggleShift,
            icon: ico::key_shift(),
            size: eframe::egui::vec2(29.3, 22.0),
            tooltip: "Wirtualny Shift (sortowanie wielokolumnowe)",
        }),
        // ToolbarItem::Modifier(ModifierButton {
        //     action: TableAction::ToggleAlt,
        //     icon: ico::key_alt(),
        //     size: eframe::egui::vec2(24.6, 22.0),
        //     tooltip: "Wirtualny Alt",
        // }),
    ]
}

pub fn build_view_registry() -> ViewRegistry {
    let mut reg = ViewRegistry::new();
    reg.register("search", Box::new(RecordsIndividualsWidget::default()));
    reg.register("files", Box::new(RecordsAssetsWidget));
    reg
}
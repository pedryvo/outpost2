//! In-Game Multilingual Internationalization (i18n) Engine
//!
//! Provides on-the-fly live language switching across English, Portuguese (BR),
//! Spanish, German, Simplified Chinese, and Russian.

use web_sys::Document;

pub struct Translation {
    pub faction: &'static str,
    pub sol: &'static str,
    pub sun: &'static str,
    pub pop: &'static str,
    pub power: &'static str,
    pub common_ore: &'static str,
    pub rare_ore: &'static str,
    pub food: &'static str,
    pub build_base: &'static str,
    pub units: &'static str,
    pub research: &'static str,
    pub save: &'static str,
    pub load: &'static str,
    pub radar: &'static str,
}

pub fn get_translation(lang: &str) -> Translation {
    match lang {
        "pt-BR" => Translation {
            faction: "Facção:",
            sol: "Sol:",
            sun: "Sol:",
            pop: "Pop:",
            power: "Energia:",
            common_ore: "Minério Comum:",
            rare_ore: "Minério Raro:",
            food: "Alimento:",
            build_base: "🏗️ Construir Base",
            units: "🚜 Unidades",
            research: "🔬 Pesquisar",
            save: "💾 Salvar",
            load: "📂 Carregar",
            radar: "Radar Tático",
        },
        "es" => Translation {
            faction: "Facción:",
            sol: "Sol:",
            sun: "Sol:",
            pop: "Población:",
            power: "Energía:",
            common_ore: "Mineral Común:",
            rare_ore: "Mineral Raro:",
            food: "Comida:",
            build_base: "🏗️ Construir Base",
            units: "🚜 Unidades",
            research: "🔬 Investigar",
            save: "💾 Guardar",
            load: "📂 Cargar",
            radar: "Radar Táctico",
        },
        "de" => Translation {
            faction: "Fraktion:",
            sol: "Sol:",
            sun: "Sonne:",
            pop: "Bevölkerung:",
            power: "Energie:",
            common_ore: "Erz:",
            rare_ore: "Seltenerz:",
            food: "Nahrung:",
            build_base: "🏗️ Basis Bauen",
            units: "🚜 Einheiten",
            research: "🔬 Forschung",
            save: "💾 Speichern",
            load: "📂 Laden",
            radar: "Taktisches Radar",
        },
        "zh-CN" => Translation {
            faction: "阵营:",
            sol: "火星日:",
            sun: "光照:",
            pop: "人口:",
            power: "电力:",
            common_ore: "普通矿石:",
            rare_ore: "稀有矿石:",
            food: "食物:",
            build_base: "🏗️ 建造基地",
            units: "🚜 作战单位",
            research: "🔬 科技研发",
            save: "💾 存档",
            load: "📂 读档",
            radar: "战术雷达",
        },
        "ru" => Translation {
            faction: "Фракция:",
            sol: "Сол:",
            sun: "Солнце:",
            pop: "Население:",
            power: "Энергия:",
            common_ore: "Обычная Руда:",
            rare_ore: "Редкая Руда:",
            food: "Пища:",
            build_base: "🏗️ Строить Базу",
            units: "🚜 Юниты",
            research: "🔬 Исследования",
            save: "💾 Сохранить",
            load: "📂 Загрузить",
            radar: "Тактический Радар",
        },
        _ => Translation {
            faction: "Faction:",
            sol: "Sol:",
            sun: "Sun:",
            pop: "Pop:",
            power: "Power:",
            common_ore: "Common Ore:",
            rare_ore: "Rare Ore:",
            food: "Food:",
            build_base: "🏗️ Build Base",
            units: "🚜 Units",
            research: "🔬 Research",
            save: "💾 Save",
            load: "📂 Load",
            radar: "Tactical Radar",
        },
    }
}

pub fn apply_language(lang: &str, doc: &Document) {
    let t = get_translation(lang);

    if let Some(el) = doc.get_element_by_id("lbl-faction") { el.set_text_content(Some(t.faction)); }
    if let Some(el) = doc.get_element_by_id("lbl-sol") { el.set_text_content(Some(t.sol)); }
    if let Some(el) = doc.get_element_by_id("lbl-sun") { el.set_text_content(Some(t.sun)); }
    if let Some(el) = doc.get_element_by_id("lbl-pop") { el.set_text_content(Some(t.pop)); }
    if let Some(el) = doc.get_element_by_id("lbl-power") { el.set_text_content(Some(t.power)); }
    if let Some(el) = doc.get_element_by_id("lbl-ore") { el.set_text_content(Some(t.common_ore)); }
    if let Some(el) = doc.get_element_by_id("lbl-rare") { el.set_text_content(Some(t.rare_ore)); }
    if let Some(el) = doc.get_element_by_id("lbl-food") { el.set_text_content(Some(t.food)); }

    if let Some(el) = doc.get_element_by_id("btn-build") { el.set_text_content(Some(t.build_base)); }
    if let Some(el) = doc.get_element_by_id("btn-units") { el.set_text_content(Some(t.units)); }
    if let Some(el) = doc.get_element_by_id("btn-research") { el.set_text_content(Some(t.research)); }
    if let Some(el) = doc.get_element_by_id("btn-save") { el.set_text_content(Some(t.save)); }
    if let Some(el) = doc.get_element_by_id("btn-load") { el.set_text_content(Some(t.load)); }
    if let Some(el) = doc.get_element_by_id("minimap-title") { el.set_text_content(Some(t.radar)); }
}

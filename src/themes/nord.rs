// Some of the colours in the Nord palette aren’t used yet, but might be in the future.
#![allow(unused)]

pub(crate) struct Nord;

// Polar Night
const NORD0: dialect::Rgb = dialect::rgb!(46, 52, 64);
const NORD1: dialect::Rgb = dialect::rgb!(59, 66, 82);
const NORD2: dialect::Rgb = dialect::rgb!(67, 76, 94);
const NORD3: dialect::Rgb = dialect::rgb!(76, 86, 106);

// Snow Storm
const NORD4: dialect::Rgb = dialect::rgb!(216, 222, 233);
const NORD5: dialect::Rgb = dialect::rgb!(229, 233, 240);
const NORD6: dialect::Rgb = dialect::rgb!(236, 239, 244);

// Frost
const NORD7: dialect::Rgb = dialect::rgb!(143, 188, 187);
const NORD8: dialect::Rgb = dialect::rgb!(136, 192, 208);
const NORD9: dialect::Rgb = dialect::rgb!(129, 161, 193);
const NORD10: dialect::Rgb = dialect::rgb!(94, 129, 172);

// Aurora
const NORD11: dialect::Rgb = dialect::rgb!(191, 97, 106);
const NORD12: dialect::Rgb = dialect::rgb!(208, 135, 112);
const NORD13: dialect::Rgb = dialect::rgb!(235, 203, 139);
const NORD14: dialect::Rgb = dialect::rgb!(163, 190, 140);
const NORD15: dialect::Rgb = dialect::rgb!(180, 142, 173);

impl dialect::Theme for Nord {
    fn default_style(&self) -> dialect::ResolvedStyle {
        dialect::ResolvedStyle {
            fg_color: NORD6,
            bg_color: NORD0,
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: dialect::HighlightGroup) -> dialect::Style {
        match group {
            dialect::HighlightGroup::CtrlFlowKeyword | dialect::HighlightGroup::OtherKeyword => {
                dialect::Style {
                    fg_color: Some(NORD9),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::FunctionDef | dialect::HighlightGroup::FunctionCall => {
                dialect::Style {
                    fg_color: Some(NORD8),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::TyDef
            | dialect::HighlightGroup::TyUse
            | dialect::HighlightGroup::InterfaceDef
            | dialect::HighlightGroup::InterfaceUse
            | dialect::HighlightGroup::PrimitiveTy => dialect::Style {
                fg_color: Some(NORD7),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::VariableDef
            | dialect::HighlightGroup::VariableUse
            | dialect::HighlightGroup::MemberDef
            | dialect::HighlightGroup::MemberUse
            | dialect::HighlightGroup::ConstantDef
            | dialect::HighlightGroup::ConstantUse
            | dialect::HighlightGroup::FunctionParam => dialect::Style {
                fg_color: Some(NORD4),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::ModuleDef | dialect::HighlightGroup::ModuleUse => {
                dialect::Style {
                    fg_color: None,
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::MacroDef
            | dialect::HighlightGroup::MacroUse
            | dialect::HighlightGroup::PreProc => dialect::Style {
                fg_color: Some(NORD10),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Unclear what highlighting this should get, as it is not specified by the Nord
            // Specification.
            dialect::HighlightGroup::SpecialIdentDef | dialect::HighlightGroup::SpecialIdentUse => {
                dialect::Style {
                    fg_color: Some(NORD7),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::Number => dialect::Style {
                fg_color: Some(NORD15),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::String | dialect::HighlightGroup::StringDelimiter => {
                dialect::Style {
                    fg_color: Some(NORD14),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::Character | dialect::HighlightGroup::CharacterDelimiter => {
                dialect::Style {
                    fg_color: Some(NORD13),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::Boolean => dialect::Style {
                fg_color: Some(NORD9),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Attribute => dialect::Style {
                fg_color: Some(NORD12),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Comment | dialect::HighlightGroup::DocComment => {
                dialect::Style {
                    fg_color: Some(NORD3),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::MemberOper
            | dialect::HighlightGroup::PointerOper
            | dialect::HighlightGroup::AssignOper
            | dialect::HighlightGroup::BinaryOper
            | dialect::HighlightGroup::OtherOper
            | dialect::HighlightGroup::Separator
            | dialect::HighlightGroup::Terminator => dialect::Style {
                fg_color: Some(NORD9),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Delimiter => dialect::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Error => dialect::Style {
                fg_color: Some(NORD11),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
        }
    }
}

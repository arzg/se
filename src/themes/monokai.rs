pub(crate) struct Monokai;

const FADED: dialect::Rgb = dialect::rgb!(117, 113, 94);

const CYAN: dialect::Rgb = dialect::rgb!(102, 217, 239);
const GREEN: dialect::Rgb = dialect::rgb!(166, 226, 46);
const ORANGE: dialect::Rgb = dialect::rgb!(253, 151, 31);
const PINK: dialect::Rgb = dialect::rgb!(249, 38, 114);
const PURPLE: dialect::Rgb = dialect::rgb!(174, 129, 255);
const YELLOW: dialect::Rgb = dialect::rgb!(230, 219, 116);

impl dialect::Theme for Monokai {
    fn default_style(&self) -> dialect::ResolvedStyle {
        dialect::ResolvedStyle {
            fg_color: dialect::rgb!(248, 248, 242),
            bg_color: dialect::rgb!(39, 40, 34),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: dialect::HighlightGroup) -> dialect::Style {
        match group {
            // Control flow and operators
            dialect::HighlightGroup::CtrlFlowKeyword
            | dialect::HighlightGroup::PointerOper
            | dialect::HighlightGroup::AssignOper
            | dialect::HighlightGroup::BinaryOper
            | dialect::HighlightGroup::OtherOper => dialect::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::OtherKeyword => dialect::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            dialect::HighlightGroup::FunctionDef
            | dialect::HighlightGroup::TyDef
            | dialect::HighlightGroup::InterfaceDef
            | dialect::HighlightGroup::MacroDef => dialect::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::FunctionCall
            | dialect::HighlightGroup::MacroUse
            | dialect::HighlightGroup::PreProc => dialect::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::TyUse
            | dialect::HighlightGroup::InterfaceUse
            | dialect::HighlightGroup::PrimitiveTy => dialect::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Variables, members and modules don’t get any highlighting
            dialect::HighlightGroup::VariableDef
            | dialect::HighlightGroup::VariableUse
            | dialect::HighlightGroup::MemberDef
            | dialect::HighlightGroup::MemberUse
            | dialect::HighlightGroup::ModuleDef
            | dialect::HighlightGroup::ModuleUse => dialect::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::SpecialIdentDef | dialect::HighlightGroup::SpecialIdentUse => {
                dialect::Style {
                    fg_color: Some(PURPLE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::FunctionParam => dialect::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Constants and literals (apart from strings)
            dialect::HighlightGroup::ConstantDef
            | dialect::HighlightGroup::ConstantUse
            | dialect::HighlightGroup::Number
            | dialect::HighlightGroup::Character
            | dialect::HighlightGroup::CharacterDelimiter
            | dialect::HighlightGroup::Boolean => dialect::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::String | dialect::HighlightGroup::StringDelimiter => {
                dialect::Style {
                    fg_color: Some(YELLOW),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::Comment | dialect::HighlightGroup::DocComment => {
                dialect::Style {
                    fg_color: Some(FADED),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::Attribute => dialect::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Error => dialect::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },

            // Miscellaneous punctuation that doesn’t get highlighted
            dialect::HighlightGroup::MemberOper
            | dialect::HighlightGroup::Delimiter
            | dialect::HighlightGroup::Separator
            | dialect::HighlightGroup::Terminator => dialect::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
        }
    }
}

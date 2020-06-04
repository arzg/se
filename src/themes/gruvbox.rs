pub(crate) struct Gruvbox;

const GRAY: dialect::Rgb = dialect::rgb!(189, 174, 147);
const RED: dialect::Rgb = dialect::rgb!(251, 73, 52);
const GREEN: dialect::Rgb = dialect::rgb!(184, 187, 38);
const YELLOW: dialect::Rgb = dialect::rgb!(250, 189, 47);
const BLUE: dialect::Rgb = dialect::rgb!(131, 165, 152);
const PURPLE: dialect::Rgb = dialect::rgb!(211, 134, 155);
const AQUA: dialect::Rgb = dialect::rgb!(142, 192, 124);
const ORANGE: dialect::Rgb = dialect::rgb!(254, 128, 25);

impl dialect::Theme for Gruvbox {
    fn default_style(&self) -> dialect::ResolvedStyle {
        dialect::ResolvedStyle {
            fg_color: dialect::rgb!(235, 219, 178),
            bg_color: dialect::rgb!(29, 32, 33),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: dialect::HighlightGroup) -> dialect::Style {
        match group {
            // Keywords
            dialect::HighlightGroup::CtrlFlowKeyword | dialect::HighlightGroup::OtherKeyword => {
                dialect::Style {
                    fg_color: Some(RED),
                    bg_color: None,
                    is_bold: true,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Functions
            dialect::HighlightGroup::FunctionDef | dialect::HighlightGroup::FunctionCall => {
                dialect::Style {
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Types
            dialect::HighlightGroup::TyDef
            | dialect::HighlightGroup::TyUse
            | dialect::HighlightGroup::InterfaceDef
            | dialect::HighlightGroup::InterfaceUse
            | dialect::HighlightGroup::PrimitiveTy => dialect::Style {
                fg_color: Some(YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Variables
            dialect::HighlightGroup::VariableDef
            | dialect::HighlightGroup::VariableUse
            | dialect::HighlightGroup::MemberDef
            | dialect::HighlightGroup::MemberUse
            | dialect::HighlightGroup::FunctionParam => dialect::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Constants
            dialect::HighlightGroup::ConstantDef
            | dialect::HighlightGroup::ConstantUse
            | dialect::HighlightGroup::Number
            | dialect::HighlightGroup::Boolean => dialect::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Modules
            dialect::HighlightGroup::ModuleDef | dialect::HighlightGroup::ModuleUse => {
                dialect::Style {
                    fg_color: Some(BLUE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Preprocessor-related
            dialect::HighlightGroup::MacroDef
            | dialect::HighlightGroup::MacroUse
            | dialect::HighlightGroup::PreProc
            | dialect::HighlightGroup::Attribute => dialect::Style {
                fg_color: Some(AQUA),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // String and character literals
            dialect::HighlightGroup::String
            | dialect::HighlightGroup::StringDelimiter
            | dialect::HighlightGroup::Character
            | dialect::HighlightGroup::CharacterDelimiter => dialect::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Special identifiers
            dialect::HighlightGroup::SpecialIdentDef | dialect::HighlightGroup::SpecialIdentUse => {
                dialect::Style {
                    fg_color: Some(ORANGE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Comments
            dialect::HighlightGroup::Comment | dialect::HighlightGroup::DocComment => {
                dialect::Style {
                    fg_color: Some(GRAY),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Punctuation
            dialect::HighlightGroup::MemberOper
            | dialect::HighlightGroup::PointerOper
            | dialect::HighlightGroup::AssignOper
            | dialect::HighlightGroup::BinaryOper
            | dialect::HighlightGroup::OtherOper
            | dialect::HighlightGroup::Delimiter
            | dialect::HighlightGroup::Separator
            | dialect::HighlightGroup::Terminator => dialect::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Errors
            dialect::HighlightGroup::Error => dialect::Style {
                fg_color: Some(RED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },
        }
    }
}

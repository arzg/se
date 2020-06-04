pub(crate) struct Dracula;

const FADED: dialect::Rgb = dialect::rgb!(98, 114, 164);
const RED: dialect::Rgb = dialect::rgb!(255, 85, 85);
const ORANGE: dialect::Rgb = dialect::rgb!(255, 184, 108);
const YELLOW: dialect::Rgb = dialect::rgb!(241, 250, 140);
const GREEN: dialect::Rgb = dialect::rgb!(80, 250, 123);
const PURPLE: dialect::Rgb = dialect::rgb!(189, 147, 249);
const CYAN: dialect::Rgb = dialect::rgb!(139, 233, 253);
const PINK: dialect::Rgb = dialect::rgb!(255, 121, 198);

impl dialect::Theme for Dracula {
    fn default_style(&self) -> dialect::ResolvedStyle {
        dialect::ResolvedStyle {
            fg_color: dialect::rgb!(248, 248, 242),
            bg_color: dialect::rgb!(40, 42, 54),
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
                    fg_color: Some(PINK),
                    bg_color: None,
                    is_bold: false,
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
            | dialect::HighlightGroup::PrimitiveTy => dialect::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Interfaces
            dialect::HighlightGroup::InterfaceDef | dialect::HighlightGroup::InterfaceUse => {
                dialect::Style {
                    fg_color: Some(CYAN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: true,
                    is_underline: false,
                }
            }

            // Variables
            dialect::HighlightGroup::VariableDef
            | dialect::HighlightGroup::VariableUse
            | dialect::HighlightGroup::MemberDef
            | dialect::HighlightGroup::MemberUse => dialect::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Function parameters
            dialect::HighlightGroup::FunctionParam => dialect::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Constants
            dialect::HighlightGroup::ConstantDef
            | dialect::HighlightGroup::ConstantUse
            | dialect::HighlightGroup::Number
            | dialect::HighlightGroup::Boolean
            | dialect::HighlightGroup::Character
            | dialect::HighlightGroup::CharacterDelimiter => dialect::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Modules
            dialect::HighlightGroup::ModuleDef | dialect::HighlightGroup::ModuleUse => {
                dialect::Style {
                    fg_color: None,
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Macros and other preprocessor-related highlight groups
            dialect::HighlightGroup::MacroDef
            | dialect::HighlightGroup::MacroUse
            | dialect::HighlightGroup::PreProc => dialect::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Attributes
            dialect::HighlightGroup::Attribute => dialect::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Strings
            dialect::HighlightGroup::String | dialect::HighlightGroup::StringDelimiter => {
                dialect::Style {
                    fg_color: Some(YELLOW),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Special identifiers
            dialect::HighlightGroup::SpecialIdentDef | dialect::HighlightGroup::SpecialIdentUse => {
                dialect::Style {
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Comments
            dialect::HighlightGroup::Comment | dialect::HighlightGroup::DocComment => {
                dialect::Style {
                    fg_color: Some(FADED),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Some punctuation gets a colour
            dialect::HighlightGroup::PointerOper
            | dialect::HighlightGroup::AssignOper
            | dialect::HighlightGroup::BinaryOper
            | dialect::HighlightGroup::OtherOper
            | dialect::HighlightGroup::Separator => dialect::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Other punctuation doesn’t
            dialect::HighlightGroup::MemberOper
            | dialect::HighlightGroup::Delimiter
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
                is_underline: false,
            },
        }
    }
}

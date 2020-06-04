pub(crate) struct Seoul;

const BLUE: dialect::Rgb = dialect::rgb!(152, 190, 222);
const BROWN: dialect::Rgb = dialect::rgb!(190, 152, 115);
const CREAM: dialect::Rgb = dialect::rgb!(223, 222, 189);
const CYAN: dialect::Rgb = dialect::rgb!(111, 188, 189);
const DARK_GREEN: dialect::Rgb = dialect::rgb!(113, 152, 114);
const GREEN: dialect::Rgb = dialect::rgb!(152, 188, 153);
const KHAKI: dialect::Rgb = dialect::rgb!(189, 187, 114);
const LEMON: dialect::Rgb = dialect::rgb!(222, 221, 153);
const LIGHT_BLUE: dialect::Rgb = dialect::rgb!(152, 188, 189);
const LIGHT_YELLOW: dialect::Rgb = dialect::rgb!(255, 222, 153);
const PURPLE: dialect::Rgb = dialect::rgb!(225, 120, 153);
const SALMON: dialect::Rgb = dialect::rgb!(255, 191, 189);
const VIOLET: dialect::Rgb = dialect::rgb!(153, 154, 189);
const YELLOW: dialect::Rgb = dialect::rgb!(223, 188, 114);

impl dialect::Theme for Seoul {
    fn default_style(&self) -> dialect::ResolvedStyle {
        dialect::ResolvedStyle {
            fg_color: dialect::rgb!(217, 217, 217),
            bg_color: dialect::rgb!(75, 75, 75),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: dialect::HighlightGroup) -> dialect::Style {
        match group {
            dialect::HighlightGroup::CtrlFlowKeyword => dialect::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::OtherKeyword => dialect::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::FunctionDef | dialect::HighlightGroup::FunctionCall => {
                dialect::Style {
                    fg_color: Some(CREAM),
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
                fg_color: Some(YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::VariableDef
            | dialect::HighlightGroup::VariableUse
            | dialect::HighlightGroup::FunctionParam => dialect::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::MemberDef | dialect::HighlightGroup::MemberUse => {
                dialect::Style {
                    fg_color: Some(SALMON),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::ConstantDef | dialect::HighlightGroup::ConstantUse => {
                dialect::Style {
                    fg_color: Some(SALMON),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

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
                fg_color: Some(KHAKI),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::SpecialIdentDef | dialect::HighlightGroup::SpecialIdentUse => {
                dialect::Style {
                    fg_color: Some(CYAN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::Number => dialect::Style {
                fg_color: Some(LIGHT_YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::String | dialect::HighlightGroup::Character => {
                dialect::Style {
                    fg_color: Some(LIGHT_BLUE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::StringDelimiter
            | dialect::HighlightGroup::CharacterDelimiter => dialect::Style {
                fg_color: Some(BROWN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Boolean => dialect::Style {
                fg_color: Some(VIOLET),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Attribute => dialect::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Comment | dialect::HighlightGroup::DocComment => {
                dialect::Style {
                    fg_color: Some(DARK_GREEN),
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
            | dialect::HighlightGroup::OtherOper => dialect::Style {
                fg_color: Some(LEMON),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Separator
            | dialect::HighlightGroup::Delimiter
            | dialect::HighlightGroup::Terminator => dialect::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Error => dialect::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },
        }
    }
}

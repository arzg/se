pub(crate) struct DarkPlus;

const DARK_BLUE: dialect::Rgb = dialect::rgb!(86, 156, 214);
const DARK_GREEN: dialect::Rgb = dialect::rgb!(107, 153, 85);
const DULL_GREEN_DARKER: dialect::Rgb = dialect::rgb!(181, 206, 168);
const FADED: dialect::Rgb = dialect::rgb!(178, 178, 178);
const GREEN: dialect::Rgb = dialect::rgb!(134, 198, 145);
const DULL_GREEN: dialect::Rgb = dialect::rgb!(184, 215, 163);
const LIGHT_BLUE: dialect::Rgb = dialect::rgb!(156, 220, 254);
const ORANGE: dialect::Rgb = dialect::rgb!(206, 144, 120);
const PURPLE: dialect::Rgb = dialect::rgb!(197, 134, 192);
const RED: dialect::Rgb = dialect::rgb!(244, 71, 71);
const TEAL: dialect::Rgb = dialect::rgb!(78, 201, 176);
const YELLOW: dialect::Rgb = dialect::rgb!(220, 220, 170);

impl dialect::Theme for DarkPlus {
    fn default_style(&self) -> dialect::ResolvedStyle {
        dialect::ResolvedStyle {
            fg_color: dialect::rgb!(212, 212, 212),
            bg_color: dialect::rgb!(30, 30, 30),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: dialect::HighlightGroup) -> dialect::Style {
        match group {
            dialect::HighlightGroup::CtrlFlowKeyword => dialect::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Keywords and things that are often treated as such
            dialect::HighlightGroup::OtherKeyword
            | dialect::HighlightGroup::PrimitiveTy
            | dialect::HighlightGroup::Boolean => dialect::Style {
                fg_color: Some(DARK_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Call-able things
            dialect::HighlightGroup::FunctionDef
            | dialect::HighlightGroup::FunctionCall
            | dialect::HighlightGroup::MacroDef
            | dialect::HighlightGroup::MacroUse => dialect::Style {
                fg_color: Some(YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::TyDef | dialect::HighlightGroup::TyUse => dialect::Style {
                fg_color: Some(TEAL),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::InterfaceDef | dialect::HighlightGroup::InterfaceUse => {
                dialect::Style {
                    fg_color: Some(DULL_GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::VariableDef
            | dialect::HighlightGroup::VariableUse
            | dialect::HighlightGroup::MemberDef
            | dialect::HighlightGroup::MemberUse
            | dialect::HighlightGroup::ConstantDef
            | dialect::HighlightGroup::ConstantUse
            | dialect::HighlightGroup::FunctionParam => dialect::Style {
                fg_color: Some(LIGHT_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::SpecialIdentDef | dialect::HighlightGroup::SpecialIdentUse => {
                dialect::Style {
                    // This colour is actually used for structs, but the distinction between
                    // structs and other types is only possible through semantic highlighting -- it
                    // is expected that all highlighters will either be simple lexers or parsers.
                    //
                    // Since ‘special identifiers’ are unique in the languages that they occur in
                    // (e.g.  lifetimes in Rust, symbols in Ruby), it makes sense to give them a
                    // special colour. This colour was left over, so I decided to use it.
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Modules aren’t highlighted
            dialect::HighlightGroup::ModuleDef | dialect::HighlightGroup::ModuleUse => {
                dialect::Style {
                    fg_color: None,
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            dialect::HighlightGroup::Number => dialect::Style {
                fg_color: Some(DULL_GREEN_DARKER),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::String
            | dialect::HighlightGroup::StringDelimiter
            | dialect::HighlightGroup::Character
            | dialect::HighlightGroup::CharacterDelimiter => dialect::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::PreProc => dialect::Style {
                fg_color: Some(DARK_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            dialect::HighlightGroup::Attribute => dialect::Style {
                fg_color: Some(FADED),
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

            // Punctuation
            dialect::HighlightGroup::MemberOper
            | dialect::HighlightGroup::PointerOper
            | dialect::HighlightGroup::AssignOper
            | dialect::HighlightGroup::BinaryOper
            | dialect::HighlightGroup::OtherOper
            | dialect::HighlightGroup::Delimiter
            | dialect::HighlightGroup::Separator
            | dialect::HighlightGroup::Terminator => dialect::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

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

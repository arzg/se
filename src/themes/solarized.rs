const BASE03: dialect::Rgb = dialect::rgb!(0, 43, 54);
const BASE01: dialect::Rgb = dialect::rgb!(88, 110, 117);
const BASE00: dialect::Rgb = dialect::rgb!(101, 123, 131);
const BASE0: dialect::Rgb = dialect::rgb!(131, 148, 150);
const BASE1: dialect::Rgb = dialect::rgb!(147, 161, 161);
const BASE3: dialect::Rgb = dialect::rgb!(253, 246, 227);
const YELLOW: dialect::Rgb = dialect::rgb!(181, 137, 0);
const ORANGE: dialect::Rgb = dialect::rgb!(203, 75, 22);
const RED: dialect::Rgb = dialect::rgb!(220, 50, 47);
const BLUE: dialect::Rgb = dialect::rgb!(38, 139, 210);
const CYAN: dialect::Rgb = dialect::rgb!(42, 161, 152);
const GREEN: dialect::Rgb = dialect::rgb!(133, 153, 0);

macro_rules! create_solarized_theme {
    ($theme_name: ident, $fg: expr, $bg: expr, $deemphasized: expr) => {
        pub(crate) struct $theme_name;

        impl dialect::Theme for $theme_name {
            fn default_style(&self) -> dialect::ResolvedStyle {
                dialect::ResolvedStyle {
                    fg_color: $fg,
                    bg_color: $bg,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            fn style(&self, group: dialect::HighlightGroup) -> dialect::Style {
                match group {
                    dialect::HighlightGroup::CtrlFlowKeyword
                    | dialect::HighlightGroup::OtherKeyword => dialect::Style {
                        fg_color: Some(GREEN),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // ‘Identifiers’ (functions, variables, modules)
                    dialect::HighlightGroup::FunctionDef
                    | dialect::HighlightGroup::FunctionCall
                    | dialect::HighlightGroup::VariableDef
                    | dialect::HighlightGroup::VariableUse
                    | dialect::HighlightGroup::MemberDef
                    | dialect::HighlightGroup::MemberUse
                    | dialect::HighlightGroup::SpecialIdentDef
                    | dialect::HighlightGroup::SpecialIdentUse
                    | dialect::HighlightGroup::FunctionParam
                    | dialect::HighlightGroup::ModuleDef
                    | dialect::HighlightGroup::ModuleUse => dialect::Style {
                        fg_color: Some(BLUE),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // Constants of any kind
                    dialect::HighlightGroup::ConstantDef
                    | dialect::HighlightGroup::ConstantUse
                    | dialect::HighlightGroup::Number
                    | dialect::HighlightGroup::String
                    | dialect::HighlightGroup::StringDelimiter
                    | dialect::HighlightGroup::Character
                    | dialect::HighlightGroup::CharacterDelimiter
                    | dialect::HighlightGroup::Boolean => dialect::Style {
                        fg_color: Some(CYAN),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

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

                    dialect::HighlightGroup::PreProc
                    | dialect::HighlightGroup::MacroDef
                    | dialect::HighlightGroup::MacroUse => dialect::Style {
                        fg_color: Some(ORANGE),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // Punctuation gets no highlighting
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

                    dialect::HighlightGroup::Comment | dialect::HighlightGroup::DocComment => {
                        dialect::Style {
                            fg_color: Some($deemphasized),
                            bg_color: None,
                            is_bold: false,
                            is_italic: true,
                            is_underline: false,
                        }
                    }

                    dialect::HighlightGroup::Attribute => dialect::Style {
                        fg_color: Some(GREEN),
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
    };
}

create_solarized_theme!(SolarizedLight, BASE00, BASE3, BASE1);
create_solarized_theme!(SolarizedDark, BASE0, BASE03, BASE01);

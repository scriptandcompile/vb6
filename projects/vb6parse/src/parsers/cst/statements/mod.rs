mod assignment;
mod attribute_statements;
mod control_ops;
mod declarations;
mod deftype_statements;
mod enum_statements;
mod file_ops;
mod for_statements;
mod fs_ops;
mod function_statements;
mod if_statements;
mod label_statements;
mod loop_statements;
mod object_ops;
mod option_statements;
mod parameters;
mod properties;
mod property_statements;
mod runtime_ops;
mod select_statements;
mod string_ops;
mod sub_statements;
mod sys_ops;
mod type_statements;

use super::Parser;
use crate::language::Token;

impl Parser<'_> {
    /// Check if the current token is a library statement keyword.
    ///
    /// Special handling:
    /// - `ErrorKeyword` followed by `DollarSign` is NOT a statement (it's the `Error$` function)
    /// - `ErrorKeyword` followed by `=` is NOT a statement (it's an assignment)
    /// - `MidKeyword` followed by `DollarSign` is NOT a statement (it's the `Mid$` function)
    ///
    /// Checks both current position and next non-whitespace token.
    pub(crate) fn is_library_statement_keyword(&self) -> bool {
        // Special case: keyword/identifier + DollarSign is a function, not a statement
        if self.at_keyword_dollar() {
            return false;
        }

        let token = if self.at_token(Token::Whitespace) {
            self.peek_next_keyword()
        } else {
            self.current_token().copied()
        };

        // Special case: `Error = ...` is an assignment, not the Error statement.
        // Look ahead past whitespace to check if Error is followed by `=`.
        if matches!(token, Some(Token::ErrorKeyword)) {
            let tokens_after: Vec<Token> = self
                .tokens
                .iter()
                .skip(self.pos)
                .filter(|(_text, t)| !matches!(t, Token::Whitespace))
                .map(|(_, t)| *t)
                .collect();
            if let Some(&Token::EqualityOperator) = tokens_after.get(1) {
                return false;
            }
        }

        matches!(
            token,
            Some(
                Token::AppActivateKeyword
                    | Token::BeepKeyword
                    | Token::ChDirKeyword
                    | Token::ChDriveKeyword
                    | Token::CloseKeyword
                    | Token::DateKeyword
                    | Token::DeleteSettingKeyword
                    | Token::ErrorKeyword
                    | Token::FileCopyKeyword
                    | Token::GetKeyword
                    | Token::PutKeyword
                    | Token::InputKeyword
                    | Token::KillKeyword
                    | Token::LineKeyword
                    | Token::LoadKeyword
                    | Token::UnloadKeyword
                    | Token::LockKeyword
                    | Token::UnlockKeyword
                    | Token::LSetKeyword
                    | Token::MidKeyword
                    | Token::MidBKeyword
                    | Token::MkDirKeyword
                    | Token::NameKeyword
                    | Token::OpenKeyword
                    | Token::PrintKeyword
                    | Token::RandomizeKeyword
                    | Token::ResetKeyword
                    | Token::RmDirKeyword
                    | Token::RSetKeyword
                    | Token::SavePictureKeyword
                    | Token::SaveSettingKeyword
                    | Token::SeekKeyword
                    | Token::SendKeysKeyword
                    | Token::SetAttrKeyword
                    | Token::StopKeyword
                    | Token::TimeKeyword
                    | Token::WidthKeyword
                    | Token::WriteKeyword
            )
        )
    }

    /// Start of dispatch library statement parsing to the appropriate parser.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse_library_statement(&mut self) {
        let token = if self.at_token(Token::Whitespace) {
            self.peek_next_keyword()
        } else {
            self.current_token().copied()
        };

        match token {
            Some(Token::AppActivateKeyword) => {
                self.parse_app_activate_statement();
            }
            Some(Token::BeepKeyword) => {
                self.parse_beep_statement();
            }
            Some(Token::ChDirKeyword) => {
                self.parse_ch_dir_statement();
            }
            Some(Token::ChDriveKeyword) => {
                self.parse_ch_drive_statement();
            }
            Some(Token::CloseKeyword) => {
                self.parse_close_statement();
            }
            Some(Token::DateKeyword) => {
                self.parse_date_statement();
            }
            Some(Token::DeleteSettingKeyword) => {
                self.parse_delete_setting_statement();
            }
            Some(Token::ErrorKeyword) => {
                self.parse_error_statement();
            }
            Some(Token::FileCopyKeyword) => {
                self.parse_file_copy_statement();
            }
            Some(Token::GetKeyword) => {
                self.parse_get_statement();
            }
            Some(Token::PutKeyword) => {
                self.parse_put_statement();
            }
            Some(Token::InputKeyword) => {
                self.parse_input_statement();
            }
            Some(Token::KillKeyword) => {
                self.parse_kill_statement();
            }
            Some(Token::LineKeyword) => {
                self.parse_line_input_statement();
            }
            Some(Token::LoadKeyword) => {
                self.parse_load_statement();
            }
            Some(Token::UnloadKeyword) => {
                self.parse_unload_statement();
            }
            Some(Token::LockKeyword) => {
                self.parse_lock_statement();
            }
            Some(Token::UnlockKeyword) => {
                self.parse_unlock_statement();
            }
            Some(Token::LSetKeyword) => {
                self.parse_lset_statement();
            }
            Some(Token::MidKeyword) => {
                self.parse_mid_statement();
            }
            Some(Token::MidBKeyword) => {
                self.parse_midb_statement();
            }
            Some(Token::MkDirKeyword) => {
                self.parse_mkdir_statement();
            }
            Some(Token::NameKeyword) => {
                self.parse_name_statement();
            }
            Some(Token::OpenKeyword) => {
                self.parse_open_statement();
            }
            Some(Token::PrintKeyword) => {
                self.parse_print_statement();
            }
            Some(Token::RandomizeKeyword) => {
                self.parse_randomize_statement();
            }
            Some(Token::ResetKeyword) => {
                self.parse_reset_statement();
            }
            Some(Token::RmDirKeyword) => {
                self.parse_rmdir_statement();
            }
            Some(Token::RSetKeyword) => {
                self.parse_rset_statement();
            }
            Some(Token::SavePictureKeyword) => {
                self.parse_savepicture_statement();
            }
            Some(Token::SaveSettingKeyword) => {
                self.parse_savesetting_statement();
            }
            Some(Token::SeekKeyword) => {
                self.parse_seek_statement();
            }
            Some(Token::SendKeysKeyword) => {
                self.parse_sendkeys_statement();
            }
            Some(Token::SetAttrKeyword) => {
                self.parse_setattr_statement();
            }
            Some(Token::StopKeyword) => {
                self.parse_stop_statement();
            }
            Some(Token::TimeKeyword) => {
                self.parse_time_statement();
            }
            Some(Token::WidthKeyword) => {
                self.parse_width_statement();
            }
            Some(Token::WriteKeyword) => {
                self.parse_write_statement();
            }
            _ => {}
        }
    }
}

//! The designer section of a form or class -- the `VERSION` header and the
//! `Begin ... End` block -- is written and read by the VB6 IDE itself, and
//! holds the control geometry. The formatter must copy it through untouched.

mod common;

#[test]
fn form_designer_block_is_left_alone() {
    common::assert_stable(concat!(
        "VERSION 5.00\r\n",
        "Begin VB.Form FrmSave \r\n",
        "   BorderStyle     =   4  'Fixed ToolWindow\r\n",
        "   Caption         =   \"Save\"\r\n",
        "   ClientHeight    =   1695\r\n",
        "   Begin VB.CommandButton Btn \r\n",
        "      Height          =   360\r\n",
        "      TabIndex        =   0\r\n",
        "   End\r\n",
        "End\r\n",
        "Attribute VB_Name = \"FrmSave\"\r\n",
    ));
}

#[test]
fn version_header_keeps_its_casing() {
    // `VERSION` is upper case in every file the IDE writes; the keyword pass
    // must not turn it into `Version`.
    let source = concat!(
        "VERSION 1.0 CLASS\r\n",
        "BEGIN\r\n",
        "  MultiUse = -1  'True\r\n",
        "END\r\n",
        "Attribute VB_Name = \"CFoo\"\r\n",
    );

    common::assert_stable(source);
}

#[test]
fn code_after_the_designer_block_is_still_formatted() {
    // Skipping the block must not switch the formatter off for the rest of the
    // file.
    common::assert_fmt(
        concat!(
            "VERSION 5.00\r\n",
            "Begin VB.Form FrmSave \r\n",
            "   ClientHeight    =   1695\r\n",
            "End\r\n",
            "Attribute VB_Name = \"FrmSave\"\r\n",
            "Private Sub Btn_Click()\r\n",
            "Dim x As Integer\r\n",
            "End Sub\r\n",
        ),
        concat!(
            "VERSION 5.00\r\n",
            "Begin VB.Form FrmSave \r\n",
            "   ClientHeight    =   1695\r\n",
            "End\r\n",
            "Attribute VB_Name = \"FrmSave\"\r\n",
            "Private Sub Btn_Click()\r\n",
            "    Dim x As Integer\r\n",
            "End Sub\r\n",
        ),
    );
}

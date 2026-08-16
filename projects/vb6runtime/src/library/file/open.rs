//! VB6 Open statement syntax:
//! - Open pathname For mode [Access access] [lock] As [#]filenumber [Len=reclength]
//!
//! Enables input/output (I/O) to a file.
//!
//!
//! The Open statement syntax has these parts:
//!
//! | Part       | Description |
//! |------------|-------------|
//! | pathname   | Required. String expression that specifies a file name — may include directory or folder, and drive. |
//! | mode       | Required. Keyword specifying the file mode: Append, Binary, Input, Output, or Random. If unspecified, the file is opened for Random access. |
//! | access     | Optional. Keyword specifying the operations permitted on the open file: Read, Write, or Read Write. |
//! | lock       | Optional. Keyword specifying the operations restricted on the open file by other processes: Shared, Lock Read, Lock Write, and Lock Read Write. |
//! | filenumber | Required. A valid file number in the range 1 to 511, inclusive. Use the FreeFile function to obtain the next available file number. |
//! | reclength  | Optional. Number less than or equal to 32,767 (bytes). For files opened for random access, this value is the record length. For sequential files, this value is the number of characters buffered. |
//!
//! ## Remarks
//!
//! - You must open a file before any I/O operation can be performed on it.
//! - If pathname specifies a file that doesn't exist, it is created when a file is opened for Append, Binary, Output, or Random modes.
//! - If the file is already opened by another process and the specified type of access is not allowed, the Open operation fails and an error occurs.
//! - The Len clause is ignored if mode is Binary.
//! - In Binary, Input, and Random modes, you can open a file using a different file number without first closing the file. In Append and Output modes, you must close a file before opening it with a different file number.
//!
//! ## Examples
//!
//! ```vb
//! ' Open for input
//! Open "TESTFILE" For Input As #1
//!
//! ' Open for output
//! Open "TESTFILE" For Output As #1
//!
//! ' Open for append
//! Open "TESTFILE" For Append As #1
//!
//! ' Open for binary
//! Open "TESTFILE" For Binary As #1
//!
//! ' Open for random with record length
//! Open "TESTFILE" For Random As #1 Len = 512
//!
//! ' Open with access control
//! Open "TESTFILE" For Input Access Read As #1
//!
//! ' Open with locking
//! Open "TESTFILE" For Binary Lock Read Write As #1
//!
//! ' Open with variable
//! Dim fileNum As Integer
//! fileNum = FreeFile
//! Open fileName For Input As fileNum
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/open-statement)

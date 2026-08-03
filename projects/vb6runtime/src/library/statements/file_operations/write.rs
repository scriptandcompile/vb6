//! # Write Statement
//!
//! Writes data to a sequential file.
//!
//! ## Syntax
//!
//! ```vb
//! Write #filenumber, [outputlist]
//! ```
//!
//! ## Parts
//!
//! - **filenumber**: Required. Any valid file number.
//! - **outputlist**: Optional. One or more comma-delimited numeric expressions or string expressions
//!   to write to a file.
//!
//! ## Remarks
//!
//! - **Data Formatting**: Data written with Write # is usually read from a file with Input #.
//! - **Delimiters**: The Write # statement inserts commas between items and quotation marks around
//!   strings as they are written to the file. You don't have to put explicit delimiters in the list.
//! - **Universal Data**: Write # writes data in a universal format that can be read by Input # regardless
//!   of the locale settings.
//! - **Numeric Data**: Numeric data is written with a period (.) as the decimal separator.
//! - **Boolean Values**: Boolean data is written as #TRUE# or #FALSE#.
//! - **Date Values**: Date data is written using the universal date format: #yyyy-mm-dd hh:mm:ss#
//! - **Empty Values**: If outputlist data is Empty, nothing is written. However, if outputlist data is
//!   Null, #NULL# is written.
//! - **Error Data**: Error values are written as #ERROR errorcode#. The number sign (#) ensures the keyword
//!   is not confused with a variable name.
//! - **Comparison with Print #**: Unlike Print #, Write # inserts commas between items and quotes around
//!   strings automatically.
//!
//! ## Examples
//!
//! ### Write Simple Data
//!
//! ```vb
//! Open "test.txt" For Output As #1
//! Write #1, "Hello", 42, True
//! Close #1
//! ' File contents: "Hello",42,#TRUE#
//! ```
//!
//! ### Write Multiple Lines
//!
//! ```vb
//! Open "data.txt" For Output As #1
//! For i = 1 To 10
//!     Write #1, i, i * i, i * i * i
//! Next i
//! Close #1
//! ```
//!
//! ### Write Mixed Data Types
//!
//! ```vb
//! Open "record.txt" For Output As #1
//! Write #1, "John Doe", 30, #1/1/1995#, True
//! Close #1
//! ```
//!
//! ### Write Without Data (New Line)
//!
//! ```vb
//! Open "output.txt" For Output As #1
//! Write #1, "First line"
//! Write #1
//! Write #1, "Third line"
//! Close #1
//! ```
//!
//! ### Write Null and Empty
//!
//! ```vb
//! Open "test.txt" For Output As #1
//! Write #1, Null, Empty, "data"
//! Close #1
//! ' File contents: #NULL#,,"data"
//! ```
//!
//! ### Write Error Values
//!
//! ```vb
//! Open "errors.txt" For Output As #1
//! Write #1, CVErr(2007)
//! Close #1
//! ' File contents: #ERROR 2007#
//! ```
//!
//! ## Common Patterns
//!
//! ### Export Data to CSV-like Format
//!
//! ```vb
//! Sub ExportData()
//!     Open "export.txt" For Output As #1
//!     
//!     ' Write header
//!     Write #1, "Name", "Age", "City"
//!     
//!     ' Write data rows
//!     For i = 0 To UBound(employees)
//!         Write #1, employees(i).Name, employees(i).Age, employees(i).City
//!     Next i
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### Write Database Records
//!
//! ```vb
//! Sub SaveRecords()
//!     Open "records.dat" For Output As #1
//!     
//!     Do Until rs.EOF
//!         Write #1, rs!ID, rs!Name, rs!Date, rs!Active
//!         rs.MoveNext
//!     Loop
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### Write Configuration Data
//!
//! ```vb
//! Sub SaveConfig()
//!     Open "config.dat" For Output As #1
//!     Write #1, appName, version, lastRun, isRegistered
//!     Close #1
//! End Sub
//! ```
//!
//! ### Write Array Data
//!
//! ```vb
//! Sub WriteArray()
//!     Open "array.dat" For Output As #1
//!     
//!     For i = LBound(data) To UBound(data)
//!         Write #1, data(i)
//!     Next i
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### Append Data to Existing File
//!
//! ```vb
//! Sub AppendRecord()
//!     Open "log.txt" For Append As #1
//!     Write #1, Now(), userName, action, details
//!     Close #1
//! End Sub
//! ```

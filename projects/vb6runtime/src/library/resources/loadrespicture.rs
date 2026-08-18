//! # `LoadResPicture` Function
//!
//! Returns a picture object (`StdPicture`) containing an image from a resource (.res) file.
//!
//! ## Syntax
//!
//! ```vb
//! LoadResPicture(index, format)
//! ```
//!
//! ## Parameters
//!
//! - `index` (Required): Integer or String identifying the picture resource
//!   - Can be a numeric ID or string name
//!   - Must match the ID/name used when the resource was compiled
//! - `format` (Required): Integer specifying the format of the picture
//!   - `vbResBitmap` (0): Bitmap (.bmp)
//!   - `vbResIcon` (1): Icon (.ico)
//!   - `vbResCursor` (2): Cursor (.cur)
//!
//! ## Return Value
//!
//! Returns a `StdPicture` object:
//! - Picture object containing the loaded image from resources
//! - Object can be assigned to Picture properties of controls
//! - Object implements `IPicture` interface
//! - Returns Nothing if resource not found (some VB versions)
//! - Raises error 326 if resource not found
//!
//! ## Remarks
//!
//! The `LoadResPicture` function loads images from embedded resources:
//!
//! - Loads images from compiled resource (.res) files
//! - Resource file must be linked to project at compile time
//! - Supports BMP, ICO, and CUR formats only
//! - Does NOT support JPG, GIF, or PNG
//! - Returns `StdPicture` object implementing `IPicture`
//! - Alternative to `LoadPicture` for embedded images
//! - No external files needed at runtime
//! - Faster than loading from disk
//! - More secure (can't be modified by users)
//! - Resources embedded in compiled EXE/DLL
//! - Only one resource file per project
//! - Resource file added via Project > Add File
//! - Resource files created with Resource Editor or RC.EXE
//! - Index can be numeric ID or string name
//! - Format constants: vbResBitmap, vbResIcon, vbResCursor
//! - Error 326: "Resource with identifier not found" if ID/format don't match
//! - Error 48: "Error loading from file" if resource file corrupt
//! - Pictures are not cached (loaded each time)
//! - Set object = Nothing to release memory
//! - Common in `Form_Load` for initial graphics
//! - Used with Image, `PictureBox`, and Form.Picture
//! - Preferred for distribution (no external image files)
//!
//! ## Typical Uses
//!
//! 1. **Load Bitmap to `PictureBox`**
//!    ```vb
//!    Picture1.Picture = LoadResPicture(101, vbResBitmap)
//!    ```
//!
//! 2. **Load Icon to Image Control**
//!    ```vb
//!    Image1.Picture = LoadResPicture(102, vbResIcon)
//!    ```
//!
//! 3. **Load Form Background**
//!    ```vb
//!    Me.Picture = LoadResPicture("BACKGROUND", vbResBitmap)
//!    ```
//!
//! 4. **Load Cursor**
//!    ```vb
//!    Me.MousePointer = vbCustom
//!    Me.MouseIcon = LoadResPicture(103, vbResCursor)
//!    ```
//!
//! 5. **Load Named Resource**
//!    ```vb
//!    imgLogo.Picture = LoadResPicture("LOGO", vbResBitmap)
//!    ```
//!
//! 6. **Conditional Image Loading**
//!    ```vb
//!    If mode = "dark" Then
//!        Picture1.Picture = LoadResPicture(201, vbResBitmap)
//!    Else
//!        Picture1.Picture = LoadResPicture(101, vbResBitmap)
//!    End If
//!    ```
//!
//! 7. **Button Icons**
//!    ```vb
//!    cmdSave.Picture = LoadResPicture(104, vbResIcon)
//!    ```
//!
//! 8. **Multiple Images in Loop**
//!    ```vb
//!    For i = 1 To 5
//!        imgArray(i).Picture = LoadResPicture(100 + i, vbResBitmap)
//!    Next i
//!    ```
//!
//! ## Basic Examples
//!
//! ### Example 1: Basic Picture Loading
//! ```vb
//! ' Load bitmap from resources
//! Picture1.Picture = LoadResPicture(101, vbResBitmap)
//!
//! ' Load icon
//! Image1.Picture = LoadResPicture(102, vbResIcon)
//!
//! ' Load using string name
//! Picture2.Picture = LoadResPicture("SPLASH", vbResBitmap)
//! ```
//!
//! ### Example 2: Form Initialization
//! ```vb
//! Private Sub Form_Load()
//!     ' Load form background
//!     Me.Picture = LoadResPicture(101, vbResBitmap)
//!     
//!     ' Load toolbar icons
//!     cmdNew.Picture = LoadResPicture(201, vbResIcon)
//!     cmdOpen.Picture = LoadResPicture(202, vbResIcon)
//!     cmdSave.Picture = LoadResPicture(203, vbResIcon)
//! End Sub
//! ```
//!
//! ### Example 3: Error Handling
//! ```vb
//! On Error Resume Next
//! Picture1.Picture = LoadResPicture(999, vbResBitmap)
//! If Err.Number = 326 Then
//!     MsgBox "Resource not found!", vbCritical
//!     Err.Clear
//! ElseIf Err.Number <> 0 Then
//!     MsgBox "Error loading resource: " & Err.Description, vbCritical
//!     Err.Clear
//! End If
//! ```
//!
//! ### Example 4: Dynamic Loading
//! ```vb
//! Dim imageID As Integer
//! imageID = 101 + selectedIndex
//! Picture1.Picture = LoadResPicture(imageID, vbResBitmap)
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `SafeLoadResPicture`
//! ```vb
//! Function SafeLoadResPicture(ByVal resID As Variant, _
//!                             ByVal resFormat As Integer, _
//!                             ByVal ctrl As Object) As Boolean
//!     On Error Resume Next
//!     Set ctrl.Picture = LoadResPicture(resID, resFormat)
//!     SafeLoadResPicture = (Err.Number = 0)
//!     Err.Clear
//! End Function
//! ```
//!
//! ### Pattern 2: `PreloadResourcePictures`
//! ```vb
//! Dim preloadedPics() As StdPicture
//!
//! Sub PreloadResourcePictures()
//!     Dim i As Long
//!     ReDim preloadedPics(1 To 5)
//!     
//!     For i = 1 To 5
//!         Set preloadedPics(i) = LoadResPicture(100 + i, vbResBitmap)
//!     Next i
//! End Sub
//!
//! Sub ShowPreloadedImage(ByVal index As Long)
//!     If index >= 1 And index <= UBound(preloadedPics) Then
//!         Set Picture1.Picture = preloadedPics(index)
//!     End If
//! End Sub
//! ```
//!
//! ### Pattern 3: `LoadResPictureWithDefault`
//! ```vb
//! Function LoadResPictureWithDefault(ByVal resID As Variant, _
//!                                    ByVal resFormat As Integer, _
//!                                    ByVal defaultID As Variant) As StdPicture
//!     On Error Resume Next
//!     
//!     Set LoadResPictureWithDefault = LoadResPicture(resID, resFormat)
//!     If Err.Number <> 0 Then
//!         Err.Clear
//!         Set LoadResPictureWithDefault = LoadResPicture(defaultID, resFormat)
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 4: `LoadResByName`
//! ```vb
//! Function LoadResByName(ByVal resName As String, _
//!                        ByVal resFormat As Integer) As StdPicture
//!     On Error Resume Next
//!     Set LoadResByName = LoadResPicture(resName, resFormat)
//!     
//!     If Err.Number <> 0 Then
//!         Debug.Print "Failed to load resource: " & resName
//!         Set LoadResByName = Nothing
//!         Err.Clear
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 5: `ToggleResPicture`
//! ```vb
//! Dim currentState As Boolean
//!
//! Sub ToggleResPicture()
//!     If currentState Then
//!         Picture1.Picture = LoadResPicture(101, vbResBitmap)
//!     Else
//!         Picture1.Picture = LoadResPicture(102, vbResBitmap)
//!     End If
//!     currentState = Not currentState
//! End Sub
//! ```
//!
//! ### Pattern 6: `LoadThemeResources`
//! ```vb
//! Enum ThemeType
//!     tmLight = 0
//!     tmDark = 1
//! End Enum
//!
//! Sub LoadThemeResources(theme As ThemeType)
//!     Dim baseID As Integer
//!     baseID = IIf(theme = tmDark, 200, 100)
//!     
//!     Me.Picture = LoadResPicture(baseID + 1, vbResBitmap)
//!     Picture1.Picture = LoadResPicture(baseID + 2, vbResBitmap)
//!     Picture2.Picture = LoadResPicture(baseID + 3, vbResBitmap)
//! End Sub
//! ```
//!
//! ### Pattern 7: `ResExists`
//! ```vb
//! Function ResExists(ByVal resID As Variant, _
//!                    ByVal resFormat As Integer) As Boolean
//!     On Error Resume Next
//!     Dim pic As StdPicture
//!     Set pic = LoadResPicture(resID, resFormat)
//!     ResExists = (Err.Number = 0)
//!     Set pic = Nothing
//!     Err.Clear
//! End Function
//! ```
//!
//! ### Pattern 8: `LoadAllResourceIcons`
//! ```vb
//! Function LoadAllResourceIcons(startID As Integer, _
//!                               endID As Integer) As Collection
//!     Dim col As New Collection
//!     Dim i As Integer
//!     Dim pic As StdPicture
//!     
//!     On Error Resume Next
//!     For i = startID To endID
//!         Set pic = LoadResPicture(i, vbResIcon)
//!         If Err.Number = 0 Then
//!             col.Add pic
//!         End If
//!         Err.Clear
//!     Next i
//!     
//!     Set LoadAllResourceIcons = col
//! End Function
//! ```
//!
//! ### Pattern 9: `SetButtonIcon`
//! ```vb
//! Sub SetButtonIcon(btn As CommandButton, _
//!                   ByVal iconID As Integer, _
//!                   ByVal enabled As Boolean)
//!     On Error Resume Next
//!     
//!     If enabled Then
//!         Set btn.Picture = LoadResPicture(iconID, vbResIcon)
//!     Else
//!         Set btn.Picture = LoadResPicture(iconID + 100, vbResIcon)
//!     End If
//!     
//!     btn.enabled = enabled
//! End Sub
//! ```
//!
//! ### Pattern 10: `LoadResourceArray`
//! ```vb
//! Sub LoadResourceArray(ByRef picArray() As StdPicture, _
//!                       ByVal startID As Integer, _
//!                       ByVal count As Integer, _
//!                       ByVal resFormat As Integer)
//!     Dim i As Integer
//!     ReDim picArray(1 To count)
//!     
//!     On Error Resume Next
//!     For i = 1 To count
//!         Set picArray(i) = LoadResPicture(startID + i - 1, resFormat)
//!         If Err.Number <> 0 Then
//!             Debug.Print "Failed to load resource: " & (startID + i - 1)
//!             Err.Clear
//!         End If
//!     Next i
//! End Sub
//! ```
//!
//! ## Advanced Examples
//!
//! ### Example 1: Resource Picture Manager
//! ```vb
//! ' Class: ResPictureManager
//! Private m_cache As Collection
//!
//! Private Sub Class_Initialize()
//!     Set m_cache = New Collection
//! End Sub
//!
//! Public Function LoadPicture(ByVal resID As Variant, _
//!                             ByVal resFormat As Integer) As StdPicture
//!     Dim key As String
//!     On Error Resume Next
//!     
//!     key = CStr(resID) & "_" & CStr(resFormat)
//!     Set LoadPicture = m_cache(key)
//!     
//!     If Err.Number <> 0 Then
//!         Err.Clear
//!         Set LoadPicture = LoadResPicture(resID, resFormat)
//!         If Err.Number = 0 Then
//!             m_cache.Add LoadPicture, key
//!         Else
//!             Err.Raise vbObjectError + 1000, "ResPictureManager", _
//!                       "Failed to load resource"
//!         End If
//!     End If
//! End Function
//!
//! Public Sub AssignToControl(ByVal ctrl As Object, _
//!                            ByVal resID As Variant, _
//!                            ByVal resFormat As Integer)
//!     Set ctrl.Picture = LoadPicture(resID, resFormat)
//! End Sub
//!
//! Public Sub ClearCache()
//!     Dim i As Long
//!     For i = m_cache.Count To 1 Step -1
//!         m_cache.Remove i
//!     Next i
//! End Sub
//!
//! Public Property Get CacheSize() As Long
//!     CacheSize = m_cache.Count
//! End Property
//!
//! Private Sub Class_Terminate()
//!     ClearCache
//!     Set m_cache = Nothing
//! End Sub
//! ```
//!
//! ### Example 2: Image Gallery from Resources
//! ```vb
//! ' Form with Picture1, Timer1, cmdNext, cmdPrev, lblInfo
//! Private Const BASE_IMAGE_ID = 1001
//! Private Const IMAGE_COUNT = 10
//! Private currentIndex As Long
//!
//! Private Sub Form_Load()
//!     currentIndex = 0
//!     ShowCurrentImage
//!     Timer1.Interval = 5000 ' 5 seconds
//!     Timer1.Enabled = True
//! End Sub
//!
//! Private Sub ShowCurrentImage()
//!     Dim imageID As Integer
//!     On Error Resume Next
//!     
//!     imageID = BASE_IMAGE_ID + currentIndex
//!     Set Picture1.Picture = LoadResPicture(imageID, vbResBitmap)
//!     
//!     If Err.Number <> 0 Then
//!         Picture1.Cls
//!         Picture1.Print "Image not found"
//!     Else
//!         lblInfo.Caption = "Image " & (currentIndex + 1) & " of " & IMAGE_COUNT
//!     End If
//!     Err.Clear
//! End Sub
//!
//! Private Sub Timer1_Timer()
//!     NextImage
//! End Sub
//!
//! Private Sub cmdNext_Click()
//!     NextImage
//! End Sub
//!
//! Private Sub cmdPrev_Click()
//!     PrevImage
//! End Sub
//!
//! Private Sub NextImage()
//!     currentIndex = (currentIndex + 1) Mod IMAGE_COUNT
//!     ShowCurrentImage
//! End Sub
//!
//! Private Sub PrevImage()
//!     currentIndex = (currentIndex - 1 + IMAGE_COUNT) Mod IMAGE_COUNT
//!     ShowCurrentImage
//! End Sub
//! ```
//!
//! ### Example 3: Toolbar with Resource Icons
//! ```vb
//! ' Form with toolbar buttons array: cmdTool(0 to 9)
//! Private Type ToolButton
//!     caption As String
//!     iconID As Integer
//!     enabled As Boolean
//!     tooltip As String
//! End Type
//!
//! Private toolConfig() As ToolButton
//!
//! Private Sub Form_Load()
//!     InitializeToolbar
//!     ApplyToolbarConfig
//! End Sub
//!
//! Private Sub InitializeToolbar()
//!     ReDim toolConfig(0 To 9)
//!     
//!     With toolConfig(0)
//!         .caption = "New"
//!         .iconID = 201
//!         .enabled = True
//!         .tooltip = "Create new document"
//!     End With
//!     
//!     With toolConfig(1)
//!         .caption = "Open"
//!         .iconID = 202
//!         .enabled = True
//!         .tooltip = "Open existing document"
//!     End With
//!     
//!     With toolConfig(2)
//!         .caption = "Save"
//!         .iconID = 203
//!         .enabled = False
//!         .tooltip = "Save current document"
//!     End With
//!     
//!     ' ... more buttons
//! End Sub
//!
//! Private Sub ApplyToolbarConfig()
//!     Dim i As Long
//!     On Error Resume Next
//!     
//!     For i = 0 To UBound(toolConfig)
//!         With cmdTool(i)
//!             .caption = toolConfig(i).caption
//!             .enabled = toolConfig(i).enabled
//!             .ToolTipText = toolConfig(i).tooltip
//!             
//!             Set .Picture = LoadResPicture(toolConfig(i).iconID, vbResIcon)
//!             If Err.Number <> 0 Then
//!                 Debug.Print "Failed to load icon: " & toolConfig(i).iconID
//!                 Err.Clear
//!             End If
//!         End With
//!     Next i
//! End Sub
//!
//! Public Sub EnableTool(ByVal index As Long)
//!     If index >= 0 And index <= UBound(toolConfig) Then
//!         toolConfig(index).enabled = True
//!         cmdTool(index).enabled = True
//!     End If
//! End Sub
//!
//! Public Sub DisableTool(ByVal index As Long)
//!     If index >= 0 And index <= UBound(toolConfig) Then
//!         toolConfig(index).enabled = False
//!         cmdTool(index).enabled = False
//!     End If
//! End Sub
//! ```
//!
//! ### Example 4: Multi-State Indicator
//! ```vb
//! ' Form with imgStatus (Image control)
//! Public Enum StatusState
//!     stIdle = 0
//!     stProcessing = 1
//!     stSuccess = 2
//!     stWarning = 3
//!     stError = 4
//! End Enum
//!
//! Private Const RES_STATUS_IDLE = 301
//! Private Const RES_STATUS_PROCESSING = 302
//! Private Const RES_STATUS_SUCCESS = 303
//! Private Const RES_STATUS_WARNING = 304
//! Private Const RES_STATUS_ERROR = 305
//!
//! Private statusIcons() As StdPicture
//! Private currentStatus As StatusState
//!
//! Private Sub Form_Load()
//!     LoadStatusIcons
//!     SetStatus stIdle
//! End Sub
//!
//! Private Sub LoadStatusIcons()
//!     ReDim statusIcons(0 To 4)
//!     
//!     On Error Resume Next
//!     Set statusIcons(stIdle) = LoadResPicture(RES_STATUS_IDLE, vbResIcon)
//!     Set statusIcons(stProcessing) = LoadResPicture(RES_STATUS_PROCESSING, vbResIcon)
//!     Set statusIcons(stSuccess) = LoadResPicture(RES_STATUS_SUCCESS, vbResIcon)
//!     Set statusIcons(stWarning) = LoadResPicture(RES_STATUS_WARNING, vbResIcon)
//!     Set statusIcons(stError) = LoadResPicture(RES_STATUS_ERROR, vbResIcon)
//!     
//!     If Err.Number <> 0 Then
//!         MsgBox "Warning: Some status icons could not be loaded", vbExclamation
//!         Err.Clear
//!     End If
//! End Sub
//!
//! Public Sub SetStatus(ByVal newStatus As StatusState)
//!     currentStatus = newStatus
//!     
//!     If newStatus >= 0 And newStatus <= UBound(statusIcons) Then
//!         If Not statusIcons(newStatus) Is Nothing Then
//!             Set imgStatus.Picture = statusIcons(newStatus)
//!         End If
//!     End If
//! End Sub
//!
//! Public Function GetStatus() As StatusState
//!     GetStatus = currentStatus
//! End Function
//!
//! Private Sub Form_Unload(Cancel As Integer)
//!     Dim i As Long
//!     For i = 0 To UBound(statusIcons)
//!         Set statusIcons(i) = Nothing
//!     Next i
//! End Sub
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! ' Error 326: Resource with identifier not found
//! On Error Resume Next
//! Set pic = LoadResPicture(999, vbResBitmap)
//! If Err.Number = 326 Then
//!     MsgBox "Resource not found!"
//! End If
//!
//! ' Error 48: Error loading from file
//! Set pic = LoadResPicture(101, vbResBitmap)
//! If Err.Number = 48 Then
//!     MsgBox "Resource file is corrupt or missing!"
//! End If
//!
//! ' Safe loading pattern
//! Function TryLoadResPicture(ByVal resID As Variant, _
//!                            ByVal resFormat As Integer, _
//!                            ByRef pic As StdPicture) As Boolean
//!     On Error Resume Next
//!     Set pic = LoadResPicture(resID, resFormat)
//!     TryLoadResPicture = (Err.Number = 0)
//!     Err.Clear
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - **Fast Loading**: Resources embedded in EXE (very fast access)
//! - **No File I/O**: No disk access required
//! - **Memory Usage**: Pictures consume memory until released
//! - **No Caching**: Each call loads fresh copy (implement caching if needed)
//! - **Preloading**: Load frequently used images once at startup
//! - **EXE Size**: Large images increase executable size
//!
//! ## Best Practices
//!
//! 1. **Always handle errors** - resource might not exist
//! 2. **Use constants** for resource IDs for maintainability
//! 3. **Preload frequently used images** for better performance
//! 4. **Release memory** by setting picture objects to Nothing when done
//! 5. **Use meaningful names** for string-based resource IDs
//! 6. **Test all resources** during development
//! 7. **Document resource IDs** in code or separate file
//! 8. **Use Resource Editor** to manage resources efficiently
//! 9. **Consider image size** - large bitmaps increase EXE size
//! 10. **Cache in Collection** for images used multiple times
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Source | External Files |
//! |----------|---------|--------|----------------|
//! | **`LoadResPicture`** | Load from resources | Embedded .res | No |
//! | **`LoadPicture`** | Load from file | External file | Yes |
//! | **`LoadResData`** | Load binary data | Embedded .res | No |
//! | **`LoadResString`** | Load string | Embedded .res | No |
//!
//! ## `LoadResPicture` vs `LoadPicture`
//!
//! ```vb
//! ' LoadResPicture - from embedded resources
//! Picture1.Picture = LoadResPicture(101, vbResBitmap)
//!
//! ' LoadPicture - from external file
//! Picture1.Picture = LoadPicture("C:\Images\logo.bmp")
//! ```
//!
//! **When to use each:**
//! - **`LoadResPicture`**: Distribution (no external files), static images, faster loading
//! - **`LoadPicture`**: Dynamic images, user-selected files, easier updates
//!
//! ## Resource Format Constants
//!
//! ```vb
//! ' Format constants
//! Const vbResBitmap = 0  ' Bitmap (.bmp)
//! Const vbResIcon = 1    ' Icon (.ico)
//! Const vbResCursor = 2  ' Cursor (.cur)
//!
//! ' Usage
//! Picture1.Picture = LoadResPicture(101, vbResBitmap)
//! Image1.Picture = LoadResPicture(102, vbResIcon)
//! Me.MouseIcon = LoadResPicture(103, vbResCursor)
//! ```
//!
//! ## Platform Notes
//!
//! - Available in VB6 (not in early VB versions)
//! - Requires resource file (.res) linked to project
//! - Resource file created with Resource Editor or RC.EXE
//! - Only one resource file per project
//! - Resources embedded in compiled EXE/DLL
//! - Returns `StdPicture` object (OLE automation object)
//! - Supports BMP, ICO, CUR formats only
//! - No native support for JPG, GIF, PNG
//! - Format parameter: 0=Bitmap, 1=Icon, 2=Cursor
//! - Icons can contain multiple sizes
//!
//! ## Limitations
//!
//! - **Format Support**: Only BMP, ICO, CUR (no JPG/GIF/PNG)
//! - **One Resource File**: Only one .res file per project
//! - **Compile Time**: Must recompile to update resources
//! - **No Modification**: Cannot modify resources at runtime
//! - **No Caching**: Each call reloads from resource
//! - **EXE Size**: Large images significantly increase EXE size
//! - **No Compression**: Images stored uncompressed
//! - **Limited Editor**: VB6 Resource Editor is basic
//! - **No Metadata**: Cannot read image dimensions before loading
//! - **Memory Usage**: Large images consume significant memory
//!
//! ## Related Functions
//!
//! - `LoadPicture`: Load picture from external file
//! - `LoadResData`: Load custom binary data from resources
//! - `LoadResString`: Load string from resources
//! - `SavePicture`: Save picture object to file
//! - `Set`: Assign object references

use super::resfile::{rt, ResEntry, ResFile, ResId, VB_RES_BITMAP, VB_RES_CURSOR, VB_RES_ICON};
use super::{index_to_res_id, resource_not_found};
use crate::error::VBResult;
use crate::state::resources;
use crate::value::VBVariant;
use crate::StdPicture;

/// Byte offset of `biWidth` within a `BITMAPINFOHEADER`.
const BI_WIDTH_OFFSET: usize = 4;
/// Byte offset of `biHeight` within a `BITMAPINFOHEADER`.
const BI_HEIGHT_OFFSET: usize = 8;
/// Byte length of the `BITMAPINFOHEADER` that opens `RT_BITMAP` and `RT_ICON`
/// data. Shorter headers (`BITMAPCOREHEADER`) are not produced by the VB6
/// Resource Editor and are not supported.
const BITMAPINFOHEADER_LEN: usize = 40;

/// Byte offset of the entry count within a `GRPICONDIR`/`GRPCURSORDIR`.
const GROUP_COUNT_OFFSET: usize = 4;
/// Byte length of a `GRPICONDIR`/`GRPCURSORDIR` header, after which the first
/// `GRPICONDIRENTRY` begins.
const GROUP_HEADER_LEN: usize = 6;
/// A `GRPICONDIRENTRY` width or height byte of 0 means 256 pixels, since the
/// field cannot hold 256 itself.
const GROUP_DIMENSION_256: i32 = 256;

/// Implementation of the `LoadResPicture` function.
///
/// VB6 behavior:
/// - Reads a bitmap, icon, or cursor from the project's linked `.res` file
/// - `index` is a numeric resource ID or a string resource name
/// - `format` is `vbResBitmap` (0), `vbResIcon` (1), or `vbResCursor` (2)
/// - Returns a `StdPicture` object carrying the image's real dimensions
/// - Raises error 326 if the picture, or the resource file, is not found
///
/// # Icons and cursors are indirect
///
/// A `.res` file does not store an icon as one resource. The ID a program
/// passes names an `RT_GROUP_ICON` *directory*, which lists the `RT_ICON`
/// images that make up the icon (16x16, 32x32, and so on). Cursors work the
/// same way through `RT_GROUP_CURSOR`. This resolves the group and reports the
/// first listed image's dimensions, which is the one VB6 draws by default.
pub fn loadrespicture(index: &VBVariant, format: &VBVariant) -> VBResult<VBVariant> {
    let res_id = index_to_res_id(index)?;
    let format = format.as_i32().map_err(|_| resource_not_found())?;

    let (width, height) = resources::with_file(|res| match format {
        VB_RES_BITMAP => bitmap_size(res, &res_id),
        VB_RES_ICON => group_size(res, &res_id, rt::GROUP_ICON, rt::ICON),
        VB_RES_CURSOR => group_size(res, &res_id, rt::GROUP_CURSOR, rt::CURSOR),
        // VB6 only defines the three constants above.
        _ => Err(resource_not_found()),
    })?;

    Ok(VBVariant::from_object(Box::new(StdPicture::new(
        width, height,
    ))))
}

/// Dimensions of an `RT_BITMAP` resource, from its `BITMAPINFOHEADER`.
fn bitmap_size(res: &ResFile, res_id: &ResId) -> VBResult<(i32, i32)> {
    let entry = find(res, rt::BITMAP, res_id)?;
    let data = res.data(entry);
    let (width, height) = bitmapinfoheader_size(data)?;
    // A top-down bitmap has a negative biHeight; the picture's height is the
    // magnitude either way.
    Ok((width, height.abs()))
}

/// Dimensions of the first image listed in an icon or cursor group.
///
/// Falls back to the individual `member_type` resource when no group directory
/// names the requested ID, which is how a `.res` file holding a lone image
/// without a group is laid out.
fn group_size(
    res: &ResFile,
    res_id: &ResId,
    group_type: u16,
    member_type: u16,
) -> VBResult<(i32, i32)> {
    if let Ok(group) = find(res, group_type, res_id) {
        let data = res.data(group);

        let count = read_u16(data, GROUP_COUNT_OFFSET).ok_or_else(resource_not_found)?;
        if count == 0 {
            return Err(resource_not_found());
        }

        // The directory's own width/height bytes describe the image, so the
        // member resource does not need to be read.
        let width = *data.get(GROUP_HEADER_LEN).ok_or_else(resource_not_found)?;
        let height = *data
            .get(GROUP_HEADER_LEN + 1)
            .ok_or_else(resource_not_found)?;
        return Ok((byte_dimension(width), byte_dimension(height)));
    }

    // No group: treat the ID as naming the image directly.
    let entry = find(res, member_type, res_id)?;
    let (width, height) = bitmapinfoheader_size(res.data(entry))?;
    // An icon's BITMAPINFOHEADER covers the colour bitmap stacked on the AND
    // mask, so biHeight is twice the icon's visible height.
    Ok((width, height.abs() / 2))
}

/// Reads `biWidth` and `biHeight` from a `BITMAPINFOHEADER`.
fn bitmapinfoheader_size(data: &[u8]) -> VBResult<(i32, i32)> {
    if data.len() < BITMAPINFOHEADER_LEN {
        return Err(resource_not_found());
    }
    let width = read_i32(data, BI_WIDTH_OFFSET).ok_or_else(resource_not_found)?;
    let height = read_i32(data, BI_HEIGHT_OFFSET).ok_or_else(resource_not_found)?;
    Ok((width, height))
}

/// Interprets a `GRPICONDIRENTRY` dimension byte, where 0 encodes 256.
fn byte_dimension(value: u8) -> i32 {
    if value == 0 {
        GROUP_DIMENSION_256
    } else {
        i32::from(value)
    }
}

/// Finds the entry of type `res_type` matching `res_id`.
fn find<'a>(res: &'a ResFile, res_type: u16, res_id: &ResId) -> VBResult<&'a ResEntry> {
    match res_id {
        ResId::Ordinal(ordinal) => res.find_by_ordinal(res_type, *ordinal),
        ResId::Name(name) => res.find_by_name(res_type, name),
    }
    .ok_or_else(resource_not_found)
}

/// Reads a little-endian `u16` at `offset`, or `None` past the end.
fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    data.get(offset..offset + 2)
        .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// Reads a little-endian `i32` at `offset`, or `None` past the end.
fn read_i32(data: &[u8], offset: usize) -> Option<i32> {
    data.get(offset..offset + 4)
        .map(|bytes| i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;
    use crate::library::resources::test_support::{
        named_record, null_record, record, with_linked_res,
    };

    /// Builds a `BITMAPINFOHEADER` with the given dimensions.
    fn bitmapinfoheader(width: i32, height: i32) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(BITMAPINFOHEADER_LEN as u32).to_le_bytes());
        bytes.extend_from_slice(&width.to_le_bytes());
        bytes.extend_from_slice(&height.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
        bytes.extend_from_slice(&4u16.to_le_bytes()); // biBitCount
        bytes.resize(BITMAPINFOHEADER_LEN, 0);
        bytes
    }

    /// Builds a `GRPICONDIR` naming one member image.
    fn group_dir(width: u8, height: u8, member_id: u16) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0u16.to_le_bytes()); // reserved
        bytes.extend_from_slice(&1u16.to_le_bytes()); // type: 1 = icon
        bytes.extend_from_slice(&1u16.to_le_bytes()); // count
        bytes.push(width);
        bytes.push(height);
        bytes.push(0); // colour count
        bytes.push(0); // reserved
        bytes.extend_from_slice(&1u16.to_le_bytes()); // planes
        bytes.extend_from_slice(&4u16.to_le_bytes()); // bit count
        bytes.extend_from_slice(&744u32.to_le_bytes()); // bytes in resource
        bytes.extend_from_slice(&member_id.to_le_bytes());
        bytes
    }

    /// Dimensions of the `StdPicture` inside `value`.
    fn size_of(value: &VBVariant) -> (i32, i32) {
        let object = value.as_object().unwrap();
        let picture = object.as_any().downcast_ref::<StdPicture>().unwrap();
        (picture.width(), picture.height())
    }

    #[test]
    fn loads_a_bitmap_with_real_dimensions() {
        let mut image = null_record();
        image.extend(record(rt::BITMAP, 101, &bitmapinfoheader(120, 80)));

        with_linked_res(&image, || {
            let value = loadrespicture(
                &VBVariant::from_integer(101),
                &VBVariant::from_long(VB_RES_BITMAP),
            )
            .unwrap();
            assert_eq!(value.as_object().unwrap().type_name(), "StdPicture");
            assert_eq!(size_of(&value), (120, 80));
        });
    }

    #[test]
    fn top_down_bitmap_reports_positive_height() {
        // A negative biHeight marks a top-down DIB.
        let mut image = null_record();
        image.extend(record(rt::BITMAP, 1, &bitmapinfoheader(64, -48)));

        with_linked_res(&image, || {
            let value = loadrespicture(
                &VBVariant::from_integer(1),
                &VBVariant::from_long(VB_RES_BITMAP),
            )
            .unwrap();
            assert_eq!(size_of(&value), (64, 48));
        });
    }

    #[test]
    fn loads_a_bitmap_by_string_name() {
        let mut image = null_record();
        image.extend(named_record("2", "LOGO", &bitmapinfoheader(16, 16)));

        with_linked_res(&image, || {
            let value = loadrespicture(
                &VBVariant::from_string("LOGO"),
                &VBVariant::from_long(VB_RES_BITMAP),
            )
            .unwrap();
            assert_eq!(size_of(&value), (16, 16));
        });
    }

    #[test]
    fn loads_an_icon_through_its_group_directory() {
        let mut image = null_record();
        // The icon's BITMAPINFOHEADER height is doubled (image + AND mask).
        image.extend(record(rt::ICON, 1, &bitmapinfoheader(32, 64)));
        image.extend(record(rt::GROUP_ICON, 101, &group_dir(32, 32, 1)));

        with_linked_res(&image, || {
            let value = loadrespicture(
                &VBVariant::from_integer(101),
                &VBVariant::from_long(VB_RES_ICON),
            )
            .unwrap();
            assert_eq!(size_of(&value), (32, 32));
        });
    }

    #[test]
    fn icon_group_dimension_zero_means_256() {
        let mut image = null_record();
        image.extend(record(rt::ICON, 1, &bitmapinfoheader(256, 512)));
        image.extend(record(rt::GROUP_ICON, 101, &group_dir(0, 0, 1)));

        with_linked_res(&image, || {
            let value = loadrespicture(
                &VBVariant::from_integer(101),
                &VBVariant::from_long(VB_RES_ICON),
            )
            .unwrap();
            assert_eq!(size_of(&value), (256, 256));
        });
    }

    #[test]
    fn icon_without_a_group_halves_the_doubled_height() {
        // No RT_GROUP_ICON, so the ID names the RT_ICON directly.
        let mut image = null_record();
        image.extend(record(rt::ICON, 5, &bitmapinfoheader(48, 96)));

        with_linked_res(&image, || {
            let value = loadrespicture(
                &VBVariant::from_integer(5),
                &VBVariant::from_long(VB_RES_ICON),
            )
            .unwrap();
            assert_eq!(size_of(&value), (48, 48));
        });
    }

    #[test]
    fn loads_a_cursor_through_its_group_directory() {
        let mut image = null_record();
        image.extend(record(rt::CURSOR, 1, &bitmapinfoheader(32, 64)));
        image.extend(record(rt::GROUP_CURSOR, 201, &group_dir(32, 32, 1)));

        with_linked_res(&image, || {
            let value = loadrespicture(
                &VBVariant::from_integer(201),
                &VBVariant::from_long(VB_RES_CURSOR),
            )
            .unwrap();
            assert_eq!(size_of(&value), (32, 32));
        });
    }

    #[test]
    fn format_selects_the_resource_type() {
        // Same ID under both RT_BITMAP and RT_GROUP_ICON, different sizes.
        let mut image = null_record();
        image.extend(record(rt::BITMAP, 1, &bitmapinfoheader(10, 20)));
        image.extend(record(rt::ICON, 2, &bitmapinfoheader(32, 64)));
        image.extend(record(rt::GROUP_ICON, 1, &group_dir(32, 32, 2)));

        with_linked_res(&image, || {
            let bitmap = loadrespicture(
                &VBVariant::from_integer(1),
                &VBVariant::from_long(VB_RES_BITMAP),
            )
            .unwrap();
            assert_eq!(size_of(&bitmap), (10, 20));

            let icon = loadrespicture(
                &VBVariant::from_integer(1),
                &VBVariant::from_long(VB_RES_ICON),
            )
            .unwrap();
            assert_eq!(size_of(&icon), (32, 32));
        });
    }

    #[test]
    fn missing_picture_raises_326() {
        let mut image = null_record();
        image.extend(record(rt::BITMAP, 101, &bitmapinfoheader(8, 8)));

        with_linked_res(&image, || {
            let error = loadrespicture(
                &VBVariant::from_integer(999),
                &VBVariant::from_long(VB_RES_BITMAP),
            )
            .unwrap_err();
            assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
        });
    }

    #[test]
    fn unknown_format_raises_326() {
        let mut image = null_record();
        image.extend(record(rt::BITMAP, 1, &bitmapinfoheader(8, 8)));

        with_linked_res(&image, || {
            for format in [VBVariant::from_long(3), VBVariant::from_long(-1)] {
                let error = loadrespicture(&VBVariant::from_integer(1), &format).unwrap_err();
                assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
            }
        });
    }

    #[test]
    fn truncated_bitmap_header_raises_326_rather_than_panicking() {
        let mut image = null_record();
        image.extend(record(rt::BITMAP, 1, &[0u8; 12]));

        with_linked_res(&image, || {
            let error = loadrespicture(
                &VBVariant::from_integer(1),
                &VBVariant::from_long(VB_RES_BITMAP),
            )
            .unwrap_err();
            assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
        });
    }

    #[test]
    fn empty_icon_group_raises_326() {
        // A directory header declaring zero images.
        let mut body = Vec::new();
        body.extend_from_slice(&0u16.to_le_bytes());
        body.extend_from_slice(&1u16.to_le_bytes());
        body.extend_from_slice(&0u16.to_le_bytes()); // count = 0
        let mut image = null_record();
        image.extend(record(rt::GROUP_ICON, 101, &body));

        with_linked_res(&image, || {
            let error = loadrespicture(
                &VBVariant::from_integer(101),
                &VBVariant::from_long(VB_RES_ICON),
            )
            .unwrap_err();
            assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
        });
    }

    #[test]
    fn no_linked_resource_file_raises_326() {
        let _guard = crate::state::test_support::lock_test();
        resources::clear();
        let error = loadrespicture(
            &VBVariant::from_integer(1),
            &VBVariant::from_long(VB_RES_BITMAP),
        )
        .unwrap_err();
        assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
    }
    // ---- against a real VB6-authored .res file ----

    #[test]
    fn loads_the_icon_from_a_real_res_file() {
        // mexe2_2.res holds an RT_GROUP_ICON named "A" listing three RT_ICON
        // images; the first is 32x32 at 4bpp.
        use crate::library::resources::test_support::with_test_data_res;

        with_test_data_res("test-data/Environment/mexe2_2.res", || {
            let value = loadrespicture(
                &VBVariant::from_string("A"),
                &VBVariant::from_long(VB_RES_ICON),
            )
            .unwrap();
            assert_eq!(size_of(&value), (32, 32));
        });
    }

    #[test]
    fn loads_individual_icon_images_from_a_real_res_file() {
        // The three RT_ICON members are 32x32, 32x32, and 16x16. Addressed
        // directly (no group names them), the doubled BITMAPINFOHEADER height
        // must be halved back to the visible size.
        use crate::library::resources::test_support::with_test_data_res;

        with_test_data_res("test-data/Environment/mexe2_2.res", || {
            let format = VBVariant::from_long(VB_RES_ICON);
            assert_eq!(
                size_of(&loadrespicture(&VBVariant::from_integer(1), &format).unwrap()),
                (32, 32)
            );
            assert_eq!(
                size_of(&loadrespicture(&VBVariant::from_integer(3), &format).unwrap()),
                (16, 16)
            );
        });
    }
}

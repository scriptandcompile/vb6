export const examples = {
    "hello-world": {
        name: "Hello World",
        code: `Attribute VB_Name = "HelloModule"

Sub Main()
    Debug.Print "Hello from VB6Interpret"
End Sub
`,
    },
    "simple-math": {
        name: "Simple Math",
        code: `Attribute VB_Name = "MathModule"

Sub Main()
    Dim total As Integer
    total = 19 + 23
    Debug.Print total
End Sub
`,
    },
    branching: {
        name: "Branching",
        code: `Attribute VB_Name = "BranchModule"

Sub Main()
    Dim score As Integer
    score = 84

    If score >= 80 Then
        Debug.Print "pass"
    Else
        Debug.Print "retry"
    End If
End Sub
`,
    },
    looping: {
        name: "Looping",
        code: `Attribute VB_Name = "LoopModule"

Sub Main()
    Dim i As Integer

    For i = 1 To 3
        Debug.Print i
    Next i
End Sub
`,
    },
    files: {
        name: "Files",
        code: `Attribute VB_NAME = "FilesModule"

Sub Main()
    Dim i as Integer

    Open "Test.txt" For Append As #1 
        For i = 1 to 3
            Print #1, i
        Next i
    Close #1

End Sub
`,
    }
};

export function getExample(id) {
    return examples[id] ?? null;
}

export function getDefaultExample() {
    return examples["hello-world"];
}
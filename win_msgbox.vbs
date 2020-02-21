'Message boxes cannot be created while panicking, so we run this vbscript instead,
'with the error message given through stdin.
MsgBox WScript.StdIn.ReadLine, vbCritical+vbApplicationModal, "Error"

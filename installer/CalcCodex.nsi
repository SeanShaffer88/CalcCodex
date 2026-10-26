; CalcCodex installer — per-user, no admin needed.
Unicode true
Name "CalcCodex"
OutFile "CalcCodex-Setup-0.1.0.exe"
InstallDir "$LOCALAPPDATA\Programs\CalcCodex"
RequestExecutionLevel user
SetCompressor /SOLID lzma

Page directory
Page instfiles

Section "Install"
  SetOutPath "$INSTDIR"
  File "..\dist\CalcCodex.exe"
  File "..\LICENSE"
  File "..\README.md"
  CreateDirectory "$SMPROGRAMS\CalcCodex"
  CreateShortcut "$SMPROGRAMS\CalcCodex\CalcCodex.lnk" "$INSTDIR\CalcCodex.exe"
  CreateShortcut "$DESKTOP\CalcCodex.lnk" "$INSTDIR\CalcCodex.exe"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcCodex" "DisplayName" "CalcCodex"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcCodex" "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcCodex" "DisplayVersion" "0.1.0"
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\CalcCodex.exe"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\CalcCodex\CalcCodex.lnk"
  RMDir "$SMPROGRAMS\CalcCodex"
  Delete "$DESKTOP\CalcCodex.lnk"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcCodex"
SectionEnd

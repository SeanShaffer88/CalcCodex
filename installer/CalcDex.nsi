; CalcDex installer — per-user, no admin needed.
Unicode true
Name "CalcDex"
OutFile "CalcDex-Setup-0.8.0.exe"
InstallDir "$LOCALAPPDATA\Programs\CalcDex"
RequestExecutionLevel user
Icon "..\assets\icon.ico"
UninstallIcon "..\assets\icon.ico"
SetCompressor /SOLID lzma

Page directory
Page instfiles

Section "Install"
  SetOutPath "$INSTDIR"
  File "..\dist\CalcDex.exe"
  File "..\LICENSE"
  File "..\README.md"
  CreateDirectory "$SMPROGRAMS\CalcDex"
  CreateShortcut "$SMPROGRAMS\CalcDex\CalcDex.lnk" "$INSTDIR\CalcDex.exe"
  CreateShortcut "$DESKTOP\CalcDex.lnk" "$INSTDIR\CalcDex.exe"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcDex" "DisplayName" "CalcDex"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcDex" "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcDex" "DisplayVersion" "0.8.0"
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\CalcDex.exe"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\CalcDex\CalcDex.lnk"
  RMDir "$SMPROGRAMS\CalcDex"
  Delete "$DESKTOP\CalcDex.lnk"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CalcDex"
SectionEnd

[Setup]
AppName=Mhserver Client
AppId=opBnr6diBkq-mhserver-client-bb72-0f6e58bbcb42
AppVersion=1.0.0
AppPublisher=Bragin Anton
AppPublisherURL=https://github.com/braginantonev
AppUpdatesURL=https://github.com/braginantonev/mhserver-client
DefaultDirName={autopf}\Mhserver Client
DefaultGroupName=Mhserver Client
Compression=lzma2
SolidCompression=yes
; "ArchitecturesAllowed=x64compatible" specifies that Setup cannot run
; on anything but x64 and Windows 11 on Arm.
ArchitecturesAllowed=x64compatible
; "ArchitecturesInstallIn64BitMode=x64compatible" requests that the
; install be done in "64-bit mode" on x64 or Windows 11 on Arm,
; meaning it should use the native 64-bit Program Files directory and
; the 64-bit view of the registry.
ArchitecturesInstallIn64BitMode=x64compatible
AllowNoIcons=yes
PrivilegesRequired=lowest

WizardStyle=modern dynamic

ShowLanguageDialog=yes
LanguageDetectionMethod=uilanguage

DisableDirPage=auto
DisableProgramGroupPage=auto
DisableReadyPage=yes

OutputBaseFilename=Mhserver Client

[Languages]
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"

[Files]
Source: "bin\mhserver-client.exe"; DestDir: "{app}"; DestName: "mhserver-client.exe"
Source: "update_windows.bat"; DestDir: "{app}"; DestName: "update.bat"

;[Icons]
;Name: "{group}\My Program"; Filename: "{app}\MyProg.exe"

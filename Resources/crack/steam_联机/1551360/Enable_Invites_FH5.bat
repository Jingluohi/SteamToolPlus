set protocol_path="\"%~dp0ForzaProtocolSelector.exe\" \"%%1\""
reg add HKCU\Software\Classes\ForzaHorizon5ProtocolSelect /f /t REG_SZ /d "URL:ForzaHorizon5ProtocolSelect"
reg add HKCU\Software\Classes\ForzaHorizon5ProtocolSelect /f /v "URL Protocol" /t REG_SZ
reg add HKCU\Software\Classes\ForzaHorizon5ProtocolSelect\shell\open\command /f /t REG_SZ /d %protocol_path%
reg add HKLM\SOFTWARE\Microsoft\XboxLive\2030093255 /f /v "AcceptProtocol" /t REG_SZ /d "ForzaHorizon5ProtocolSelect://?1;{0};{1};{2}"
reg add HKLM\SOFTWARE\Microsoft\XboxLive\2030093255 /f /v "JoinProtocol" /t REG_SZ /d "ForzaHorizon5ProtocolSelect://?2;{0};{1};{2}"
pause
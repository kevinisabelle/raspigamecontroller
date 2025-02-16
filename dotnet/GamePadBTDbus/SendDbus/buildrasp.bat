dotnet publish -c Release -r linux-arm64 --self-contained true -p:PublishSingleFile=true -p:PublishTrimmed=true -o ./publish/linux-arm64

REM Copy the publish folder to the Raspberry Pi (/home/kevin/gampad/dotnet) 192.168.1.126 user : kevin (I'm on windows) (using sftp)
REM sftp kevin@192.168.1.126:/home/kevin/gampad/dotnet <<EOF

REM echo put -r "./publish/linux-arm64" "/home/kevin/gampad/dotnet" > sftp_cmd.txt && sftp -b sftp_cmd.txt kevin@192.168.1.126 && del sftp_cmd.txt



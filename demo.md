
# Windows Notes

## Table of Contents

- Path to add apps to start
- Stops Superfetch
- Opens program uninstallers (from run)
- Get product key
- Windows Debloater Script
- Windows ls
- Command to Kill Cortana
- Type info run utility prompt to view all services
- Create Symlink
- Disk Partition Utility
- Forces a partition to be deleted
- Scan and repair corruptions in system files
- Get Execution policy for powershell
- Set Execution policy for powershell to unrestricted
- View event History
- Connect WSL to xlaunch display

### Path to add apps to start

`%AppData%\\Microsoft\\Windows\\Start Menu\\Programs`

### Stops Superfetch

`net.exe stop superfetch`

### Opens program uninstallers (from run)

`Appwiz.cpl`

### Get product key

`wmic path softwareLicensingService get OA3xOriginalProductKey`

### Windows Debloater Script

`iwr -useb https://git.io/debloat|iex`

### Windows ls

`dir`

### Command to Kill Cortana

`Get-AppxPackage -allusers Microsoft.549981C3F5F10 | Remove-AppxPackage`

### Type into run utility prompt to view all services

`services.msc`

### Create Symlink

`mklink /J \"%USERPROFILE%\\Apple\\MobileSync\\Backup\" \"H:\\Users\\dalit\\iTunes\\backup\"`

### Disk Partition Utility

`diskpart`

### Forces a partition to be deleted

`DELETE PARTITION OVERRIDE`

### Scan and repair corruptions in system files

`sfc /scannow`

### Get Exececution policy for powershell

`Get-ExecutionPolicy`

### Set Exececution policy for powershell to unrestricted

`Set-ExecutionPolicy unrestricted`

### View event History

`eventvwr`

### Connect WSL to xlaunch display

`export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2; exit;}'):0`

[Go back to the homepage](#)

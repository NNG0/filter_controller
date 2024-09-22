# filter_controller

Simple controller for a pool filter. The programm is run on a raspberry pi

## setup

add to path by moving binary
```
sudo cp target/release/filter_controller /usr/local/bin/
```
### automatically start on boot 

create systemd
```
sudo nano /etc/systemd/system/filter_controller.service
```
edit service file (set User)
```
[Unit]
Description=Filter Controller Service
After=network.target

[Service]
ExecStart=/usr/local/bin/filter_controller
Restart=always
User=EDIT_NAME
StandardOutput=journal
StandardError=journal
SyslogIdentifier=filter_controller

[Install]
WantedBy=multi-user.target
```
save with crtl+o

reload systemd
```
sudo systemctl daemon-reload
```
enable start on boot
```
sudo systemctl enable filter_controller.service
```

start service
```
sudo systemctl start filter_controller.service
```

check status
```
sudo systemctl status filter_controller.service
```
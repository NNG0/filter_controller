# filter_controller

Simple controller for a pool filter. The programm is run on a raspberry pi

## setup
create binary
```bash
cargo build --release
```

add to path by moving binary
```bash
sudo cp target/release/filter_controller /usr/local/bin/
```
### automatically start on boot 

create systemd
```bash
sudo vim /etc/systemd/system/filter_controller.service
```
edit service file (set User)
```
[Unit]
Description=Filter Controller Service

[Service]
ExecStart=/usr/local/bin/filter_controller
Restart=always
SyslogIdentifier=filter_controller

[Install]
WantedBy=multi-user.target
```

reload systemd
```bash
sudo systemctl daemon-reload
```
enable start on boot
```bash
sudo systemctl enable filter_controller.service
```

start service
```bash
sudo systemctl start filter_controller.service
```

check status
```bash
sudo systemctl status filter_controller.service
```

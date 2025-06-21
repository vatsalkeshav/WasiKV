# steps followed to use crun as k3s runtime instead of runc

```sh
# install crun
sudo apt update
sudo apt install -y crun

# containerd config file for K3s
sudo mkdir -p /etc/rancher/k3s/
sudo tee /etc/rancher/k3s/containerd-config.toml > /dev/null <<EOF
version = 2
[plugins."io.containerd.grpc.v1.cri".containerd]
  default_runtime_name = "crun"
  [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.crun]
    runtime_type = "io.containerd.runc.v2"
    [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.crun.options]
      BinaryName = "crun"
EOF

# Install K3s
curl -sfL https://get.k3s.io | sh -s -

# add crun config
sudo mkdir -p /etc/rancher/k3s/

sudo tee /etc/rancher/k3s/containerd-config.toml > /dev/null <<EOF
version = 2
[plugins."io.containerd.grpc.v1.cri".containerd]
  default_runtime_name = "crun"
  [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.crun]
    runtime_type = "io.containerd.runc.v2"
    [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.crun.options]
      BinaryName = "crun"
EOF

sudo systemctl restart k3s

# try verifying use of crun instead of runc - fail :(
sudo ctr version
# Client:
#   Version:  1.7.27
#   Revision:
#   Go version: go1.22.2

# Server:
#   Version:  1.7.27
#   Revision:
#   UUID: eea5551d-83ae-4665-92db-f7ced6f649a6
# dev@ubuntu:~$ sudo ctr containers ls
# CONTAINER    IMAGE    RUNTIME
# dev@ubuntu:~$ sudo kubectl run test --image=nginx
# pod/test created
# dev@ubuntu:~$ CONTAINER_ID=$(sudo crictl ps -q --name test)
# sudo crictl inspect $CONTAINER_ID | grep runtimeType
#       "runtimeType": "io.containerd.runc.v2",
#       "runtimeType": "io.containerd.runc.v2",
#       "runtimeType": "io.containerd.runc.v2",
#       "runtimeType": "io.containerd.runc.v2",
#       "runtimeType": "io.containerd.runc.v2",
```



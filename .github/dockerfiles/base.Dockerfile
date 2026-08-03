FROM public.ecr.aws/lts/ubuntu:24.04_stable

USER root

RUN apt-get update \
  && DEBIAN_FRONTEND="noninteractive" apt-get dist-upgrade --yes \
  && DEBIAN_FRONTEND="noninteractive" apt-get install --yes ca-certificates curl gnupg lsb-release \
  && mkdir -p /etc/apt/keyrings \
  && curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg \
  && chmod a+r /etc/apt/keyrings/docker.gpg \
  && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null \
  && DEBIAN_FRONTEND="noninteractive" apt-get update \
  && DEBIAN_FRONTEND="noninteractive" apt-get install --yes \
       bash \
       build-essential \
       containerd.io \
       docker-ce \
       docker-ce-cli \
       docker-compose-plugin \
       dos2unix \
       exiftool \
       libx11-6 \
       locales \
       m4 \
       pkg-config \
       python3 \
       python3-pip \
       python3-venv \
       software-properties-common \
       sudo \
       unzip \
       wget \
       zip \
  # Install latest Git using their official PPA
  && add-apt-repository ppa:git-core/ppa \
  && DEBIAN_FRONTEND="noninteractive" apt-get install --yes git \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

# Enables Docker starting with systemd
RUN systemctl enable docker

# Make typing unicode characters in the terminal work.
ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8

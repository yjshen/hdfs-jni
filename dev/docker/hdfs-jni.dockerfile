# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

# Turn .dockerignore to .dockerallow by excluding everything and explicitly
# allowing specific files and directories. This enables us to quickly add
# dependency files to the docker content without scanning the whole directory.
# This setup requires to all of our docker containers have arrow's source
# as a mounted directory.

#ARG RELEASE_FLAG=--release
FROM hdfs-base:0.1.0 AS base
WORKDIR /tmp/hdfs-jni
RUN apt-get -y install cmake
RUN cargo install cargo-chef --version 0.1.23

FROM base as planner
RUN mkdir /tmp/hdfs-jni/src
ADD Cargo.toml .
COPY src ./src/
RUN cargo chef prepare --recipe-path recipe.json

FROM base as cacher
COPY --from=planner /tmp/hdfs-jni/recipe.json recipe.json
RUN cargo chef cook $RELEASE_FLAG --recipe-path recipe.json

FROM base as builder
RUN mkdir /tmp/hdfs-jni/src
ADD Cargo.toml .
ADD build.rs .
COPY src ./src/
COPY --from=cacher /tmp/hdfs-jni/target target

#ARG RELEASE_FLAG=--release

RUN bash -l -c 'echo export LD_LIBRARY_PATH="/usr/local/hadoop/lib/native:/usr/lib/jvm/adoptopenjdk-8-hotspot-amd64/jre/lib/amd64/server" >> /etc/bash.bashrc'
RUN bash -l -c 'echo export LIBRARY_PATH="/usr/local/hadoop/lib/native:/usr/lib/jvm/adoptopenjdk-8-hotspot-amd64/jre/lib/amd64/server" >> /etc/bash.bashrc'
RUN bash -l -c 'echo export CLASSPATH="$($HADOOP_HOME/bin/hadoop classpath --glob)" >> /etc/bash.bashrc'

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

# force build.rs to run to generate configure_me code.
ENV FORCE_REBUILD='true'
RUN RUSTFLAGS='-L /usr/local/hadoop/lib/native -L /tmp/hdfs-jni/target/debug/deps -L /usr/lib/jvm/adoptopenjdk-8-hotspot-amd64/jre/lib/amd64/server/libjvm.so' cargo test -vv
#RUN cargo test --verbose

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

FROM rust-base:0.1.0 AS base

####################
# JAVA
####################

RUN DEBIAN_FRONTEND=noninteractive apt-get install -y apt-transport-https ca-certificates wget dirmngr gnupg software-properties-common && \
    wget -qO - https://adoptopenjdk.jfrog.io/adoptopenjdk/api/gpg/key/public | apt-key add - && \
    add-apt-repository --yes https://adoptopenjdk.jfrog.io/adoptopenjdk/deb/ && \
    apt-get update && apt-get install -y adoptopenjdk-8-hotspot

ENV JAVA_HOME		/usr/lib/jvm/adoptopenjdk-8-hotspot-amd64

####################
# HADOOP
####################

ENV HADOOP_VERSION	2.6.0
ENV HADOOP_HOME		/usr/local/hadoop
ENV HADOOP_OPTS		-Djava.library.path=/usr/local/hadoop/lib/native
ENV PATH		$PATH:$HADOOP_HOME/bin:$HADOOP_HOME/sbin

RUN wget http://archive.apache.org/dist/hadoop/core/hadoop-$HADOOP_VERSION/hadoop-$HADOOP_VERSION.tar.gz && \
   tar -zxf /hadoop-$HADOOP_VERSION.tar.gz && \
   rm /hadoop-$HADOOP_VERSION.tar.gz && \
   mv hadoop-$HADOOP_VERSION /usr/local/hadoop && \
   mkdir -p /usr/local/hadoop/logs

# Overwrite default HADOOP configuration files with our config files
# COPY conf  $HADOOP_HOME/etc/hadoop/

# Formatting HDFS
# RUN mkdir -p /data/dfs/data /data/dfs/name /data/dfs/namesecondary && \
#     hdfs namenode -format
# VOLUME /data


####################
# PORTS
####################
#
# http://docs.hortonworks.com/HDPDocuments/HDP2/HDP-2.3.0/bk_HDP_Reference_Guide/content/reference_chap2.html
# http://www.cloudera.com/content/cloudera/en/documentation/core/latest/topics/cdh_ig_ports_cdh5.html
# http://hadoop.apache.org/docs/current/hadoop-project-dist/hadoop-common/core-default.xml
# http://hadoop.apache.org/docs/current/hadoop-project-dist/hadoop-hdfs/hdfs-default.xml

# HDFS: NameNode (NN):
#	 8020 = fs.defaultFS			(IPC / File system metadata operations)
#						(9000 is also frequently used alternatively)
#	 8022 = dfs.namenode.servicerpc-address	(optional port used by HDFS daemons to avoid sharing RPC port)
#       50070 = dfs.namenode.http-address	(HTTP  / NN Web UI)
#	50470 = dfs.namenode.https-address	(HTTPS / Secure UI)
# HDFS: DataNode (DN):
#	50010 = dfs.datanode.address		(Data transfer)
#	50020 = dfs.datanode.ipc.address	(IPC / metadata operations)
#	50075 = dfs.datanode.http.address	(HTTP  / DN Web UI)
#	50475 = dfs.datanode.https.address	(HTTPS / Secure UI)
# HDFS: Secondary NameNode (SNN)
#	50090 = dfs.secondary.http.address	(HTTP / Checkpoint for NameNode metadata)
# EXPOSE 9000 50070 50010 50020 50075 50090

# CMD ["hdfs"]

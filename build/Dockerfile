FROM konstin2/maturin

RUN curl -O https://www.openssl.org/source/openssl-1.1.1l.tar.gz && \
    tar -zxvf openssl-1.1.1l.tar.gz --directory /opt && \
    mv /opt/openssl-1.1.1l /opt/openssl && \
    cd /opt/openssl && \
    ./config shared && \
    make && \
    make install

ENV PKG_CONFIG_PATH=/usr/local/lib64/pkgconfig
ENV OPENSSL_LIB_DIR=/usr/local/lib64